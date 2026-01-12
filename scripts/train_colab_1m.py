"""
LEMMA MathBERT Training - Google Colab Version
===============================================
Production training on 1M synthetic problems with 30 epochs.

Instructions:
1. Upload this file and data/synthetic_1000000.json to Colab
2. Run: !pip install transformers torch accelerate tqdm -q
3. Execute all cells
4. Download the model folder when complete

Estimated time: ~4-6 hours on T4 GPU, ~2-3 hours on A100
"""

import torch
import torch.nn as nn
from torch.utils.data import Dataset, DataLoader
from transformers import (
    BertConfig, 
    BertForSequenceClassification,
    AutoTokenizer, 
    get_cosine_schedule_with_warmup
)
from sklearn.model_selection import train_test_split
import numpy as np
import json
import os
from tqdm.auto import tqdm

# ============================================================================
# CONFIGURATION - 30 epochs, 10 layer BERT
# ============================================================================
VOCAB = [
    "x = 0", "y = 0", "x = y", "x = 1", "y = 1",
    "a = b = c = 1", "abc = 1 constraint", "Apply AM-GM", "Apply Cauchy-Schwarz",
    "Assume f is linear", "Assume f is injective", "Assume f is monotonic",
    "Check small cases", "Use modular arithmetic", "Homogenize", "WLOG assume ordering",
    "Substitute c = 1/(ab)", "y = f(x)", "x = -y", "Consider p = 2 separately",
]

CONFIG = {
    # Model architecture - 10 layer BERT
    "hidden_size": 768,
    "num_hidden_layers": 10,
    "num_attention_heads": 12,
    "intermediate_size": 3072,
    "hidden_dropout_prob": 0.1,
    "attention_probs_dropout_prob": 0.1,
    
    # Training
    "max_length": 256,
    "epochs": 30,
    "batch_size": 64,  # Larger batch for GPU
    "learning_rate": 3e-5,
    "warmup_ratio": 0.1,
    "weight_decay": 0.01,
    "max_samples": None,  # Use all 1M
    
    # Output
    "output_dir": "lemma_mathbert_1m",
    "save_every_n_epochs": 5,  # Checkpoint every 5 epochs
}

# ============================================================================
# DATASET
# ============================================================================
class SyntheticMathDataset(Dataset):
    def __init__(self, data, tokenizer, vocab, max_length=256):
        self.data = data
        self.tokenizer = tokenizer
        self.vocab_to_idx = {v: i for i, v in enumerate(vocab)}
        self.max_length = max_length
        
    def __len__(self):
        return len(self.data)
    
    def __getitem__(self, idx):
        item = self.data[idx]
        text = item["statement"]
        subs = item.get("substitutions", [])
        
        # Multi-label target
        labels = torch.zeros(len(self.vocab_to_idx))
        for sub in subs:
            if sub in self.vocab_to_idx:
                labels[self.vocab_to_idx[sub]] = 1.0
        
        encoding = self.tokenizer(
            text,
            max_length=self.max_length,
            padding="max_length",
            truncation=True,
            return_tensors="pt"
        )
        
        return {
            "input_ids": encoding["input_ids"].squeeze(),
            "attention_mask": encoding["attention_mask"].squeeze(),
            "labels": labels
        }

# ============================================================================
# TRAINING FUNCTIONS
# ============================================================================
def train_epoch(model, loader, optimizer, scheduler, device, epoch):
    model.train()
    total_loss = 0
    pbar = tqdm(loader, desc=f"Epoch {epoch}")
    
    for batch in pbar:
        optimizer.zero_grad()
        outputs = model(
            input_ids=batch["input_ids"].to(device),
            attention_mask=batch["attention_mask"].to(device),
            labels=batch["labels"].to(device)
        )
        loss = outputs.loss
        loss.backward()
        torch.nn.utils.clip_grad_norm_(model.parameters(), 1.0)
        optimizer.step()
        scheduler.step()
        total_loss += loss.item()
        pbar.set_postfix({"loss": f"{loss.item():.4f}", "lr": f"{scheduler.get_last_lr()[0]:.2e}"})
    
    return total_loss / len(loader)

def evaluate(model, loader, device):
    model.eval()
    total_loss = 0
    all_preds = []
    all_labels = []
    
    with torch.no_grad():
        for batch in tqdm(loader, desc="Evaluating"):
            outputs = model(
                input_ids=batch["input_ids"].to(device),
                attention_mask=batch["attention_mask"].to(device),
                labels=batch["labels"].to(device)
            )
            total_loss += outputs.loss.item()
            
            preds = (torch.sigmoid(outputs.logits) > 0.5).float()
            all_preds.append(preds.cpu())
            all_labels.append(batch["labels"])
    
    all_preds = torch.cat(all_preds)
    all_labels = torch.cat(all_labels)
    
    # Metrics
    exact_match = (all_preds == all_labels).all(dim=1).float().mean().item()
    per_label_acc = (all_preds == all_labels).float().mean(dim=0)
    
    return {
        "loss": total_loss / len(loader),
        "exact_match": exact_match,
        "per_label_acc": per_label_acc.mean().item()
    }

# ============================================================================
# MAIN
# ============================================================================
def main():
    print("=" * 70)
    print("  LEMMA MathBERT Training - 1M Synthetic Problems")
    print("  30 Epochs | 10-Layer BERT | Production Quality")
    print("=" * 70)
    
    # Device
    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
    print(f"\n🖥️  Device: {device}")
    if device.type == "cuda":
        print(f"   GPU: {torch.cuda.get_device_name(0)}")
        print(f"   Memory: {torch.cuda.get_device_properties(0).total_memory / 1e9:.1f} GB")
    
    # Load data
    print("\n📂 Loading data...")
    data_file = "data/synthetic_1000000.json"
    if not os.path.exists(data_file):
        data_file = "data/synthetic_1000000.json"  # Local path
    
    with open(data_file, "r", encoding="utf-8") as f:
        data = json.load(f)
    print(f"   Loaded {len(data):,} problems")
    
    if CONFIG["max_samples"]:
        import random
        random.seed(42)
        data = random.sample(data, CONFIG["max_samples"])
        print(f"   Subsampled to {len(data):,}")
    
    # Create 10-layer BERT model
    print("\n🧠 Creating 10-layer BERT model...")
    tokenizer = AutoTokenizer.from_pretrained("bert-base-uncased")
    
    config = BertConfig(
        vocab_size=tokenizer.vocab_size,
        hidden_size=CONFIG["hidden_size"],
        num_hidden_layers=CONFIG["num_hidden_layers"],
        num_attention_heads=CONFIG["num_attention_heads"],
        intermediate_size=CONFIG["intermediate_size"],
        hidden_dropout_prob=CONFIG["hidden_dropout_prob"],
        attention_probs_dropout_prob=CONFIG["attention_probs_dropout_prob"],
        num_labels=len(VOCAB),
        problem_type="multi_label_classification"
    )
    
    model = BertForSequenceClassification(config)
    model.to(device)
    
    total_params = sum(p.numel() for p in model.parameters())
    trainable_params = sum(p.numel() for p in model.parameters() if p.requires_grad)
    print(f"   Total parameters: {total_params:,}")
    print(f"   Trainable: {trainable_params:,}")
    
    # Split data
    train_data, val_data = train_test_split(data, test_size=0.05, random_state=42)
    print(f"\n📊 Train: {len(train_data):,}, Val: {len(val_data):,}")
    
    # Datasets
    train_dataset = SyntheticMathDataset(train_data, tokenizer, VOCAB, CONFIG["max_length"])
    val_dataset = SyntheticMathDataset(val_data, tokenizer, VOCAB, CONFIG["max_length"])
    
    train_loader = DataLoader(train_dataset, batch_size=CONFIG["batch_size"], shuffle=True, num_workers=2, pin_memory=True)
    val_loader = DataLoader(val_dataset, batch_size=CONFIG["batch_size"], num_workers=2, pin_memory=True)
    
    # Optimizer
    optimizer = torch.optim.AdamW(
        model.parameters(), 
        lr=CONFIG["learning_rate"],
        weight_decay=CONFIG["weight_decay"]
    )
    
    total_steps = len(train_loader) * CONFIG["epochs"]
    warmup_steps = int(total_steps * CONFIG["warmup_ratio"])
    scheduler = get_cosine_schedule_with_warmup(optimizer, warmup_steps, total_steps)
    
    print(f"\n⚙️  Training Configuration:")
    print(f"   Epochs: {CONFIG['epochs']}")
    print(f"   Batch size: {CONFIG['batch_size']}")
    print(f"   Steps per epoch: {len(train_loader):,}")
    print(f"   Total steps: {total_steps:,}")
    print(f"   Warmup steps: {warmup_steps:,}")
    
    # Training loop
    print("\n" + "=" * 70)
    print("  TRAINING START")
    print("=" * 70)
    
    best_val_loss = float("inf")
    history = []
    
    for epoch in range(1, CONFIG["epochs"] + 1):
        print(f"\n{'─' * 70}")
        print(f"Epoch {epoch}/{CONFIG['epochs']}")
        print('─' * 70)
        
        train_loss = train_epoch(model, train_loader, optimizer, scheduler, device, epoch)
        val_metrics = evaluate(model, val_loader, device)
        
        history.append({
            "epoch": epoch,
            "train_loss": train_loss,
            **val_metrics
        })
        
        print(f"\n📈 Results:")
        print(f"   Train Loss: {train_loss:.4f}")
        print(f"   Val Loss: {val_metrics['loss']:.4f}")
        print(f"   Exact Match: {val_metrics['exact_match']:.4f}")
        print(f"   Per-Label Acc: {val_metrics['per_label_acc']:.4f}")
        
        # Save checkpoint
        if epoch % CONFIG["save_every_n_epochs"] == 0:
            checkpoint_dir = f"{CONFIG['output_dir']}/checkpoint-{epoch}"
            os.makedirs(checkpoint_dir, exist_ok=True)
            model.save_pretrained(checkpoint_dir)
            print(f"   💾 Checkpoint saved: {checkpoint_dir}")
        
        # Save best model
        if val_metrics["loss"] < best_val_loss:
            best_val_loss = val_metrics["loss"]
            os.makedirs(CONFIG["output_dir"], exist_ok=True)
            model.save_pretrained(CONFIG["output_dir"])
            tokenizer.save_pretrained(CONFIG["output_dir"])
            print(f"   ⭐ New best model saved!")
    
    # Save training history
    with open(f"{CONFIG['output_dir']}/training_history.json", "w") as f:
        json.dump(history, f, indent=2)
    
    # Export to ONNX - robust version
    print("\n📦 Exporting to ONNX...")
    onnx_path = f"{CONFIG['output_dir']}/mathbert_1m.onnx"
    
    try:
        # Move model to CPU and set eval mode
        model_cpu = model.cpu()
        model_cpu.eval()
        
        # Create dummy inputs with correct shapes
        dummy_input_ids = torch.zeros(1, CONFIG["max_length"], dtype=torch.long)
        dummy_attention_mask = torch.ones(1, CONFIG["max_length"], dtype=torch.long)
        
        # Disable gradient computation
        with torch.no_grad():
            torch.onnx.export(
                model_cpu,
                (dummy_input_ids, dummy_attention_mask),
                onnx_path,
                input_names=["input_ids", "attention_mask"],
                output_names=["logits"],
                dynamic_axes={
                    "input_ids": {0: "batch_size", 1: "sequence"},
                    "attention_mask": {0: "batch_size", 1: "sequence"},
                    "logits": {0: "batch_size"}
                },
                opset_version=14,
                do_constant_folding=True,
                export_params=True,
            )
        
        # Verify the export
        import os
        if os.path.exists(onnx_path):
            size_mb = os.path.getsize(onnx_path) / (1024 * 1024)
            print(f"   ✅ ONNX saved: {onnx_path} ({size_mb:.1f} MB)")
        else:
            print(f"   ⚠️ ONNX file not created")
            
    except Exception as e:
        print(f"   ⚠️ ONNX export failed: {e}")
        print("   💡 Saving as SafeTensors instead...")
        try:
            from safetensors.torch import save_file
            state_dict = {k: v.cpu() for k, v in model.state_dict().items()}
            safetensors_path = f"{CONFIG['output_dir']}/model.safetensors"
            save_file(state_dict, safetensors_path)
            print(f"   ✅ SafeTensors saved: {safetensors_path}")
        except ImportError:
            # SafeTensors not available, save as PyTorch
            torch.save(model.state_dict(), f"{CONFIG['output_dir']}/pytorch_model.bin")
            print(f"   ✅ PyTorch model saved: {CONFIG['output_dir']}/pytorch_model.bin")
    
    # Also save vocab for inference
    vocab_path = f"{CONFIG['output_dir']}/vocab.json"
    with open(vocab_path, "w") as f:
        json.dump({"substitutions": VOCAB}, f, indent=2)
    print(f"   ✅ Vocab saved: {vocab_path}")

    print("\n" + "=" * 70)
    print("  TRAINING COMPLETE!")
    print(f"  Best model: {CONFIG['output_dir']}")
    print(f"  Best val loss: {best_val_loss:.4f}")
    print("=" * 70)

if __name__ == "__main__":
    main()

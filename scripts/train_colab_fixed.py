"""
LEMMA MathBERT Training - FIXED VERSION
========================================
Fixes:
1. Uses pretrained BERT (not random init)
2. Proper F1 metrics for multilabel
3. Mixed precision (AMP) for 2x speed
4. Gradient accumulation for larger effective batch
5. More DataLoader workers
6. Throttled saving

Usage on Colab:
  !pip install transformers torch accelerate tqdm scikit-learn -q
  !python train_colab_fixed.py
"""

import torch
import torch.nn as nn
from torch.utils.data import Dataset, DataLoader
from torch.cuda.amp import autocast, GradScaler
from transformers import (
    BertForSequenceClassification,
    AutoTokenizer, 
    get_cosine_schedule_with_warmup
)
from sklearn.model_selection import train_test_split
from sklearn.metrics import f1_score, precision_score, recall_score
import numpy as np
import json
import os
from tqdm.auto import tqdm

# ============================================================================
# CONFIGURATION - FIXED
# ============================================================================
VOCAB = [
    "x = 0", "y = 0", "x = y", "x = 1", "y = 1",
    "a = b = c = 1", "abc = 1 constraint", "Apply AM-GM", "Apply Cauchy-Schwarz",
    "Assume f is linear", "Assume f is injective", "Assume f is monotonic",
    "Check small cases", "Use modular arithmetic", "Homogenize", "WLOG assume ordering",
    "Substitute c = 1/(ab)", "y = f(x)", "x = -y", "Consider p = 2 separately",
]

CONFIG = {
    # Use PRETRAINED bert-base (not random init!)
    "model_name": "bert-base-uncased",
    
    # Training - optimized for T4
    "max_length": 128,           # Shorter for speed
    "epochs": 10,                # Reasonable
    "batch_size": 32,            # Safe for T4
    "gradient_accumulation": 4,  # Effective batch = 128
    "learning_rate": 2e-5,
    "warmup_ratio": 0.1,
    "weight_decay": 0.01,
    "max_samples": 200000,       # 200K is enough for good model
    
    # DataLoader
    "num_workers": 4,
    
    # Saving
    "save_every_n_epochs": 2,    # Don't save every best
    
    # Output
    "output_dir": "lemma_mathbert_v2",
}

# ============================================================================
# DATASET
# ============================================================================
class MathDataset(Dataset):
    def __init__(self, data, tokenizer, vocab, max_length=128):
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
# METRICS - Proper multilabel metrics
# ============================================================================
def compute_metrics(preds, labels):
    """Compute proper multilabel metrics"""
    preds_binary = (preds > 0.5).astype(int)
    labels_binary = labels.astype(int)
    
    # Flatten for per-sample metrics
    micro_f1 = f1_score(labels_binary, preds_binary, average='micro', zero_division=0)
    macro_f1 = f1_score(labels_binary, preds_binary, average='macro', zero_division=0)
    
    # Exact match (for reference only)
    exact_match = (preds_binary == labels_binary).all(axis=1).mean()
    
    # Per-label stats
    precision = precision_score(labels_binary, preds_binary, average='micro', zero_division=0)
    recall = recall_score(labels_binary, preds_binary, average='micro', zero_division=0)
    
    return {
        "micro_f1": micro_f1,
        "macro_f1": macro_f1,
        "exact_match": exact_match,
        "precision": precision,
        "recall": recall,
    }

# ============================================================================
# TRAINING WITH AMP
# ============================================================================
def train_epoch(model, loader, optimizer, scheduler, scaler, device, accumulation_steps):
    model.train()
    total_loss = 0
    optimizer.zero_grad()
    
    pbar = tqdm(loader, desc="Training")
    for step, batch in enumerate(pbar):
        with autocast():  # Mixed precision
            outputs = model(
                input_ids=batch["input_ids"].to(device),
                attention_mask=batch["attention_mask"].to(device),
                labels=batch["labels"].to(device)
            )
            loss = outputs.loss / accumulation_steps
        
        scaler.scale(loss).backward()
        
        if (step + 1) % accumulation_steps == 0:
            scaler.unscale_(optimizer)
            torch.nn.utils.clip_grad_norm_(model.parameters(), 1.0)
            scaler.step(optimizer)
            scaler.update()
            scheduler.step()
            optimizer.zero_grad()
        
        total_loss += loss.item() * accumulation_steps
        pbar.set_postfix({"loss": f"{loss.item() * accumulation_steps:.4f}"})
    
    return total_loss / len(loader)

def evaluate(model, loader, device):
    model.eval()
    total_loss = 0
    all_preds = []
    all_labels = []
    
    with torch.no_grad():
        for batch in tqdm(loader, desc="Evaluating"):
            with autocast():
                outputs = model(
                    input_ids=batch["input_ids"].to(device),
                    attention_mask=batch["attention_mask"].to(device),
                    labels=batch["labels"].to(device)
                )
            total_loss += outputs.loss.item()
            
            preds = torch.sigmoid(outputs.logits).cpu().numpy()
            all_preds.append(preds)
            all_labels.append(batch["labels"].numpy())
    
    all_preds = np.concatenate(all_preds)
    all_labels = np.concatenate(all_labels)
    
    metrics = compute_metrics(all_preds, all_labels)
    metrics["loss"] = total_loss / len(loader)
    
    return metrics

# ============================================================================
# MAIN
# ============================================================================
def main():
    print("=" * 70)
    print("  LEMMA MathBERT Training - FIXED VERSION")
    print("  Pretrained BERT | AMP | Proper Metrics")
    print("=" * 70)
    
    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
    print(f"\n🖥️  Device: {device}")
    if device.type == "cuda":
        print(f"   GPU: {torch.cuda.get_device_name(0)}")
        print(f"   Memory: {torch.cuda.get_device_properties(0).total_memory / 1e9:.1f} GB")
    
    # Load data
    print("\n📂 Loading data...")
    data_file = "synthetic_1000000.json"
    if not os.path.exists(data_file):
        data_file = "data/synthetic_1000000.json"
    
    with open(data_file, "r", encoding="utf-8") as f:
        data = json.load(f)
    print(f"   Loaded {len(data):,} problems")
    
    if CONFIG["max_samples"]:
        import random
        random.seed(42)
        data = random.sample(data, min(CONFIG["max_samples"], len(data)))
        print(f"   Subsampled to {len(data):,}")
    
    # Load PRETRAINED model (not random init!)
    print(f"\n🧠 Loading pretrained {CONFIG['model_name']}...")
    tokenizer = AutoTokenizer.from_pretrained(CONFIG["model_name"])
    model = BertForSequenceClassification.from_pretrained(
        CONFIG["model_name"],
        num_labels=len(VOCAB),
        problem_type="multi_label_classification"
    )
    model.to(device)
    
    total_params = sum(p.numel() for p in model.parameters())
    print(f"   Parameters: {total_params:,}")
    print(f"   ✓ PRETRAINED weights loaded (not random init)")
    
    # Split
    train_data, val_data = train_test_split(data, test_size=0.05, random_state=42)
    print(f"\n📊 Train: {len(train_data):,}, Val: {len(val_data):,}")
    
    # Datasets
    train_dataset = MathDataset(train_data, tokenizer, VOCAB, CONFIG["max_length"])
    val_dataset = MathDataset(val_data, tokenizer, VOCAB, CONFIG["max_length"])
    
    train_loader = DataLoader(
        train_dataset, 
        batch_size=CONFIG["batch_size"], 
        shuffle=True, 
        num_workers=CONFIG["num_workers"],
        pin_memory=True,
        persistent_workers=True
    )
    val_loader = DataLoader(
        val_dataset, 
        batch_size=CONFIG["batch_size"] * 2,  # Bigger for eval
        num_workers=CONFIG["num_workers"],
        pin_memory=True
    )
    
    # Optimizer with AMP
    optimizer = torch.optim.AdamW(
        model.parameters(), 
        lr=CONFIG["learning_rate"],
        weight_decay=CONFIG["weight_decay"]
    )
    scaler = GradScaler()  # For AMP
    
    effective_batch = CONFIG["batch_size"] * CONFIG["gradient_accumulation"]
    steps_per_epoch = len(train_loader) // CONFIG["gradient_accumulation"]
    total_steps = steps_per_epoch * CONFIG["epochs"]
    warmup_steps = int(total_steps * CONFIG["warmup_ratio"])
    
    scheduler = get_cosine_schedule_with_warmup(optimizer, warmup_steps, total_steps)
    
    print(f"\n⚙️  Training Configuration:")
    print(f"   Epochs: {CONFIG['epochs']}")
    print(f"   Batch size: {CONFIG['batch_size']} x {CONFIG['gradient_accumulation']} = {effective_batch}")
    print(f"   Steps per epoch: {steps_per_epoch:,}")
    print(f"   Total steps: {total_steps:,}")
    print(f"   ✓ Mixed Precision (AMP) enabled")
    print(f"   ✓ Gradient accumulation enabled")
    
    # Training
    print("\n" + "=" * 70)
    print("  TRAINING START")
    print("=" * 70)
    
    best_f1 = 0
    history = []
    
    for epoch in range(1, CONFIG["epochs"] + 1):
        print(f"\n{'─' * 70}")
        print(f"Epoch {epoch}/{CONFIG['epochs']}")
        print('─' * 70)
        
        train_loss = train_epoch(
            model, train_loader, optimizer, scheduler, scaler, 
            device, CONFIG["gradient_accumulation"]
        )
        val_metrics = evaluate(model, val_loader, device)
        
        history.append({"epoch": epoch, "train_loss": train_loss, **val_metrics})
        
        print(f"\n📈 Results:")
        print(f"   Train Loss: {train_loss:.4f}")
        print(f"   Val Loss: {val_metrics['loss']:.4f}")
        print(f"   Micro F1: {val_metrics['micro_f1']:.4f}")
        print(f"   Macro F1: {val_metrics['macro_f1']:.4f}")
        print(f"   Precision: {val_metrics['precision']:.4f}")
        print(f"   Recall: {val_metrics['recall']:.4f}")
        
        # Save checkpoint (throttled)
        if epoch % CONFIG["save_every_n_epochs"] == 0:
            checkpoint_dir = f"{CONFIG['output_dir']}/checkpoint-{epoch}"
            os.makedirs(checkpoint_dir, exist_ok=True)
            torch.save(model.state_dict(), f"{checkpoint_dir}/model.pt")
            print(f"   💾 Checkpoint saved")
        
        # Save best (by F1, not loss)
        if val_metrics["micro_f1"] > best_f1:
            best_f1 = val_metrics["micro_f1"]
            os.makedirs(CONFIG["output_dir"], exist_ok=True)
            model.save_pretrained(CONFIG["output_dir"])
            tokenizer.save_pretrained(CONFIG["output_dir"])
            print(f"   ⭐ New best model (F1: {best_f1:.4f})")
    
    # Save history and vocab
    with open(f"{CONFIG['output_dir']}/history.json", "w") as f:
        json.dump(history, f, indent=2)
    with open(f"{CONFIG['output_dir']}/vocab.json", "w") as f:
        json.dump({"substitutions": VOCAB}, f, indent=2)
    
    print("\n" + "=" * 70)
    print("  TRAINING COMPLETE!")
    print(f"  Best Micro F1: {best_f1:.4f}")
    print(f"  Model: {CONFIG['output_dir']}")
    print("=" * 70)

if __name__ == "__main__":
    main()

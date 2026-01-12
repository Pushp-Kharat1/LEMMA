"""
MathBERT Training on 1M Synthetic Problems
==========================================
Trains substitution prediction model on synthetic_1000000.json

Usage:
  python train_on_synthetic.py

For GPU (Google Colab):
  !pip install transformers torch accelerate -q
  python train_on_synthetic.py
"""

import torch
import torch.nn as nn
from torch.utils.data import Dataset, DataLoader
from transformers import AutoTokenizer, AutoModelForSequenceClassification, get_cosine_schedule_with_warmup
from sklearn.model_selection import train_test_split
import numpy as np
import json
import os
from tqdm import tqdm

# Substitution vocabulary (must match Rust SubstitutionPredictor)
VOCAB = [
    "x = 0", "y = 0", "x = y", "x = 1", "y = 1",
    "a = b = c = 1", "abc = 1 constraint", "Apply AM-GM", "Apply Cauchy-Schwarz",
    "Assume f is linear", "Assume f is injective", "Assume f is monotonic",
    "Check small cases", "Use modular arithmetic", "Homogenize", "WLOG assume ordering",
    "Substitute c = 1/(ab)", "y = f(x)", "x = -y", "Consider p = 2 separately",
]

CONFIG = {
    "model_name": "distilbert-base-uncased",  # Fast and effective
    "max_length": 256,
    "epochs": 3,  # Fewer epochs for large dataset
    "batch_size": 32,  # Larger batch for efficiency
    "learning_rate": 2e-5,
    "warmup_ratio": 0.1,
    "max_samples": 100000,  # Train on 100K for speed (use None for all 1M)
    "output_dir": "model/mathbert_synthetic",
}

class SyntheticDataset(Dataset):
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
        
        # Get substitutions from data
        subs = item.get("substitutions", [])
        
        # Create multi-label target
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

def train_epoch(model, loader, optimizer, scheduler, device):
    model.train()
    total_loss = 0
    pbar = tqdm(loader, desc="Training")
    
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
        pbar.set_postfix({"loss": f"{loss.item():.4f}"})
    
    return total_loss / len(loader)

def evaluate(model, loader, device):
    model.eval()
    total_loss = 0
    correct = 0
    total = 0
    
    with torch.no_grad():
        for batch in tqdm(loader, desc="Evaluating"):
            outputs = model(
                input_ids=batch["input_ids"].to(device),
                attention_mask=batch["attention_mask"].to(device),
                labels=batch["labels"].to(device)
            )
            total_loss += outputs.loss.item()
            preds = (torch.sigmoid(outputs.logits) > 0.5).float()
            correct += (preds == batch["labels"].to(device)).all(dim=1).sum().item()
            total += len(batch["labels"])
    
    return {"loss": total_loss / len(loader), "accuracy": correct / total}

def main():
    print("=" * 60)
    print("MathBERT Training on 1M Synthetic Problems")
    print("=" * 60)
    
    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
    print(f"Device: {device}")
    
    # Load synthetic data
    data_path = "data/synthetic_1000000.json"
    print(f"Loading {data_path}...")
    
    with open(data_path, "r", encoding="utf-8") as f:
        data = json.load(f)
    print(f"Loaded {len(data):,} synthetic problems")
    
    # Subsample for faster training
    if CONFIG["max_samples"] and len(data) > CONFIG["max_samples"]:
        import random
        random.seed(42)
        data = random.sample(data, CONFIG["max_samples"])
        print(f"Subsampled to {len(data):,} problems for training")
    
    # Load model
    print(f"Loading {CONFIG['model_name']}...")
    tokenizer = AutoTokenizer.from_pretrained(CONFIG["model_name"])
    model = AutoModelForSequenceClassification.from_pretrained(
        CONFIG["model_name"], 
        num_labels=len(VOCAB), 
        problem_type="multi_label_classification"
    )
    model.to(device)
    
    # Split data
    train_data, val_data = train_test_split(data, test_size=0.1, random_state=42)
    print(f"Train: {len(train_data):,}, Val: {len(val_data):,}")
    
    # Create datasets
    train_dataset = SyntheticDataset(train_data, tokenizer, VOCAB, CONFIG["max_length"])
    val_dataset = SyntheticDataset(val_data, tokenizer, VOCAB, CONFIG["max_length"])
    
    train_loader = DataLoader(train_dataset, batch_size=CONFIG["batch_size"], shuffle=True, num_workers=0)
    val_loader = DataLoader(val_dataset, batch_size=CONFIG["batch_size"], num_workers=0)
    
    # Optimizer & scheduler
    optimizer = torch.optim.AdamW(model.parameters(), lr=CONFIG["learning_rate"])
    total_steps = len(train_loader) * CONFIG["epochs"]
    warmup_steps = int(total_steps * CONFIG["warmup_ratio"])
    scheduler = get_cosine_schedule_with_warmup(optimizer, warmup_steps, total_steps)
    
    print(f"\nTraining Configuration:")
    print(f"  Epochs: {CONFIG['epochs']}")
    print(f"  Batch size: {CONFIG['batch_size']}")
    print(f"  Total steps: {total_steps:,}")
    print(f"  Warmup steps: {warmup_steps:,}")
    print()
    
    # Training loop
    best_val_loss = float("inf")
    
    for epoch in range(CONFIG["epochs"]):
        print(f"\n{'='*60}")
        print(f"Epoch {epoch + 1}/{CONFIG['epochs']}")
        print('='*60)
        
        train_loss = train_epoch(model, train_loader, optimizer, scheduler, device)
        val_metrics = evaluate(model, val_loader, device)
        
        print(f"\nTrain Loss: {train_loss:.4f}")
        print(f"Val Loss: {val_metrics['loss']:.4f}")
        print(f"Val Accuracy: {val_metrics['accuracy']:.4f}")
        
        if val_metrics["loss"] < best_val_loss:
            best_val_loss = val_metrics["loss"]
            os.makedirs(CONFIG["output_dir"], exist_ok=True)
            model.save_pretrained(CONFIG["output_dir"])
            tokenizer.save_pretrained(CONFIG["output_dir"])
            print(f"✓ Saved best model to {CONFIG['output_dir']}")
    
    print("\n" + "=" * 60)
    print("Training Complete!")
    print(f"Best model saved to: {CONFIG['output_dir']}")
    print("=" * 60)
    
    # Export to ONNX
    print("\nExporting to ONNX...")
    try:
        dummy_input = tokenizer("Test problem", return_tensors="pt", padding="max_length", max_length=256)
        model.eval()
        torch.onnx.export(
            model,
            (dummy_input["input_ids"], dummy_input["attention_mask"]),
            f"{CONFIG['output_dir']}/substitution_model.onnx",
            input_names=["input_ids", "attention_mask"],
            output_names=["logits"],
            dynamic_axes={"input_ids": {0: "batch"}, "attention_mask": {0: "batch"}, "logits": {0: "batch"}}
        )
        print(f"✓ ONNX model saved to {CONFIG['output_dir']}/substitution_model.onnx")
    except Exception as e:
        print(f"ONNX export failed: {e}")

if __name__ == "__main__":
    main()

"""
MathBERT Training Script for LEMMA
Uses verified_imo_problems.json

Usage in Google Colab:
1. Upload this script and data/verified_imo_problems.json
2. Run: !pip install transformers torch scikit-learn onnx onnxruntime onnxscript -q
3. Run all cells
4. Download lemma_model.zip
"""

import torch
import torch.nn as nn
from torch.utils.data import Dataset, DataLoader
from transformers import AutoTokenizer, AutoModelForSequenceClassification, get_cosine_schedule_with_warmup
from sklearn.model_selection import train_test_split
import numpy as np
import json
import os

VOCAB = [
    "x = 0", "y = 0", "x = y", "x = 1", "y = 1",
    "a = b = c = 1", "abc = 1 constraint", "Apply AM-GM", "Apply Cauchy-Schwarz",
    "Assume f is linear", "Assume f is injective", "Assume f is monotonic",
    "Check small cases", "Use modular arithmetic", "Homogenize", "WLOG assume ordering",
    "Substitute c = 1/(ab)", "y = f(x)", "x = -y", "Consider p = 2 separately",
]

CONFIG = {
    "model_name": "tbs17/MathBERT",
    "fallback_model": "distilbert-base-uncased",
    "max_length": 256,
    "epochs": 20,
    "batch_size": 16,
    "learning_rate": 5e-5,  
    "warmup_ratio": 0.06,
    "patience": 4,
    "output_dir": "lemma_model",
}

class IMODataset(Dataset):
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
        subs = item["subs"]
        
        encoding = self.tokenizer(text, max_length=self.max_length, padding="max_length", truncation=True, return_tensors="pt")
        
        label = torch.zeros(len(self.vocab_to_idx))
        for sub in subs:
            if sub in self.vocab_to_idx:
                label[self.vocab_to_idx[sub]] = 1.0
                
        return {
            "input_ids": encoding["input_ids"].squeeze(),
            "attention_mask": encoding["attention_mask"].squeeze(),
            "labels": label
        }

def train_epoch(model, loader, optimizer, scheduler, device):
    model.train()
    total_loss = 0
    for batch in loader:
        optimizer.zero_grad()
        outputs = model(input_ids=batch["input_ids"].to(device), attention_mask=batch["attention_mask"].to(device))
        loss = nn.BCEWithLogitsLoss()(outputs.logits, batch["labels"].to(device))
        loss.backward()
        torch.nn.utils.clip_grad_norm_(model.parameters(), 1.0)
        optimizer.step()
        scheduler.step()
        total_loss += loss.item()
    return total_loss / len(loader)

def evaluate(model, loader, device):
    model.eval()
    total_loss, correct, total = 0, 0, 0
    with torch.no_grad():
        for batch in loader:
            outputs = model(input_ids=batch["input_ids"].to(device), attention_mask=batch["attention_mask"].to(device))
            loss = nn.BCEWithLogitsLoss()(outputs.logits, batch["labels"].to(device))
            total_loss += loss.item()
            preds = (torch.sigmoid(outputs.logits) > 0.5).float()
            correct += (preds == batch["labels"].to(device)).all(dim=1).sum().item()
            total += len(batch["labels"])
    return {"loss": total_loss / len(loader), "accuracy": correct / total}

def main():
    print("="*60)
    print("LEMMA MathBERT Training")
    print("="*60)
    
    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
    print(f"Device: {device}")
    
    with open("real_imo_problems.json", "r", encoding="utf-8") as f:
        data = json.load(f)
    print(f"Loaded {len(data)} verified IMO problems")
    
    try:
        tokenizer = AutoTokenizer.from_pretrained(CONFIG["model_name"])
        model = AutoModelForSequenceClassification.from_pretrained(CONFIG["model_name"], num_labels=len(VOCAB), problem_type="multi_label_classification")
        print(f"Using: {CONFIG['model_name']}")
    except:
        print(f"MathBERT unavailable, using {CONFIG['fallback_model']}")
        tokenizer = AutoTokenizer.from_pretrained(CONFIG["fallback_model"])
        model = AutoModelForSequenceClassification.from_pretrained(CONFIG["fallback_model"], num_labels=len(VOCAB), problem_type="multi_label_classification")
    
    model.to(device)
    
    train_data, val_data = train_test_split(data, test_size=0.2, random_state=42)
    print(f"Train: {len(train_data)}, Val: {len(val_data)}")
    
    train_loader = DataLoader(IMODataset(train_data, tokenizer, VOCAB, CONFIG["max_length"]), batch_size=CONFIG["batch_size"], shuffle=True)
    val_loader = DataLoader(IMODataset(val_data, tokenizer, VOCAB, CONFIG["max_length"]), batch_size=CONFIG["batch_size"])
    
    optimizer = torch.optim.AdamW(model.parameters(), lr=CONFIG["learning_rate"])
    scheduler = get_cosine_schedule_with_warmup(optimizer, num_warmup_steps=int(len(train_loader) * CONFIG["epochs"] * CONFIG["warmup_ratio"]), num_training_steps=len(train_loader) * CONFIG["epochs"])
    
    best_acc, patience_counter = 0, 0
    
    for epoch in range(CONFIG["epochs"]):
        train_loss = train_epoch(model, train_loader, optimizer, scheduler, device)
        val_metrics = evaluate(model, val_loader, device)
        
        print(f"Epoch {epoch+1:2d} | Train Loss: {train_loss:.4f} | Val Acc: {val_metrics['accuracy']*100:.1f}%")
        
        if val_metrics["accuracy"] > best_acc:
            best_acc = val_metrics["accuracy"]
            patience_counter = 0
            torch.save(model.state_dict(), "best_model.pt")
        else:
            patience_counter += 1
            if patience_counter >= CONFIG["patience"]:
                print("Early stopping")
                break
    
    model.load_state_dict(torch.load("best_model.pt"))
    
    os.makedirs(CONFIG["output_dir"], exist_ok=True)
    with open(f"{CONFIG['output_dir']}/vocab.json", "w") as f:
        json.dump(VOCAB, f)
    tokenizer.save_pretrained(CONFIG["output_dir"])
    
    print("\nExporting to ONNX...")
    model.eval().cpu()
    dummy = tokenizer("Find all functions", return_tensors="pt", max_length=CONFIG["max_length"], truncation=True, padding="max_length")
    torch.onnx.export(model, (dummy["input_ids"], dummy["attention_mask"]), f"{CONFIG['output_dir']}/substitution_model.onnx", input_names=["input_ids", "attention_mask"], output_names=["logits"], opset_version=14)
    
    print(f"\nDone! Best accuracy: {best_acc*100:.1f}%")
    print(f"Model saved to {CONFIG['output_dir']}/")
    print("\nTo download: !zip -r lemma_model.zip lemma_model/")

if __name__ == "__main__":
    main()

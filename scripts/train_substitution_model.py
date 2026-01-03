#!/usr/bin/env python3
"""
DistilBERT Fine-tuning for IMO Substitution Prediction

Fine-tunes DistilBERT to predict useful substitutions for IMO problems.
Exports model to ONNX for Rust inference.

Usage:
    python train_substitution_model.py --data annotated_problems.json --output model/
    
Requirements:
    pip install transformers torch datasets onnx onnxruntime
"""

import json
import argparse
from pathlib import Path
from typing import List, Dict, Tuple
import random

import torch
from torch.utils.data import Dataset, DataLoader
from transformers import (
    DistilBertTokenizer,
    DistilBertForSequenceClassification,
    DistilBertConfig,
    Trainer,
    TrainingArguments,
    EarlyStoppingCallback,
)
from sklearn.preprocessing import MultiLabelBinarizer
from sklearn.model_selection import train_test_split
import numpy as np

# ============================================================================
# Data Preparation
# ============================================================================

# Common substitutions we want to predict
SUBSTITUTION_VOCAB = [
    "x = 0",
    "y = 0",
    "x = y",
    "x = 1",
    "y = 1",
    "a = b = c = 1",
    "abc = 1 constraint",
    "Apply AM-GM",
    "Apply Cauchy-Schwarz",
    "Assume f is linear",
    "Assume f is injective",
    "Assume f is monotonic",
    "Check small cases",
    "Use modular arithmetic",
    "Homogenize",
    "WLOG assume ordering",
    "Substitute c = 1/(ab)",
    "y = f(x)",
    "x = -y",
    "Consider p = 2 separately",
]

class SubstitutionDataset(Dataset):
    """Dataset for substitution prediction."""
    
    def __init__(self, problems: List[Dict], tokenizer, mlb, max_length: int = 256):
        self.tokenizer = tokenizer
        self.mlb = mlb
        self.max_length = max_length
        self.data = []
        
        for p in problems:
            text = p.get('problem_text', '')
            subs = p.get('substitutions', [])
            
            # Normalize substitutions to match vocabulary
            normalized_subs = self._normalize_substitutions(subs)
            
            if text and normalized_subs:
                self.data.append({
                    'text': text,
                    'substitutions': normalized_subs,
                })
    
    def _normalize_substitutions(self, subs: List[str]) -> List[str]:
        """Map substitutions to vocabulary."""
        normalized = []
        for s in subs:
            s_lower = s.lower().strip()
            
            # Match against vocabulary
            for vocab_sub in SUBSTITUTION_VOCAB:
                if vocab_sub.lower() in s_lower or s_lower in vocab_sub.lower():
                    normalized.append(vocab_sub)
                    break
            else:
                # Try to match key patterns
                if 'x = 0' in s_lower or 'x=0' in s_lower:
                    normalized.append('x = 0')
                elif 'y = 0' in s_lower or 'y=0' in s_lower:
                    normalized.append('y = 0')
                elif 'am-gm' in s_lower or 'amgm' in s_lower:
                    normalized.append('Apply AM-GM')
                elif 'cauchy' in s_lower:
                    normalized.append('Apply Cauchy-Schwarz')
                elif 'linear' in s_lower:
                    normalized.append('Assume f is linear')
                    
        return list(set(normalized))[:5]  # Dedupe and limit
    
    def __len__(self):
        return len(self.data)
    
    def __getitem__(self, idx):
        item = self.data[idx]
        
        # Tokenize
        encoding = self.tokenizer(
            item['text'],
            truncation=True,
            max_length=self.max_length,
            padding='max_length',
            return_tensors='pt',
        )
        
        # Multi-label target
        labels = self.mlb.transform([item['substitutions']])
        
        return {
            'input_ids': encoding['input_ids'].squeeze(),
            'attention_mask': encoding['attention_mask'].squeeze(),
            'labels': torch.tensor(labels.squeeze(), dtype=torch.float),
        }

# ============================================================================
# Model
# ============================================================================

class DistilBertSubstitutionModel(torch.nn.Module):
    """DistilBERT with multi-label classification head."""
    
    def __init__(self, num_labels: int, pretrained: str = 'distilbert-base-uncased'):
        super().__init__()
        
        self.config = DistilBertConfig.from_pretrained(
            pretrained,
            num_labels=num_labels,
            problem_type='multi_label_classification',
        )
        
        self.bert = DistilBertForSequenceClassification.from_pretrained(
            pretrained,
            config=self.config,
        )
        
    def forward(self, input_ids, attention_mask, labels=None):
        return self.bert(
            input_ids=input_ids,
            attention_mask=attention_mask,
            labels=labels,
        )

# ============================================================================
# Training
# ============================================================================

def compute_metrics(pred):
    """Compute accuracy metrics."""
    labels = pred.label_ids
    preds = (torch.sigmoid(torch.tensor(pred.predictions)) > 0.5).numpy()
    
    # Per-sample accuracy
    exact_match = np.all(preds == labels, axis=1).mean()
    
    # Per-label accuracy
    label_accuracy = (preds == labels).mean()
    
    return {
        'exact_match': exact_match,
        'label_accuracy': label_accuracy,
    }

def train_model(
    data_path: Path,
    output_dir: Path,
    epochs: int = 10,
    batch_size: int = 8,
    learning_rate: float = 2e-5,
):
    """Train the substitution prediction model."""
    
    # Load data
    print(f"Loading data from {data_path}...")
    with open(data_path, 'r', encoding='utf-8') as f:
        problems = json.load(f)
    
    print(f"Loaded {len(problems)} problems")
    
    # Initialize tokenizer and label encoder
    tokenizer = DistilBertTokenizer.from_pretrained('distilbert-base-uncased')
    mlb = MultiLabelBinarizer(classes=SUBSTITUTION_VOCAB)
    mlb.fit([SUBSTITUTION_VOCAB])  # Fit on full vocab
    
    # Create dataset
    dataset = SubstitutionDataset(problems, tokenizer, mlb)
    print(f"Created dataset with {len(dataset)} samples")
    
    if len(dataset) < 2:
        print("Not enough data for training. Add more annotated problems.")
        return
    
    # Split
    train_size = int(0.8 * len(dataset))
    train_dataset = torch.utils.data.Subset(dataset, range(train_size))
    eval_dataset = torch.utils.data.Subset(dataset, range(train_size, len(dataset)))
    
    # Model
    model = DistilBertSubstitutionModel(num_labels=len(SUBSTITUTION_VOCAB))
    
    # Training args
    training_args = TrainingArguments(
        output_dir=str(output_dir / 'checkpoints'),
        num_train_epochs=epochs,
        per_device_train_batch_size=batch_size,
        per_device_eval_batch_size=batch_size,
        warmup_steps=50,
        weight_decay=0.01,
        logging_dir=str(output_dir / 'logs'),
        logging_steps=10,
        evaluation_strategy='epoch',
        save_strategy='epoch',
        load_best_model_at_end=True,
        metric_for_best_model='label_accuracy',
        greater_is_better=True,
    )
    
    # Trainer
    trainer = Trainer(
        model=model.bert,
        args=training_args,
        train_dataset=train_dataset,
        eval_dataset=eval_dataset,
        compute_metrics=compute_metrics,
        callbacks=[EarlyStoppingCallback(early_stopping_patience=3)],
    )
    
    # Train
    print("Starting training...")
    trainer.train()
    
    # Save
    print(f"Saving model to {output_dir}...")
    trainer.save_model(str(output_dir / 'final'))
    tokenizer.save_pretrained(str(output_dir / 'final'))
    
    # Save label encoder
    with open(output_dir / 'label_vocab.json', 'w') as f:
        json.dump(SUBSTITUTION_VOCAB, f)
    
    print("Training complete!")
    return model, tokenizer, mlb

# ============================================================================
# ONNX Export
# ============================================================================

def export_to_onnx(model_dir: Path, output_path: Path):
    """Export trained model to ONNX format for Rust inference."""
    from transformers import DistilBertForSequenceClassification, DistilBertTokenizer
    import torch.onnx
    
    print(f"Loading model from {model_dir}...")
    model = DistilBertForSequenceClassification.from_pretrained(str(model_dir))
    tokenizer = DistilBertTokenizer.from_pretrained(str(model_dir))
    
    model.eval()
    
    # Dummy input
    dummy_text = "Find all functions f: R → R such that f(x + y) = f(x) + f(y)."
    inputs = tokenizer(dummy_text, return_tensors='pt', max_length=256, truncation=True, padding='max_length')
    
    # Export
    print(f"Exporting to {output_path}...")
    torch.onnx.export(
        model,
        (inputs['input_ids'], inputs['attention_mask']),
        str(output_path),
        input_names=['input_ids', 'attention_mask'],
        output_names=['logits'],
        dynamic_axes={
            'input_ids': {0: 'batch'},
            'attention_mask': {0: 'batch'},
            'logits': {0: 'batch'},
        },
        opset_version=14,
    )
    
    print(f"ONNX model saved to {output_path}")
    
    # Verify
    import onnxruntime as ort
    session = ort.InferenceSession(str(output_path))
    result = session.run(
        None,
        {
            'input_ids': inputs['input_ids'].numpy(),
            'attention_mask': inputs['attention_mask'].numpy(),
        }
    )
    print(f"Verification passed! Output shape: {result[0].shape}")

# ============================================================================
# Inference
# ============================================================================

def predict_substitutions(
    text: str,
    model_dir: Path,
    top_k: int = 3,
) -> List[Tuple[str, float]]:
    """Predict top-k substitutions for a problem."""
    from transformers import DistilBertForSequenceClassification, DistilBertTokenizer
    
    # Load
    model = DistilBertForSequenceClassification.from_pretrained(str(model_dir))
    tokenizer = DistilBertTokenizer.from_pretrained(str(model_dir))
    
    with open(model_dir / 'label_vocab.json', 'r') as f:
        labels = json.load(f)
    
    model.eval()
    
    # Tokenize
    inputs = tokenizer(text, return_tensors='pt', max_length=256, truncation=True, padding='max_length')
    
    # Predict
    with torch.no_grad():
        outputs = model(**inputs)
    
    probs = torch.sigmoid(outputs.logits).squeeze().numpy()
    
    # Get top-k
    top_indices = probs.argsort()[-top_k:][::-1]
    results = [(labels[i], float(probs[i])) for i in top_indices]
    
    return results

# ============================================================================
# Main
# ============================================================================

def main():
    parser = argparse.ArgumentParser(description="Train substitution prediction model")
    parser.add_argument("--data", type=Path, required=True, help="Annotated problems JSON")
    parser.add_argument("--output", type=Path, default=Path("model"), help="Output directory")
    parser.add_argument("--epochs", type=int, default=10, help="Training epochs")
    parser.add_argument("--batch-size", type=int, default=8, help="Batch size")
    parser.add_argument("--export-onnx", action="store_true", help="Export to ONNX after training")
    
    args = parser.parse_args()
    
    args.output.mkdir(parents=True, exist_ok=True)
    
    # Train
    train_model(
        data_path=args.data,
        output_dir=args.output,
        epochs=args.epochs,
        batch_size=args.batch_size,
    )
    
    # Export to ONNX
    if args.export_onnx:
        export_to_onnx(
            model_dir=args.output / 'final',
            output_path=args.output / 'substitution_model.onnx',
        )

if __name__ == "__main__":
    main()

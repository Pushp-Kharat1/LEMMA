#!/usr/bin/env python3
"""
Download and Extract Kaggle IMO Dataset

This script downloads the IMO problems dataset from Kaggle and extracts it.

Prerequisites:
    pip install kaggle

Setup Kaggle API:
    1. Go to https://www.kaggle.com/settings
    2. Click "Create New Token" to download kaggle.json
    3. Place kaggle.json in:
       - Windows: C:\Users\<username>\.kaggle\kaggle.json
       - Linux/Mac: ~/.kaggle/kaggle.json
    4. chmod 600 ~/.kaggle/kaggle.json (Linux/Mac only)

Usage:
    python download_imo_dataset.py
"""

import os
import sys
import zipfile
import shutil
from pathlib import Path
import subprocess

# Dataset options - try these Kaggle datasets
KAGGLE_DATASETS = [
    "neelramesh/imo-problems-from-1959-to-2021",  # Most comprehensive
    "ashwani15/imo-problem",
    "neelramesh/imo-problems",
]

# Alternative direct download URLs (if Kaggle doesn't work)
BACKUP_URLS = [
    # These are hypothetical - actual URLs would need to be found
    "https://artofproblemsolving.com/wiki/api.php?action=parse&page=IMO_Problems_and_Solutions",
]

def check_kaggle_installed():
    """Check if kaggle CLI is installed."""
    try:
        result = subprocess.run(['kaggle', '--version'], capture_output=True, text=True)
        return result.returncode == 0
    except FileNotFoundError:
        return False

def install_kaggle():
    """Install kaggle package."""
    print("Installing kaggle package...")
    subprocess.run([sys.executable, '-m', 'pip', 'install', 'kaggle', '-q'])
    print("Kaggle package installed.")

def check_kaggle_credentials():
    """Check if Kaggle credentials are configured."""
    kaggle_dir = Path.home() / '.kaggle'
    kaggle_json = kaggle_dir / 'kaggle.json'
    
    if not kaggle_json.exists():
        # Check Windows location
        win_kaggle = Path(os.environ.get('USERPROFILE', '')) / '.kaggle' / 'kaggle.json'
        if not win_kaggle.exists():
            return False
    return True

def download_from_kaggle(dataset: str, output_dir: Path) -> bool:
    """Download dataset from Kaggle."""
    try:
        print(f"Downloading {dataset}...")
        result = subprocess.run(
            ['kaggle', 'datasets', 'download', '-d', dataset, '-p', str(output_dir)],
            capture_output=True,
            text=True,
        )
        
        if result.returncode == 0:
            print(f"Downloaded: {dataset}")
            return True
        else:
            print(f"Failed to download {dataset}: {result.stderr}")
            return False
            
    except Exception as e:
        print(f"Error downloading {dataset}: {e}")
        return False

def extract_zip(zip_path: Path, output_dir: Path):
    """Extract a zip file."""
    print(f"Extracting {zip_path.name}...")
    
    with zipfile.ZipFile(zip_path, 'r') as zip_ref:
        zip_ref.extractall(output_dir)
    
    print(f"Extracted to {output_dir}")
    
    # List contents
    print("\nExtracted files:")
    for f in output_dir.iterdir():
        size = f.stat().st_size if f.is_file() else "DIR"
        print(f"  {f.name}: {size}")

def create_sample_dataset(output_dir: Path):
    """Create a sample dataset if downloads fail."""
    import json
    
    print("\nCreating sample dataset for testing...")
    
    sample_problems = [
        {
            "year": 2022,
            "problem_number": 3,
            "problem_text": "Find all functions f: ℝ⁺ → ℝ⁺ such that for every x ∈ ℝ⁺, there is exactly one y ∈ ℝ⁺ satisfying xf(y) + yf(x) ≤ 2.",
            "category": "functional_equation",
        },
        {
            "year": 2019,
            "problem_number": 1,
            "problem_text": "Find all functions f: ℤ → ℤ such that for all integers a, b: f(2a) + 2f(b) = f(f(a + b)).",
            "category": "functional_equation",
        },
        {
            "year": 2017,
            "problem_number": 2,
            "problem_text": "Find all functions f: ℝ → ℝ such that f(f(x)f(y)) + f(x + y) = f(xy) for all x, y ∈ ℝ.",
            "category": "functional_equation",
        },
        {
            "year": 2015,
            "problem_number": 5,
            "problem_text": "Find all functions f: ℝ → ℝ that satisfy f(x + f(x + y)) + f(xy) = x + f(x + y) + yf(x).",
            "category": "functional_equation",
        },
        {
            "year": 2024,
            "problem_number": 1,
            "problem_text": "Find all real numbers α such that, for every positive integer n, the sum ⌊α⌋ + ⌊2α⌋ + ... + ⌊nα⌋ is divisible by n.",
            "category": "number_theory",
        },
        {
            "year": 2024,
            "problem_number": 2,
            "problem_text": "Find all pairs (a,b) of positive integers such that gcd(a^n + b, b^n + a) is eventually constant.",
            "category": "number_theory",
        },
        {
            "year": 2008,
            "problem_number": 2,
            "problem_text": "Prove that for positive reals a, b, c with abc = 1: (a-1+1/b)(b-1+1/c)(c-1+1/a) ≤ 1.",
            "category": "algebra",
        },
        {
            "year": 2001,
            "problem_number": 2,
            "problem_text": "Prove that for positive reals a, b, c: a/√(a² + 8bc) + b/√(b² + 8ca) + c/√(c² + 8ab) ≥ 1.",
            "category": "algebra",
        },
    ]
    
    output_file = output_dir / "sample_imo_problems.json"
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump(sample_problems, f, indent=2, ensure_ascii=False)
    
    # Also create CSV format
    csv_file = output_dir / "sample_imo_problems.csv"
    with open(csv_file, 'w', encoding='utf-8') as f:
        f.write("year,problem_number,problem_text,category\n")
        for p in sample_problems:
            text = p['problem_text'].replace('"', '""')
            f.write(f'{p["year"]},{p["problem_number"]},"{text}",{p["category"]}\n')
    
    print(f"Created: {output_file}")
    print(f"Created: {csv_file}")
    print(f"\nSample dataset with {len(sample_problems)} problems ready for testing.")

def main():
    # Setup paths
    script_dir = Path(__file__).parent
    data_dir = script_dir.parent / 'data'
    raw_dir = data_dir / 'raw'
    
    raw_dir.mkdir(parents=True, exist_ok=True)
    
    print("=" * 60)
    print("IMO Dataset Downloader")
    print("=" * 60)
    
    # Check/install kaggle
    if not check_kaggle_installed():
        install_kaggle()
    
    # Check credentials
    if not check_kaggle_credentials():
        print("\n⚠️  Kaggle credentials not found!")
        print("\nTo setup Kaggle API:")
        print("1. Go to https://www.kaggle.com/settings")
        print("2. Scroll to 'API' section")
        print("3. Click 'Create New Token' to download kaggle.json")
        print("4. Place kaggle.json in: ~/.kaggle/ (or %USERPROFILE%\\.kaggle\\ on Windows)")
        print("\nCreating sample dataset instead...")
        create_sample_dataset(raw_dir)
        return
    
    # Try each dataset
    success = False
    for dataset in KAGGLE_DATASETS:
        if download_from_kaggle(dataset, raw_dir):
            success = True
            break
    
    if not success:
        print("\n⚠️  Could not download from Kaggle. Creating sample dataset...")
        create_sample_dataset(raw_dir)
        return
    
    # Extract any zip files
    for zip_file in raw_dir.glob("*.zip"):
        extract_zip(zip_file, raw_dir)
        zip_file.unlink()  # Remove zip after extraction
    
    print("\n" + "=" * 60)
    print("✅ Download complete!")
    print("=" * 60)
    print(f"\nData saved to: {raw_dir}")
    print("\nNext steps:")
    print(f"  python filter_imo_problems.py --input {raw_dir}/<filename>.csv --output ../data/filtered_problems.json")

if __name__ == "__main__":
    main()

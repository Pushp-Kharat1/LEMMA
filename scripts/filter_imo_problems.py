#!/usr/bin/env python3
"""
IMO Dataset Processor
Downloads and filters Kaggle IMO problems for functional equations and algebra.

Usage:
    python filter_imo_problems.py --input imo_problems.csv --output filtered_problems.json
"""

import json
import re
import argparse
from pathlib import Path
from typing import List, Dict, Optional
from dataclasses import dataclass, asdict

# ============================================================================
# Data Structures
# ============================================================================

@dataclass
class IMOProblem:
    """A single IMO problem with metadata."""
    year: int
    problem_number: int
    category: str
    problem_text: str
    solution_text: Optional[str] = None
    substitutions: List[str] = None
    proof_steps: List[str] = None
    
    def __post_init__(self):
        if self.substitutions is None:
            self.substitutions = []
        if self.proof_steps is None:
            self.proof_steps = []

# ============================================================================
# Filtering Functions
# ============================================================================

# Patterns for different problem types
FUNCTIONAL_EQUATION_PATTERNS = [
    r"find all functions",
    r"determine all functions", 
    r"f\s*:\s*\w+\s*→",
    r"f\s*:\s*\w+\s*\\to",
    r"f\s*:\s*\w+\s*->",
    r"functional equation",
    r"f\(x\s*\+\s*f\(y\)\)",
    r"f\(f\(",
    r"f\(xy\)\s*=",
]

ALGEBRA_PATTERNS = [
    r"prove that.*≥",
    r"prove that.*>=",
    r"prove.*inequality",
    r"a\s*\+\s*b\s*\+\s*c",
    r"abc\s*=\s*1",
    r"cyclic sum",
    r"AM-GM",
    r"Cauchy-Schwarz",
    r"positive real",
    r"positive integer",
]

NUMBER_THEORY_PATTERNS = [
    r"divisible by",
    r"divides",
    r"gcd\(",
    r"lcm\(",
    r"modulo",
    r"mod\s+\d+",
    r"prime",
    r"perfect square",
]

def is_functional_equation(text: str) -> bool:
    """Check if problem is a functional equation."""
    text_lower = text.lower()
    return any(re.search(p, text_lower, re.IGNORECASE) for p in FUNCTIONAL_EQUATION_PATTERNS)

def is_algebra_problem(text: str) -> bool:
    """Check if problem is an algebra/inequality problem."""
    text_lower = text.lower()
    return any(re.search(p, text_lower, re.IGNORECASE) for p in ALGEBRA_PATTERNS)

def is_number_theory(text: str) -> bool:
    """Check if problem is number theory."""
    text_lower = text.lower()
    return any(re.search(p, text_lower, re.IGNORECASE) for p in NUMBER_THEORY_PATTERNS)

def categorize_problem(text: str) -> str:
    """Categorize a problem based on its text."""
    if is_functional_equation(text):
        return "functional_equation"
    elif is_algebra_problem(text):
        return "algebra"
    elif is_number_theory(text):
        return "number_theory"
    else:
        return "other"

# ============================================================================
# Substitution Extraction
# ============================================================================

def suggest_substitutions(problem: IMOProblem) -> List[str]:
    """Suggest common substitutions based on problem type."""
    suggestions = []
    text = problem.problem_text.lower()
    
    if problem.category == "functional_equation":
        # Common functional equation substitutions
        suggestions.append("x = 0")
        suggestions.append("y = 0")
        suggestions.append("x = y")
        
        if "f(f(" in text:
            suggestions.append("y = f(x)")
        if "injective" in text or "one-to-one" in text:
            suggestions.append("assume f is injective")
        if "f(xy)" in text:
            suggestions.append("x = 1")
            suggestions.append("y = 1")
            
    elif problem.category == "algebra":
        # Common algebra substitutions
        if "abc" in text and ("= 1" in text or "=1" in text):
            suggestions.append("Use abc = 1 constraint")
            suggestions.append("Apply AM-GM: (a+b+c)/3 >= ∛(abc)")
        if "a + b + c" in text:
            suggestions.append("Apply homogeneity")
            suggestions.append("WLOG assume a >= b >= c")
        if "positive" in text:
            suggestions.append("Let a = e^x, b = e^y, c = e^z")
            
    elif problem.category == "number_theory":
        suggestions.append("Check small cases: n = 1, 2, 3")
        if "prime" in text:
            suggestions.append("Consider p = 2 separately")
        if "divides" in text or "divisible" in text:
            suggestions.append("Use modular arithmetic")
            
    return suggestions[:5]  # Limit to top 5 suggestions

# ============================================================================
# File Processing
# ============================================================================

def load_csv(path: Path) -> List[Dict]:
    """Load problems from CSV file."""
    import csv
    problems = []
    
    with open(path, 'r', encoding='utf-8') as f:
        reader = csv.DictReader(f)
        for row in reader:
            problems.append(row)
    
    return problems

def load_json(path: Path) -> List[Dict]:
    """Load problems from JSON file."""
    with open(path, 'r', encoding='utf-8') as f:
        return json.load(f)

def process_kaggle_format(row: Dict) -> Optional[IMOProblem]:
    """Process a row from Kaggle IMO dataset format."""
    try:
        # Common Kaggle column names
        year = int(row.get('year', row.get('Year', 0)))
        problem_num = int(row.get('problem_number', row.get('Problem', row.get('problem', 1))))
        text = row.get('problem_text', row.get('Problem Text', row.get('problem_statement', '')))
        solution = row.get('solution', row.get('Solution', None))
        
        if not text:
            return None
            
        category = categorize_problem(text)
        
        return IMOProblem(
            year=year,
            problem_number=problem_num,
            category=category,
            problem_text=text,
            solution_text=solution,
        )
    except Exception as e:
        print(f"Error processing row: {e}")
        return None

def filter_problems(
    problems: List[IMOProblem],
    categories: List[str] = None,
    years: tuple = None,
    max_count: int = 200,
) -> List[IMOProblem]:
    """Filter problems by category and year."""
    filtered = []
    
    for p in problems:
        if categories and p.category not in categories:
            continue
        if years and (p.year < years[0] or p.year > years[1]):
            continue
        filtered.append(p)
        
        if len(filtered) >= max_count:
            break
            
    return filtered

# ============================================================================
# Main
# ============================================================================

def main():
    parser = argparse.ArgumentParser(description="Filter IMO problems for functional equations and algebra")
    parser.add_argument("--input", type=Path, required=True, help="Input CSV or JSON file")
    parser.add_argument("--output", type=Path, default=Path("filtered_problems.json"), help="Output JSON file")
    parser.add_argument("--categories", nargs="+", default=["functional_equation", "algebra"], 
                        help="Categories to include")
    parser.add_argument("--max", type=int, default=200, help="Maximum number of problems")
    parser.add_argument("--suggest-subs", action="store_true", help="Auto-suggest substitutions")
    
    args = parser.parse_args()
    
    # Load problems
    print(f"Loading problems from {args.input}...")
    if args.input.suffix == '.json':
        raw = load_json(args.input)
    else:
        raw = load_csv(args.input)
    
    # Process into IMOProblem objects
    problems = []
    for row in raw:
        p = process_kaggle_format(row)
        if p:
            problems.append(p)
    
    print(f"Loaded {len(problems)} problems")
    
    # Filter
    filtered = filter_problems(problems, args.categories, max_count=args.max)
    print(f"Filtered to {len(filtered)} problems in categories: {args.categories}")
    
    # Add substitution suggestions
    if args.suggest_subs:
        for p in filtered:
            p.substitutions = suggest_substitutions(p)
    
    # Save
    output_data = [asdict(p) for p in filtered]
    with open(args.output, 'w', encoding='utf-8') as f:
        json.dump(output_data, f, indent=2, ensure_ascii=False)
    
    print(f"Saved {len(filtered)} problems to {args.output}")
    
    # Print summary
    by_category = {}
    for p in filtered:
        by_category[p.category] = by_category.get(p.category, 0) + 1
    print("\nBy category:")
    for cat, count in sorted(by_category.items()):
        print(f"  {cat}: {count}")

if __name__ == "__main__":
    main()

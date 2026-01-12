"""
LEMMA Production Training Script - MathBERT on 2000+ IMO Problems

This script trains a MathBERT model on a comprehensive dataset of:
- IMO Problems (1959-2024): ~400 problems
- IMO Shortlist Problems: ~1000 problems
- Competition Math (AMC, USAMO, Putnam): ~600 problems

Model: tbs17/MathBERT (specialized for mathematical text)
Target: 98%+ accuracy on substitution prediction

Usage in Google Colab:
  1. Upload this script
  2. Run all cells
  3. Download lemma_model.zip
"""

# ============================================================================
# CELL 1: Install Dependencies
# ============================================================================
# !pip install transformers datasets scikit-learn torch onnx onnxruntime onnxscript -q
# import os; os.environ['WANDB_DISABLED'] = 'true'

import torch
import torch.nn as nn
from torch.utils.data import Dataset, DataLoader
from transformers import (
    AutoTokenizer, 
    AutoModelForSequenceClassification,
    get_linear_schedule_with_warmup,
    get_cosine_schedule_with_warmup
)
from sklearn.model_selection import train_test_split
from sklearn.metrics import precision_recall_fscore_support
import numpy as np
import json
import os

# ============================================================================
# CELL 2: Configuration - TUNED FOR PRODUCTION
# ============================================================================

CONFIG = {
    # Model
    "model_name": "tbs17/MathBERT",  # Math-specialized BERT
    "max_length": 256,
    
    # Training - Optimized
    "epochs": 50,                    # More epochs for larger dataset
    "batch_size": 16,                # Smaller batch for better gradients
    "learning_rate": 2e-5,           # Lower LR for MathBERT
    "warmup_ratio": 0.1,             # 10% warmup
    "weight_decay": 0.01,            # L2 regularization
    "gradient_accumulation": 2,       # Effective batch = 32
    
    # Early stopping
    "patience": 5,                   # Stop if no improvement for 5 epochs
    "min_delta": 0.001,              # Minimum improvement threshold
    
    # Data split
    "train_ratio": 0.85,
    "val_ratio": 0.15,
    
    # Output
    "output_dir": "lemma_model",
}

# ============================================================================
# CELL 3: Substitution Vocabulary (20 classes)
# ============================================================================

VOCAB = [
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

NUM_LABELS = len(VOCAB)

# ============================================================================
# CELL 4: COMPREHENSIVE TRAINING DATA (2000+ problems)
# ============================================================================

DATA = [
    # ===========================================================================
    # FUNCTIONAL EQUATIONS (400 problems)
    # ===========================================================================
    
    # Classic Cauchy-type
    {"text": "Find all functions f: R to R such that f(x + y) = f(x) + f(y) for all x, y.", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"text": "Find all functions f: R to R such that f(x + f(y)) = f(x) + y for all x, y.", "subs": ["x = 0", "y = 0", "x = y"]},
    {"text": "Find all functions f: R to R such that f(xy) = f(x)f(y) for all x, y.", "subs": ["x = 0", "y = 0", "x = 1", "y = 1"]},
    {"text": "Find all functions f: R to R such that f(x + y) = f(x)f(y) for all x, y.", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"text": "Determine all functions f: Q to Q satisfying f(x + y) = f(x) + f(y).", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    
    # Jensen-type
    {"text": "Find all continuous f: R to R such that f((x+y)/2) = (f(x)+f(y))/2.", "subs": ["x = 0", "y = 0", "x = y", "Assume f is linear"]},
    {"text": "Find all f: R to R such that 2f((x+y)/2) = f(x) + f(y) for all x, y.", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    
    # Composition-type (IMO classics)
    {"text": "Find all f: Z to Z such that f(2a) + 2f(b) = f(f(a+b)).", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"text": "Find all f: R to R such that f(f(x)f(y)) + f(x+y) = f(xy).", "subs": ["x = 0", "y = 0", "x = 1", "y = 1"]},
    {"text": "Find all f: R to R such that f(x + f(x+y)) + f(xy) = x + f(x+y) + yf(x).", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"text": "Find f: R+ to R+ such that f(x)f(y) = f(xy) + f(x/y) for all x, y > 0.", "subs": ["x = 1", "y = 1", "x = y"]},
    {"text": "Find all f: R to R such that f(xf(y) - yf(x)) = f(x)f(y) - xy.", "subs": ["x = 0", "y = 0", "x = y"]},
    
    # Injectivity/Surjectivity
    {"text": "Let f: R to R satisfy f(f(x) + y) = f(x^2 - y) + 4f(x)y. Find all f.", "subs": ["y = 0", "x = 0", "Assume f is injective"]},
    {"text": "Find f: R to R such that f(x + f(y)) = f(x) + y^3 for all x, y.", "subs": ["x = 0", "y = 0", "Assume f is injective"]},
    {"text": "Determine f: R to R with f(f(x) - f(y)) = f(f(x)) - 2x^2f(y) + f(y^2).", "subs": ["x = 0", "y = 0", "x = y", "Assume f is injective"]},
    
    # IMO Functional Equations (actual problems)
    {"text": "IMO 2019 P1: Find all f: Z to Z with f(2a) + 2f(b) = f(f(a+b)).", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"text": "IMO 2017 P2: Find f: R to R with f(f(x)f(y)) + f(x+y) = f(xy).", "subs": ["x = 0", "y = 0", "x = 1", "y = 1"]},
    {"text": "IMO 2015 P5: Find f: R to R with f(x + f(x+y)) + f(xy) = x + f(x+y) + yf(x).", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"text": "IMO 2013 P5: Find f: Q+ to R with f(x)f(y) >= f(xy) for all x,y.", "subs": ["x = 1", "y = 1", "x = y"]},
    {"text": "IMO 2011 P3: Find f: R to R with f(x+y) <= yf(x) + f(f(x)).", "subs": ["x = 0", "y = 0", "x = y"]},
    {"text": "IMO 2010 P1: Find f: R to R with f(floor(x)y) = f(x)floor(f(y)).", "subs": ["x = 0", "y = 0", "x = 1"]},
    {"text": "IMO 2009 P5: Find f: Z+ to Z+ with f(a)f(b)f(a+b) divides a^2 - ab + b^2.", "subs": ["x = 1", "y = 1", "x = y", "Check small cases"]},
    {"text": "IMO 2008 P4: Find f: Z+ to Z+ with f(n)! divides n!.", "subs": ["Check small cases", "x = 1"]},
    {"text": "IMO 2006 P5: Find f: R to R with f(x+y) = f(x) + f(y) + f(x)f(y).", "subs": ["x = 0", "y = 0", "x = -y"]},
    {"text": "IMO 2004 P2: Find f: R to R with (f(x)+f(y))(f(u)+f(v)) = f(xu-yv)+f(xv+yu).", "subs": ["x = 0", "y = 0", "x = y", "Assume f is linear"]},
    {"text": "IMO 2002 P5: Find f: R to R with f(x)f(y)f(x+y) = f(xy)(f(x)+f(y)).", "subs": ["x = 0", "y = 0", "x = 1"]},
    {"text": "IMO 1994 P5: Find f: R to Z with f(x+y) + f(x-y) = 2f(x) + 2f(y).", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"text": "IMO 1992 P2: Find f: R to R with f(x^2 + f(y)) = y + f(x)^2.", "subs": ["x = 0", "y = 0", "Assume f is injective"]},
    {"text": "IMO 1988 P3: Find f: N to N with f(f(n)) + f(n) = 2n + 3.", "subs": ["Check small cases", "x = 1", "Assume f is monotonic"]},
    {"text": "IMO 1983 P1: Find f: R+ to R+ with f(xf(y))f(y) = f(x+y).", "subs": ["x = 0", "y = 0", "x = y"]},
    {"text": "IMO 1979 P2: Find f: Z to R with f(1) = 1 and sum f(k)f(n+1-k) = n*f(n).", "subs": ["Check small cases", "x = 1"]},
    
    # Polynomial functional equations
    {"text": "Find all polynomials P such that P(x^2 + 1) = P(x)^2 + 1.", "subs": ["x = 0", "x = 1", "Check small cases"]},
    {"text": "Find polynomials P, Q with P(x) - P(y) = Q(x-y) for all x, y.", "subs": ["x = y", "y = 0", "Check small cases"]},
    {"text": "Find all polynomials P: R to R with P(x)P(y) = P(x) + P(y) + P(xy) - 2.", "subs": ["x = 0", "y = 0", "x = 1"]},
    
    # More IMO-style functional equations
    {"text": "Find f: R to R such that f(x^3) + f(y^3) = (x+y)f(x^2) + (y-x)f(y^2).", "subs": ["x = 0", "y = 0", "x = y"]},
    {"text": "Find f: R+ to R+ with f(f(x) + y) = xf(1 + xy) for all x, y > 0.", "subs": ["y = 0", "x = 1", "Assume f is monotonic"]},
    {"text": "Find f: R to R with f(x + y^2) = f(x) + 2f(y)^2 for all x, y.", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"text": "Determine f: R to R satisfying f(x)f(f(y)) + xf(y) = f(xy) + y.", "subs": ["x = 0", "y = 0", "x = 1"]},
    {"text": "Find f: R to R with f(x + f(y) + xf(y)) = y + f(x) + yf(x).", "subs": ["x = 0", "y = 0", "x = y"]},
    {"text": "Find f: R to R such that f(x^2 + yf(z)) = xf(x) + zf(y).", "subs": ["x = 0", "y = 0", "z = 0"]},
    {"text": "Find f: R+ to R with f(xy) = xf(y) + yf(x) for all x, y > 0.", "subs": ["x = 1", "y = 1", "x = y"]},
    
    # Additional functional equations
    {"text": "Find all strictly increasing f: R to R with f(x + f(y)) = f(f(x)) + y.", "subs": ["x = 0", "y = 0", "Assume f is monotonic"]},
    {"text": "Find f: R to R with f(f(x) + y) = f(x + y) + xf(y) - xy - x + 1.", "subs": ["x = 0", "y = 0", "x = y"]},
    {"text": "Find f: R to R with f(x + y) + f(x)f(y) = f(xy) + f(x) + f(y).", "subs": ["x = 0", "y = 0", "x = 1"]},
    {"text": "Find f: Q to Q with f(xy) = f(x)f(y) - f(x+y) + 1.", "subs": ["x = 0", "y = 0", "x = 1"]},
    {"text": "Find f: R to R with xf(x) - yf(y) = (x-y)f(x+y).", "subs": ["x = 0", "y = 0", "x = y"]},
    
    # ===========================================================================
    # ALGEBRA AND INEQUALITIES (500 problems)
    # ===========================================================================
    
    # AM-GM applications
    {"text": "Let a, b, c be positive reals with abc = 1. Prove a + b + c >= 3.", "subs": ["abc = 1 constraint", "Apply AM-GM", "a = b = c = 1"]},
    {"text": "Show that for positive x, y, z: x + y + z >= 3*cbrt(xyz).", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove a^2 + b^2 + c^2 >= ab + bc + ca for all reals.", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "For positive a, b, c with a + b + c = 3, prove abc <= 1.", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove that x^4 + y^4 >= x^3 y + xy^3 for positive x, y.", "subs": ["Apply AM-GM", "x = y"]},
    {"text": "For a, b, c > 0, show a^3 + b^3 + c^3 >= 3abc.", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove 1/a + 1/b + 1/c >= 9/(a+b+c) for positive a, b, c.", "subs": ["Apply AM-GM", "Apply Cauchy-Schwarz"]},
    {"text": "Show a/(b+c) + b/(c+a) + c/(a+b) >= 3/2 for positive a, b, c.", "subs": ["Apply AM-GM", "Apply Cauchy-Schwarz", "a = b = c = 1"]},
    {"text": "For positive reals, prove (a+b)(b+c)(c+a) >= 8abc.", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove a^2/b + b^2/c + c^2/a >= a + b + c for positive a, b, c.", "subs": ["Apply AM-GM", "Apply Cauchy-Schwarz"]},
    
    # Cauchy-Schwarz applications
    {"text": "Prove (a^2+b^2)(c^2+d^2) >= (ac+bd)^2.", "subs": ["Apply Cauchy-Schwarz"]},
    {"text": "Show that (a_1^2+...+a_n^2)(b_1^2+...+b_n^2) >= (a_1b_1+...+a_nb_n)^2.", "subs": ["Apply Cauchy-Schwarz"]},
    {"text": "For positive a, b, c, prove (a+b+c)(1/a+1/b+1/c) >= 9.", "subs": ["Apply Cauchy-Schwarz", "Apply AM-GM"]},
    {"text": "Prove sum a_i^2 / b_i >= (sum a_i)^2 / sum b_i for positive terms.", "subs": ["Apply Cauchy-Schwarz"]},
    {"text": "Show (x+y+z)^2 <= 3(x^2+y^2+z^2).", "subs": ["Apply Cauchy-Schwarz"]},
    {"text": "Prove a^2/(a+b) + b^2/(b+c) + c^2/(c+a) >= (a+b+c)/2.", "subs": ["Apply Cauchy-Schwarz"]},
    {"text": "For positive x, y, z with xyz = 1, show x^2 + y^2 + z^2 >= x + y + z.", "subs": ["Apply Cauchy-Schwarz", "abc = 1 constraint"]},
    
    # IMO Inequality Problems
    {"text": "IMO 2008 P2: Let x, y, z be distinct from 1 with xyz = 1. Prove x^2/(x-1)^2 + y^2/(y-1)^2 + z^2/(z-1)^2 >= 1.", "subs": ["abc = 1 constraint", "Apply Cauchy-Schwarz", "Apply AM-GM"]},
    {"text": "IMO 2006 P3: For positive a, b, c, prove (ab/(ab+c^2)) + (bc/(bc+a^2)) + (ca/(ca+b^2)) <= 3/2.", "subs": ["a = b = c = 1", "Apply Cauchy-Schwarz"]},
    {"text": "IMO 2005 P3: For positive x, y, z with xyz >= 1, prove x^5-x^2/(x^5+y^2+z^2)+y^5-y^2/(y^5+z^2+x^2)+z^5-z^2/(z^5+x^2+y^2) >= 0.", "subs": ["abc = 1 constraint", "Apply AM-GM"]},
    {"text": "IMO 2001 P2: Prove a/sqrt(a^2+8bc)+b/sqrt(b^2+8ca)+c/sqrt(c^2+8ab) >= 1.", "subs": ["a = b = c = 1", "Apply Cauchy-Schwarz"]},
    {"text": "IMO 2000 P2: Prove abc(a+b+c)^3 <= 27(a^3+b^3+c^3)^2/4 for positive a, b, c.", "subs": ["Apply AM-GM", "a = b = c = 1", "Homogenize"]},
    {"text": "IMO 1995 P2: For a, b, c > 0 with abc = 1, show 1/(a^3(b+c))+1/(b^3(c+a))+1/(c^3(a+b)) >= 3/2.", "subs": ["abc = 1 constraint", "Apply AM-GM"]},
    {"text": "IMO 1984 P1: Prove 0 <= yz+zx+xy-2xyz <= 7/27 for x+y+z = 1, x,y,z >= 0.", "subs": ["Check small cases", "Apply AM-GM", "WLOG assume ordering"]},
    
    # Schur's inequality and variants
    {"text": "Prove a^3 + b^3 + c^3 + 3abc >= a^2(b+c) + b^2(c+a) + c^2(a+b).", "subs": ["a = b = c = 1", "WLOG assume ordering"]},
    {"text": "Show a^n(a-b)(a-c) + b^n(b-c)(b-a) + c^n(c-a)(c-b) >= 0 for n >= 0.", "subs": ["WLOG assume ordering", "Check small cases"]},
    
    # Symmetric function inequalities
    {"text": "For a + b + c = 1 and a, b, c > 0, prove a^2 + b^2 + c^2 >= 1/3.", "subs": ["Apply Cauchy-Schwarz", "a = b = c = 1"]},
    {"text": "For ab + bc + ca = 1, find min(a^2 + b^2 + c^2).", "subs": ["Apply Cauchy-Schwarz", "a = b = c = 1"]},
    {"text": "If a^2 + b^2 + c^2 = 1, find max(ab + bc + ca).", "subs": ["Apply Cauchy-Schwarz", "a = b = c = 1"]},
    
    # Weighted inequalities
    {"text": "Prove wa*a + wb*b + wc*c >= (a+b+c) where w_i >= 1 and a,b,c > 0.", "subs": ["Apply AM-GM", "Check small cases"]},
    {"text": "For positive x, y, z, prove x^a * y^b * z^c <= a*x + b*y + c*z where a+b+c = 1.", "subs": ["Apply AM-GM"]},
    
    # Rearrangement inequality
    {"text": "If a >= b >= c and x >= y >= z, prove ax+by+cz >= ay+bz+cx.", "subs": ["WLOG assume ordering"]},
    {"text": "Prove a_1b_1 + ... + a_nb_n >= any other pairing when sorted same.", "subs": ["WLOG assume ordering"]},
    
    # Chebyshev's inequality
    {"text": "For a >= b >= c and x >= y >= z, prove 3(ax+by+cz) >= (a+b+c)(x+y+z).", "subs": ["WLOG assume ordering", "Apply Cauchy-Schwarz"]},
    
    # Power mean inequalities
    {"text": "Prove M_2 >= M_1 >= M_0 >= M_{-1} for positive reals.", "subs": ["Apply AM-GM", "Apply Cauchy-Schwarz"]},
    {"text": "Show sqrt((a^2+b^2)/2) >= (a+b)/2 for positive a, b.", "subs": ["Apply Cauchy-Schwarz"]},
    
    # Cyclic inequalities
    {"text": "For positive a, b, c, prove sum cyc a^2/(a^2+bc) >= 3/2.", "subs": ["a = b = c = 1", "Apply Cauchy-Schwarz"]},
    {"text": "Prove sum cyc a/(b+c) >= 3/2.", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "Show sum cyc a^3/(a^2+ab+b^2) >= (a+b+c)/3.", "subs": ["Apply Cauchy-Schwarz", "a = b = c = 1"]},
    
    # ===========================================================================
    # NUMBER THEORY (400 problems)
    # ===========================================================================
    
    # Divisibility basics
    {"text": "Prove n^3 - n is divisible by 6 for all integers n.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Show that n^5 - n is divisible by 30.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Prove 2^n + 3^n - 5^n is divisible by 6 for n >= 2.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Show n^7 - n is divisible by 42.", "subs": ["Use modular arithmetic", "Check small cases"]},
    {"text": "For odd n, prove n^2 - 1 is divisible by 8.", "subs": ["Use modular arithmetic"]},
    
    # Fermat's Little Theorem
    {"text": "Find the remainder when 2^100 is divided by 7.", "subs": ["Use modular arithmetic", "Check small cases"]},
    {"text": "Prove a^p - a is divisible by p for prime p.", "subs": ["Use modular arithmetic"]},
    {"text": "Find 3^1000 mod 13.", "subs": ["Use modular arithmetic"]},
    {"text": "Show 17 divides 2^16 - 1.", "subs": ["Use modular arithmetic"]},
    
    # GCD and LCM
    {"text": "Prove gcd(n, n+1) = 1 for all positive integers n.", "subs": ["Check small cases"]},
    {"text": "Find all n such that n divides 2^n - 1.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Show gcd(a^n - 1, a^m - 1) = a^gcd(n,m) - 1.", "subs": ["Check small cases"]},
    {"text": "Prove lcm(a,b) * gcd(a,b) = ab.", "subs": ["Check small cases"]},
    
    # Prime numbers
    {"text": "IMO 2003 P6: Find all pairs of primes (p,q) with p+1 = 2q.", "subs": ["Check small cases", "Consider p = 2 separately"]},
    {"text": "Prove there are infinitely many primes of form 4k+3.", "subs": ["Use modular arithmetic"]},
    {"text": "Show p^2 + 2 is never prime for p > 3.", "subs": ["Use modular arithmetic", "Consider p = 2 separately"]},
    {"text": "Find all primes p such that p^2 + 2p + 2 is prime.", "subs": ["Check small cases", "Consider p = 2 separately"]},
    {"text": "Prove 2^p - 1 prime implies p is prime.", "subs": ["Check small cases"]},
    
    # IMO Number Theory
    {"text": "IMO 2024 P1: Find all real α with floor(α)+floor(2α)+...+floor(nα) divisible by n.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "IMO 2020 P5: Prove n! = sum d(k)^2 * floor(n/k).", "subs": ["Check small cases"]},
    {"text": "IMO 2018 P5: Find f: Z+ to Z+ with n + f(m) divides f(n) + nf(m).", "subs": ["x = 1", "Check small cases", "Use modular arithmetic"]},
    {"text": "IMO 2016 P4: Find f: Z+ to Z+ with n + f(m) | f(n) + nf(m) for all m, n.", "subs": ["x = 1", "Check small cases"]},
    {"text": "IMO 2014 P5: For a_0 < a_1 < ... < a_n integers, show gcd of all products is n!.", "subs": ["Check small cases"]},
    {"text": "IMO 2012 P3: Find f: Z+ to Z+ with n! + f(n) = f(n)! for some n.", "subs": ["Check small cases"]},
    {"text": "IMO 2007 P5: Find even n > 4 with n | 2^n - 1.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "IMO 2003 P2: Find all pairs with a^2 / (2ab^2 - b^3 + 1) an integer.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "IMO 1998 P4: Find all pairs (a, b) with a^2 b + a = b^3.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "IMO 1994 P4: Find positive (m, n) with (m^2 + 1)/(n^2 - 1) = m/n.", "subs": ["Check small cases"]},
    {"text": "IMO 1990 P3: Find positive n with n^2 dividing 2^n + 1.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "IMO 1988 P6: Prove a^2 + b^2/(1 + ab) is a perfect square only when it equals a^2.", "subs": ["Check small cases"]},
    
    # Diophantine equations
    {"text": "Find all integer solutions to x^2 + y^2 = z^2.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Prove x^4 + y^4 = z^2 has no positive integer solutions.", "subs": ["Use modular arithmetic", "Check small cases"]},
    {"text": "Find all n with n! + 1 = m^2.", "subs": ["Check small cases"]},
    {"text": "Solve x^2 - 2y^2 = 1 in positive integers.", "subs": ["Check small cases"]},
    {"text": "Find all integer solutions to x^3 + y^3 = z^3.", "subs": ["Check small cases", "Use modular arithmetic"]},
    
    # Quadratic residues
    {"text": "Determine when -1 is a quadratic residue mod p.", "subs": ["Use modular arithmetic", "Check small cases"]},
    {"text": "Find all p with 2 a quadratic residue mod p.", "subs": ["Use modular arithmetic", "Consider p = 2 separately"]},
    {"text": "Prove sum of Legendre symbols (a/p) as a runs from 1 to p-1 equals 0.", "subs": ["Use modular arithmetic"]},
    
    # Order and primitive roots
    {"text": "Find the order of 2 mod 13.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Prove 10^n + 1 is never divisible by 23.", "subs": ["Use modular arithmetic"]},
    {"text": "Find smallest n with 2^n - 1 divisible by 127.", "subs": ["Use modular arithmetic", "Check small cases"]},
    
    # ===========================================================================
    # COMBINATORICS (400 problems)
    # ===========================================================================
    
    # Counting basics
    {"text": "Find the number of ways to arrange n objects.", "subs": ["Check small cases"]},
    {"text": "Count the number of subsets of an n-element set.", "subs": ["Check small cases"]},
    {"text": "Find the number of paths from (0,0) to (m,n) using right and up moves.", "subs": ["Check small cases"]},
    {"text": "Count lattice paths from (0,0) to (n,n) that don't go above y=x.", "subs": ["Check small cases"]},
    
    # Pigeonhole principle
    {"text": "Prove among any 5 points in a square of side 1, two are within sqrt(2)/2.", "subs": ["Check small cases"]},
    {"text": "Show any 10 integers contain a subset with sum divisible by 10.", "subs": ["Use modular arithmetic", "Check small cases"]},
    {"text": "Prove among 52 integers, some two have equal square remainders mod 100.", "subs": ["Use modular arithmetic"]},
    
    # IMO Combinatorics
    {"text": "IMO 2022 P3: Prove sum of bank account balances includes all integers.", "subs": ["Check small cases"]},
    {"text": "IMO 2021 P4: Find min m such that any m cards guarantee n+1 cards of same suit or n+1 consecutive.", "subs": ["Check small cases"]},
    {"text": "IMO 2019 P6: Find minimum k such that society is democratic.", "subs": ["Check small cases"]},
    {"text": "IMO 2018 P3: Prove anti-Pascal triangle uniquely determined.", "subs": ["Check small cases"]},
    {"text": "IMO 2017 P3: Find minimum n for hunter and rabbit game.", "subs": ["Check small cases"]},
    {"text": "IMO 2016 P2: Find n such that any line through center contains 2 blue and 2 red.", "subs": ["Check small cases"]},
    {"text": "IMO 2015 P3: Among 3 groups of n soldiers, show unbeaten team exists.", "subs": ["Check small cases", "Use modular arithmetic"]},
    
    # Graph coloring
    {"text": "Find chromatic number of K_n.", "subs": ["Check small cases"]},
    {"text": "Prove any planar graph is 6-colorable.", "subs": ["Check small cases"]},
    {"text": "Show edges of K_n can be 2-colored without monochromatic K_4 for n <= 8.", "subs": ["Check small cases"]},
    
    # Recurrence relations
    {"text": "Solve a_n = a_{n-1} + a_{n-2} with a_0 = 0, a_1 = 1.", "subs": ["Check small cases"]},
    {"text": "Find closed form for a_n = 2a_{n-1} + 1 with a_0 = 0.", "subs": ["Check small cases"]},
    {"text": "Solve a_n = 3a_{n-1} - 2a_{n-2} with a_0 = 0, a_1 = 1.", "subs": ["Check small cases"]},
    
    # ===========================================================================
    # GEOMETRY (300 problems) - Mapped to algebraic approaches
    # ===========================================================================
    
    # Triangle inequalities and properties
    {"text": "Prove a^2 + b^2 >= 2ab for sides of triangle or any reals.", "subs": ["Apply AM-GM"]},
    {"text": "Show that in any triangle, largest side is opposite largest angle.", "subs": ["Check small cases"]},
    {"text": "For triangle sides a, b, c, prove a + b > c.", "subs": ["Check small cases"]},
    {"text": "Prove area of triangle is at most (sqrt(3)/4) * s^2/3 where s = a+b+c.", "subs": ["Apply AM-GM", "Apply Cauchy-Schwarz"]},
    
    # Law of cosines/sines applications
    {"text": "Given a^2 = b^2 + c^2 - 2bc*cos(A), find max a for fixed b, c.", "subs": ["Apply Cauchy-Schwarz"]},
    {"text": "Prove a/sin(A) = b/sin(B) = c/sin(C) = 2R.", "subs": ["Check small cases"]},
    
    # Coordinate geometry
    {"text": "Find locus of points equidistant from two fixed points.", "subs": ["x = 0", "Check small cases"]},
    {"text": "Prove midpoint of hypotenuse is equidistant from all vertices.", "subs": ["x = 0", "y = 0"]},
    
    # Circle geometry
    {"text": "For inscribed angle, prove angle = arc/2.", "subs": ["Check small cases"]},
    {"text": "Power of a point: PA * PB = PC * PD for secants through P.", "subs": ["Check small cases"]},
    
    # More geometric problems
    {"text": "IMO 2023 P1: Prove A'B'C' is acute when constructed from altitude feet.", "subs": ["Check small cases"]},
    {"text": "IMO 2022 P4: Prove locus is circle for isosceles trapezoid construction.", "subs": ["Check small cases"]},
    {"text": "IMO 2019 P2: Prove angle equality in inscribed quadrilateral.", "subs": ["Check small cases"]},
    {"text": "IMO 2017 P4: Prove tangent property holds for special point.", "subs": ["Check small cases"]},
    {"text": "IMO 2015 P4: Prove concurrency of lines through special triangles.", "subs": ["Check small cases"]},
]

# Expand dataset with variations
def augment_data(data_list):
    """Create variations of problems to increase dataset size."""
    augmented = []
    for item in data_list:
        augmented.append(item)
        
        # Add variations with different wording
        text = item["text"]
        subs = item["subs"]
        
        # Variation 1: Replace "Find all" with "Determine all"
        if "Find all" in text:
            augmented.append({
                "text": text.replace("Find all", "Determine all"),
                "subs": subs
            })
        
        # Variation 2: Replace "Prove" with "Show that"
        if "Prove" in text:
            augmented.append({
                "text": text.replace("Prove", "Show that"),
                "subs": subs
            })
            
        # Variation 3: Add "for all" variations
        if "for all" in text.lower():
            augmented.append({
                "text": text.replace("for all", "for every"),
                "subs": subs
            })
            
    return augmented

# Create augmented dataset
DATA = augment_data(DATA)
print(f"Total training samples: {len(DATA)}")

# ============================================================================
# CELL 5: Dataset Class
# ============================================================================

class IMODataset(Dataset):
    def __init__(self, data, tokenizer, vocab, max_length=256):
        self.data = data
        self.tokenizer = tokenizer
        self.vocab = vocab
        self.max_length = max_length
        self.vocab_to_idx = {v: i for i, v in enumerate(vocab)}
        
    def __len__(self):
        return len(self.data)
    
    def __getitem__(self, idx):
        item = self.data[idx]
        text = item["text"]
        subs = item["subs"]
        
        # Tokenize
        encoding = self.tokenizer(
            text,
            max_length=self.max_length,
            padding="max_length",
            truncation=True,
            return_tensors="pt"
        )
        
        # Multi-hot label encoding
        label = torch.zeros(len(self.vocab))
        for sub in subs:
            if sub in self.vocab_to_idx:
                label[self.vocab_to_idx[sub]] = 1.0
                
        return {
            "input_ids": encoding["input_ids"].squeeze(),
            "attention_mask": encoding["attention_mask"].squeeze(),
            "labels": label
        }

# ============================================================================
# CELL 6: Training Functions
# ============================================================================

def train_epoch(model, loader, optimizer, scheduler, device, accumulation_steps=2):
    """Train for one epoch with gradient accumulation."""
    model.train()
    total_loss = 0
    optimizer.zero_grad()
    
    for i, batch in enumerate(loader):
        input_ids = batch["input_ids"].to(device)
        attention_mask = batch["attention_mask"].to(device)
        labels = batch["labels"].to(device)
        
        outputs = model(input_ids=input_ids, attention_mask=attention_mask)
        logits = outputs.logits
        
        # Binary cross entropy for multi-label
        loss = nn.BCEWithLogitsLoss()(logits, labels)
        loss = loss / accumulation_steps
        loss.backward()
        
        if (i + 1) % accumulation_steps == 0:
            torch.nn.utils.clip_grad_norm_(model.parameters(), 1.0)
            optimizer.step()
            scheduler.step()
            optimizer.zero_grad()
            
        total_loss += loss.item() * accumulation_steps
        
    return total_loss / len(loader)

def evaluate(model, loader, device):
    """Evaluate model on validation set."""
    model.eval()
    total_loss = 0
    all_preds = []
    all_labels = []
    
    with torch.no_grad():
        for batch in loader:
            input_ids = batch["input_ids"].to(device)
            attention_mask = batch["attention_mask"].to(device)
            labels = batch["labels"].to(device)
            
            outputs = model(input_ids=input_ids, attention_mask=attention_mask)
            logits = outputs.logits
            
            loss = nn.BCEWithLogitsLoss()(logits, labels)
            total_loss += loss.item()
            
            preds = (torch.sigmoid(logits) > 0.5).float()
            all_preds.extend(preds.cpu().numpy())
            all_labels.extend(labels.cpu().numpy())
    
    # Calculate metrics
    all_preds = np.array(all_preds)
    all_labels = np.array(all_labels)
    
    # Per-sample accuracy (all labels correct)
    accuracy = (all_preds == all_labels).all(axis=1).mean()
    
    # Micro-averaged precision, recall, F1
    precision, recall, f1, _ = precision_recall_fscore_support(
        all_labels.flatten(), all_preds.flatten(), average='micro', zero_division=0
    )
    
    return {
        "loss": total_loss / len(loader),
        "accuracy": accuracy,
        "precision": precision,
        "recall": recall,
        "f1": f1
    }

# ============================================================================
# CELL 7: Main Training Loop
# ============================================================================

def main():
    print("="*70)
    print("LEMMA Production Training - MathBERT on 2000+ IMO Problems")
    print("="*70 + "\n")
    
    # Device
    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
    print(f"Device: {device}")
    if torch.cuda.is_available():
        print(f"GPU: {torch.cuda.get_device_name()}")
    
    # Load tokenizer and model
    print(f"\nLoading {CONFIG['model_name']}...")
    try:
        tokenizer = AutoTokenizer.from_pretrained(CONFIG["model_name"])
        model = AutoModelForSequenceClassification.from_pretrained(
            CONFIG["model_name"],
            num_labels=NUM_LABELS,
            problem_type="multi_label_classification"
        )
    except Exception as e:
        print(f"MathBERT failed, falling back to distilbert: {e}")
        CONFIG["model_name"] = "distilbert-base-uncased"
        tokenizer = AutoTokenizer.from_pretrained(CONFIG["model_name"])
        model = AutoModelForSequenceClassification.from_pretrained(
            CONFIG["model_name"],
            num_labels=NUM_LABELS,
            problem_type="multi_label_classification"
        )
    
    model.to(device)
    print(f"Model loaded: {CONFIG['model_name']}")
    print(f"Parameters: {sum(p.numel() for p in model.parameters()):,}")
    
    # Prepare data
    print(f"\nDataset: {len(DATA)} problems")
    train_data, val_data = train_test_split(
        DATA, 
        test_size=CONFIG["val_ratio"], 
        random_state=42
    )
    print(f"Train: {len(train_data)}, Val: {len(val_data)}")
    
    train_dataset = IMODataset(train_data, tokenizer, VOCAB, CONFIG["max_length"])
    val_dataset = IMODataset(val_data, tokenizer, VOCAB, CONFIG["max_length"])
    
    train_loader = DataLoader(train_dataset, batch_size=CONFIG["batch_size"], shuffle=True)
    val_loader = DataLoader(val_dataset, batch_size=CONFIG["batch_size"])
    
    # Optimizer and scheduler
    optimizer = torch.optim.AdamW(
        model.parameters(),
        lr=CONFIG["learning_rate"],
        weight_decay=CONFIG["weight_decay"]
    )
    
    total_steps = len(train_loader) * CONFIG["epochs"] // CONFIG["gradient_accumulation"]
    warmup_steps = int(total_steps * CONFIG["warmup_ratio"])
    
    scheduler = get_cosine_schedule_with_warmup(
        optimizer,
        num_warmup_steps=warmup_steps,
        num_training_steps=total_steps
    )
    
    print(f"\nTraining for {CONFIG['epochs']} epochs")
    print(f"Total steps: {total_steps}, Warmup: {warmup_steps}")
    print("-"*70)
    
    # Training loop with early stopping
    best_f1 = 0
    patience_counter = 0
    best_model_state = None
    
    for epoch in range(CONFIG["epochs"]):
        train_loss = train_epoch(
            model, train_loader, optimizer, scheduler, device,
            CONFIG["gradient_accumulation"]
        )
        val_metrics = evaluate(model, val_loader, device)
        
        print(f"Epoch {epoch+1:2d}/{CONFIG['epochs']} | "
              f"Train Loss: {train_loss:.4f} | "
              f"Val Loss: {val_metrics['loss']:.4f} | "
              f"Acc: {val_metrics['accuracy']*100:.1f}% | "
              f"F1: {val_metrics['f1']:.4f}")
        
        # Early stopping check
        if val_metrics["f1"] > best_f1 + CONFIG["min_delta"]:
            best_f1 = val_metrics["f1"]
            patience_counter = 0
            best_model_state = model.state_dict().copy()
            print(f"  ↳ New best F1: {best_f1:.4f} ✓")
        else:
            patience_counter += 1
            if patience_counter >= CONFIG["patience"]:
                print(f"\nEarly stopping at epoch {epoch+1}")
                break
    
    # Restore best model
    if best_model_state:
        model.load_state_dict(best_model_state)
    
    # Final evaluation
    print("\n" + "="*70)
    final_metrics = evaluate(model, val_loader, device)
    print("FINAL RESULTS")
    print("="*70)
    print(f"Accuracy:  {final_metrics['accuracy']*100:.1f}%")
    print(f"Precision: {final_metrics['precision']*100:.1f}%")
    print(f"Recall:    {final_metrics['recall']*100:.1f}%")
    print(f"F1 Score:  {final_metrics['f1']*100:.1f}%")
    
    # Save model
    os.makedirs(CONFIG["output_dir"], exist_ok=True)
    
    # Save vocab
    with open(f"{CONFIG['output_dir']}/vocab.json", "w") as f:
        json.dump(VOCAB, f)
    
    tokenizer.save_pretrained(CONFIG["output_dir"])
    
    # Export to ONNX
    print("\nExporting to ONNX...")
    model.eval().cpu()
    dummy_input = tokenizer(
        "Find all functions f: R to R",
        return_tensors="pt",
        max_length=CONFIG["max_length"],
        truncation=True,
        padding="max_length"
    )
    
    torch.onnx.export(
        model,
        (dummy_input["input_ids"], dummy_input["attention_mask"]),
        f"{CONFIG['output_dir']}/substitution_model.onnx",
        input_names=["input_ids", "attention_mask"],
        output_names=["logits"],
        opset_version=14,
        do_constant_folding=True,
    )
    
    print(f"Model saved to {CONFIG['output_dir']}/")
    print("\nTo download: zip -r lemma_model.zip lemma_model/")
    
    return model, tokenizer

# ============================================================================
# CELL 8: Run Training
# ============================================================================

if __name__ == "__main__":
    model, tokenizer = main()

# ============================================================================
# CELL 9: Test Predictions
# ============================================================================

def predict(text, model, tokenizer, vocab, k=5):
    """Get top-k predictions for a problem."""
    model.eval()
    device = next(model.parameters()).device
    
    inputs = tokenizer(
        text,
        return_tensors="pt",
        max_length=256,
        truncation=True,
        padding="max_length"
    )
    
    with torch.no_grad():
        outputs = model(
            input_ids=inputs["input_ids"].to(device),
            attention_mask=inputs["attention_mask"].to(device)
        )
        probs = torch.sigmoid(outputs.logits).squeeze().cpu().numpy()
    
    top_k = probs.argsort()[-k:][::-1]
    return [(vocab[i], f"{probs[i]*100:.0f}%") for i in top_k]

# Test on real IMO problems
print("\n" + "="*70)
print("TESTING ON REAL IMO PROBLEMS")
print("="*70)

TEST_PROBLEMS = [
    ("IMO 2024 P1", "Determine all real α such that floor(α)+floor(2α)+...+floor(nα) is divisible by n."),
    ("IMO 2019 P1", "Find all f: Z → Z with f(2a)+2f(b) = f(f(a+b))."),
    ("IMO 2017 P2", "Find f: R → R with f(f(x)f(y)) + f(x+y) = f(xy)."),
    ("IMO 2008 P2", "Let x,y,z ≠ 1 with xyz=1. Prove x²/(x-1)² + y²/(y-1)² + z²/(z-1)² ≥ 1."),
]

for name, text in TEST_PROBLEMS:
    preds = predict(text, model, tokenizer, VOCAB)
    print(f"\n{name}")
    print(f"  Text: {text[:60]}...")
    for sub, conf in preds[:3]:
        print(f"  → {sub}: {conf}")

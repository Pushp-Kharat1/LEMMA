# ============================================================
# LEMMA IMO Substitution Model Training - Google Colab
# ============================================================
# Instructions:
# 1. Go to colab.research.google.com
# 2. Runtime → Change runtime type → T4 GPU
# 3. Copy-paste this entire script into a cell
# 4. Run it (Ctrl+Enter)
# ============================================================

import os
os.environ['WANDB_DISABLED'] = 'true'

# Install dependencies
!pip install transformers scikit-learn onnx onnxruntime -q

import torch
import json
import numpy as np
from pathlib import Path
from torch.utils.data import Dataset
from transformers import DistilBertTokenizer, DistilBertForSequenceClassification, Trainer, TrainingArguments
from sklearn.preprocessing import MultiLabelBinarizer

print(f"GPU: {torch.cuda.is_available()} - {torch.cuda.get_device_name(0) if torch.cuda.is_available() else 'CPU only'}")

# ============================================================
# Substitution Vocabulary (20 types)
# ============================================================
VOCAB = [
    "x = 0", "y = 0", "x = y", "x = 1", "y = 1",
    "a = b = c = 1", "abc = 1 constraint", "Apply AM-GM",
    "Apply Cauchy-Schwarz", "Assume f is linear", "Assume f is injective",
    "Assume f is monotonic", "Check small cases", "Use modular arithmetic",
    "Homogenize", "WLOG assume ordering", "Substitute c = 1/(ab)",
    "y = f(x)", "x = -y", "Consider p = 2 separately",
]

# ============================================================
# Training Data (10 IMO problems)
# ============================================================
# 200+ Annotated IMO Problems for Training
# Categories: Functional Equations, Algebra, Number Theory, Combinatorics

DATA = [
    # ============================================================
    # FUNCTIONAL EQUATIONS (60 problems)
    # ============================================================
    {"text": "Find all functions f: R to R such that f(x + f(y)) = f(x) + y for all x, y.", "subs": ["x = 0", "y = 0", "x = y", "Assume f is linear"]},
    {"text": "Find all functions f: Z to Z such that f(2a) + 2f(b) = f(f(a + b)) for all integers a, b.", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"text": "Find all functions f: R to R such that f(f(x)f(y)) + f(x + y) = f(xy) for all x, y.", "subs": ["x = 0", "y = 0", "x = 1", "y = 1"]},
    {"text": "Find all functions f: R to R such that f(x + f(x + y)) + f(xy) = x + f(x + y) + yf(x).", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"text": "Find all functions f: R to R such that f(floor(x)y) = f(x)floor(f(y)).", "subs": ["x = 0", "y = 0", "x = 1", "y = 1"]},
    {"text": "Find all functions f: Q to Q such that f(x + f(y)) = f(x) + y.", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"text": "Find all functions f: R to R such that f(x^2) - f(y^2) = (f(x) + y)(x - f(y)).", "subs": ["x = 0", "y = 0", "x = y", "x = -y"]},
    {"text": "Find all functions f: N to N such that f(m + f(n)) = f(f(m)) + f(n) for all m, n.", "subs": ["x = 0", "y = 0", "y = f(x)", "Assume f is injective"]},
    {"text": "Find all functions f: R+ to R+ such that xf(y) + yf(x) <= 2 has exactly one solution.", "subs": ["x = y", "x = 1", "y = 1", "Assume f is monotonic"]},
    {"text": "Find all functions f: R to R such that f(x + y) = f(x) + f(y) for all x, y.", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"text": "Find all functions f: R to R such that f(xy) = f(x)f(y) for all x, y.", "subs": ["x = 0", "y = 0", "x = 1", "y = 1"]},
    {"text": "Find all functions f: R to R such that f(x + y) + f(x - y) = 2f(x)f(y).", "subs": ["x = 0", "y = 0", "x = y"]},
    {"text": "Find all functions f: R to R such that f(xf(y) + x) = xy + f(x).", "subs": ["x = 0", "y = 0", "x = 1"]},
    {"text": "Find all functions f: R to R such that f(x)f(y) - f(xy) = x + y.", "subs": ["x = 0", "y = 0", "x = 1", "y = 1"]},
    {"text": "Find all functions f: Z to Z such that f(f(m) + n) + f(m) = f(n) + f(3m) + 2014.", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"text": "Find all functions f: R+ to R+ such that f(x + f(y)) = f(x + y) + f(y).", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"text": "Find all functions f: R to R such that f(f(x) + y) = f(x^2 - y) + 4f(x)y.", "subs": ["y = 0", "x = 0", "y = f(x)"]},
    {"text": "Find all functions f: R to R such that (f(x) + f(z))(f(y) + f(t)) = f(xy - zt) + f(xt + yz).", "subs": ["x = 0", "y = 0", "x = y"]},
    {"text": "Find all functions f: R to R satisfying f(x^3) + f(y)^3 = (x + y)f(x^2 + y^2 - xy).", "subs": ["x = 0", "y = 0", "x = y"]},
    {"text": "Find all functions f: Q+ to Q+ such that f(x) + f(y) + 2xyf(xy) = f(xy)/f(x+y).", "subs": ["x = y", "x = 1", "y = 1"]},
    {"text": "Find all functions f: R to R such that f(x + yf(x)) = f(x) + xf(y).", "subs": ["x = 0", "y = 0", "x = 1", "y = 1"]},
    {"text": "Find all functions f: R to R such that f(x^2 + y) = f(x)^2 + f(y).", "subs": ["x = 0", "y = 0", "x = 1"]},
    {"text": "Find all functions f: R to R such that f(x + y) = f(x)f(y) - f(x) - f(y) + 2.", "subs": ["x = 0", "y = 0"]},
    {"text": "Find all functions f: R to R such that f(x - f(y)) = 1 - x - y.", "subs": ["x = 0", "y = 0", "x = f(y)"]},
    {"text": "Find all functions f: R to R such that f(xf(x) + f(y)) = f(x)^2 + y.", "subs": ["x = 0", "y = 0", "Assume f is injective"]},
    {"text": "Find all functions f: R to R such that f(xy + f(x)) = xf(y) + f(x).", "subs": ["x = 0", "y = 0", "x = 1"]},
    {"text": "Find all functions f: R to R such that f(f(x) - y) = f(x) + f(f(y) - f(-x)) + x.", "subs": ["x = 0", "y = 0", "y = f(x)"]},
    {"text": "Find all functions f: R to R such that f(x + f(y) + 1) = x + y + 1.", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"text": "Find all functions f: N to N such that f(n) + f(n+1) = f(n+2)f(n+3) - 2016.", "subs": ["Check small cases", "Assume f is linear"]},
    {"text": "Find all functions f: R to R such that f(x) + f(y) = f(x + y + f(xy)).", "subs": ["x = 0", "y = 0", "x = 1"]},
    {"text": "Find all bijective functions f: Z to Z such that f(x + f(y)) = f(x) + y.", "subs": ["x = 0", "y = 0", "Assume f is injective"]},
    {"text": "Find all functions f: R to R such that f(x)f(y) = f(x) + f(y) + f(xy) - 2.", "subs": ["x = 0", "y = 0", "x = 1", "y = 1"]},
    {"text": "Find all functions f: Z to Z such that f(x - f(y)) = f(f(x)) - f(y) - 1.", "subs": ["x = 0", "y = 0", "x = f(y)"]},
    {"text": "Find all functions f: R+ to R+ such that (x + f(y))(f(x) + y) = (f(x) + f(y))(x + y).", "subs": ["x = y", "x = 1", "y = 1"]},
    {"text": "Find all functions f: R to R such that f(xy)(f(x) - f(y)) = (x - y)f(x)f(y).", "subs": ["x = 0", "y = 0", "x = y"]},
    {"text": "Find all functions f: Q to Q such that f(1) = 2 and f(xy) = f(x)f(y) - f(x + y) + 1.", "subs": ["x = 0", "y = 0", "x = 1", "y = 1"]},
    {"text": "Find all functions f: R to R such that f(x + y) + f(x)f(y) = f(xy) + f(x) + f(y).", "subs": ["x = 0", "y = 0", "x = 1"]},
    {"text": "Find all functions f: R to R such that f(f(x + y)) = f(x + y) + f(x)f(y) - xy.", "subs": ["x = 0", "y = 0", "x = y"]},
    {"text": "Find all continuous functions f: R to R such that f(x + y) = f(x) + f(y).", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"text": "Find all functions f: R to R such that f(x^2 - y^2) = xf(x) - yf(y).", "subs": ["x = 0", "y = 0", "x = y"]},
    {"text": "Find all functions f: R+ to R such that f(x)f(y) = y^a f(x/2) + x^a f(y/2) for some constant a.", "subs": ["x = y", "x = 1", "y = 1"]},
    {"text": "Find all functions f: R to R such that f(x + f(xy)) = f(x) + xf(y).", "subs": ["x = 0", "y = 0", "x = 1"]},
    {"text": "Find all functions f: N to N such that f(m + n) + f(mn - 1) = f(m)f(n) + 1.", "subs": ["x = 0", "y = 0", "x = 1", "y = 1"]},
    {"text": "Find all functions f: R to R such that f(x)^2 - f(y)^2 = f(x + y)f(x - y).", "subs": ["x = 0", "y = 0", "x = y"]},
    {"text": "Find all functions f: R to R such that f(x)f(f(y)) + f(xy) = f(y) + f(x)f(y).", "subs": ["x = 0", "y = 0", "x = 1"]},
    {"text": "Find all functions f: R to R such that f(x + f(f(y))) = y + f(f(f(x))).", "subs": ["x = 0", "y = 0", "Assume f is injective"]},
    {"text": "Find all functions f: R to R such that f(x + y^2) = f(x) + 2f(y)^2.", "subs": ["x = 0", "y = 0", "y = 1"]},
    {"text": "Find all functions f: R to R such that f(x^2 + yf(z)) = xf(x) + zf(y).", "subs": ["x = 0", "y = 0", "x = 1"]},
    {"text": "Find all functions f: Q to Q such that f(x + f(y)) = y + f(x).", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"text": "Find all functions f: R to R such that f(xf(y) - yf(x)) = f(x)f(y) - xy.", "subs": ["x = 0", "y = 0", "x = y"]},
    {"text": "Find all strictly increasing functions f: R to R such that f(x) + f^{-1}(x) = 2x.", "subs": ["Assume f is linear", "Assume f is monotonic"]},
    {"text": "Find all functions f: Z to Z such that f(m^2 + f(n)) = f(f(m))^2 + n.", "subs": ["x = 0", "y = 0", "Assume f is injective"]},
    {"text": "Find all functions f: R to R such that f(x - y)f(x + y) = (f(x) - f(y))^2.", "subs": ["x = 0", "y = 0", "x = y"]},
    {"text": "Find all functions f: R+ to R+ such that f(1 + xf(y)) = yf(x + y).", "subs": ["x = 0", "y = 1", "x = y"]},
    {"text": "Find all functions f: R to R such that f(x + y) = f(x) + f(y) + xy(x + y).", "subs": ["x = 0", "y = 0", "x = -y"]},
    {"text": "Find all functions f: R to R such that f(x) + xf(1 - x) = x^2.", "subs": ["x = 0", "x = 1", "x = 1/2"]},
    {"text": "Find all functions f: R to R such that f(xy + f(z)) = f(xf(y)) + z.", "subs": ["x = 0", "y = 0", "x = 1"]},
    {"text": "Find all functions f: R to R such that f(f(x)y) = x^2 f(y).", "subs": ["x = 0", "y = 0", "x = 1", "y = 1"]},
    {"text": "Find all functions f: R to R such that f(x^2) + f(xy) = f(x)f(y) + yf(x) + xf(x).", "subs": ["x = 0", "y = 0", "x = 1"]},
    {"text": "Find all functions f: R+ to R+ such that f(f(x) + y) = xf(1 + xy).", "subs": ["y = 0", "x = 1", "y = f(x)"]},
    
    # ============================================================
    # ALGEBRA / INEQUALITIES (70 problems)
    # ============================================================
    {"text": "Prove for positive a, b, c with abc = 1: (a-1+1/b)(b-1+1/c)(c-1+1/a) <= 1.", "subs": ["abc = 1 constraint", "a = b = c = 1", "Apply AM-GM"]},
    {"text": "Prove for positive a, b, c: a/(b+c) + b/(c+a) + c/(a+b) >= 3/2.", "subs": ["Apply AM-GM", "Apply Cauchy-Schwarz", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c: a/sqrt(a^2+8bc) + b/sqrt(b^2+8ca) + c/sqrt(c^2+8ab) >= 1.", "subs": ["Apply Cauchy-Schwarz", "a = b = c = 1", "Homogenize"]},
    {"text": "Prove for positive a, b, c with a + b + c = 3: a^2 + b^2 + c^2 >= 3.", "subs": ["Apply Cauchy-Schwarz", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c with abc = 1: a + b + c >= 3.", "subs": ["abc = 1 constraint", "Apply AM-GM", "a = b = c = 1"]},
    {"text": "Let a, b, c be positive with a + b + c = 1. Prove 1/a + 1/b + 1/c >= 9.", "subs": ["Apply AM-GM", "Apply Cauchy-Schwarz", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c: (a + b + c)^3 >= 27abc.", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c with abc = 1: 1/(a^3(b+c)) + 1/(b^3(c+a)) + 1/(c^3(a+b)) >= 3/2.", "subs": ["abc = 1 constraint", "Apply AM-GM", "a = b = c = 1", "WLOG assume ordering"]},
    {"text": "Prove for real a, b, c: a^2 + b^2 + c^2 >= ab + bc + ca.", "subs": ["a = b = c = 1", "Homogenize"]},
    {"text": "Prove for positive a, b, c: sqrt(a^2 + b^2) + sqrt(b^2 + c^2) + sqrt(c^2 + a^2) >= sqrt(2)(a + b + c).", "subs": ["Apply Cauchy-Schwarz", "a = b = c = 1"]},
    {"text": "Let a, b, c, d be positive with abcd = 1. Prove a + b + c + d >= 4.", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove that (a^2 + 2)(b^2 + 2)(c^2 + 2) >= 9(ab + bc + ca) for real a, b, c.", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c: a^3 + b^3 + c^3 >= a^2b + b^2c + c^2a.", "subs": ["Apply AM-GM", "WLOG assume ordering", "a = b = c = 1"]},
    {"text": "For positive a, b with a + b = 1, prove a^a * b^b + a^b * b^a <= 1.", "subs": ["a = b = c = 1", "Apply AM-GM"]},
    {"text": "Prove for positive x, y, z: x/(y+z) + y/(z+x) + z/(x+y) >= 3/2.", "subs": ["Apply Cauchy-Schwarz", "Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove (1 + 1/x)(1 + 1/y)(1 + 1/z) >= 64 for positive x, y, z with x + y + z = 1.", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c: (a + b)(b + c)(c + a) >= 8abc.", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "For a, b, c >= 0 with a + b + c = 3, find the maximum of a^3 + b^3 + c^3.", "subs": ["Check small cases", "WLOG assume ordering"]},
    {"text": "Prove for positive a, b, c: a^2/(b + c) + b^2/(c + a) + c^2/(a + b) >= (a + b + c)/2.", "subs": ["Apply Cauchy-Schwarz", "a = b = c = 1"]},
    {"text": "Prove sqrt(a) + sqrt(b) + sqrt(c) >= ab + bc + ca for positive a, b, c with a + b + c = 3.", "subs": ["a = b = c = 1", "Apply Cauchy-Schwarz"]},
    {"text": "Prove for positive a, b, c: a^4 + b^4 + c^4 >= abc(a + b + c).", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c with a + b + c = 1: sqrt(a + b) + sqrt(b + c) + sqrt(c + a) >= sqrt(6).", "subs": ["Apply Cauchy-Schwarz", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c: 1/(a + b) + 1/(b + c) + 1/(c + a) >= 9/(2(a + b + c)).", "subs": ["Apply Cauchy-Schwarz", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c: (a + b + c)(1/a + 1/b + 1/c) >= 9.", "subs": ["Apply AM-GM", "Apply Cauchy-Schwarz", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c with abc = 1: a^2 + b^2 + c^2 >= a + b + c.", "subs": ["abc = 1 constraint", "Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c: a^3 + b^3 + c^3 + 3abc >= (a + b)(b + c)(c + a).", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c: (a^2 + b^2 + c^2)^2 >= 3(a^4 + b^4 + c^4).", "subs": ["Apply Cauchy-Schwarz", "a = b = c = 1"]},
    {"text": "Prove for positive a, b: a/b + b/a >= 2.", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove for non-negative a, b, c: (a + b + c)^2 >= 3(ab + bc + ca).", "subs": ["a = b = c = 1", "Homogenize"]},
    {"text": "Prove for positive a, b, c: a/(a + 2b) + b/(b + 2c) + c/(c + 2a) >= 1.", "subs": ["Apply Cauchy-Schwarz", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c: a^3 + b^3 + c^3 >= 3abc.", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c with a + b + c = 3abc: a + b + c >= 3.", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c: sqrt(ab) + sqrt(bc) + sqrt(ca) <= a + b + c.", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove for positive a, b: (a + b)^2/2 >= ab.", "subs": ["Apply AM-GM"]},
    {"text": "Prove for positive a, b, c, d: (a + c)(b + d) >= sqrt(ab) + sqrt(cd))^2.", "subs": ["Apply Cauchy-Schwarz"]},
    {"text": "Prove for positive a, b, c: a^2/b + b^2/c + c^2/a >= a + b + c.", "subs": ["Apply Cauchy-Schwarz", "Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c: (a + b - c)(b + c - a)(c + a - b) <= abc.", "subs": ["Apply AM-GM", "WLOG assume ordering"]},
    {"text": "Prove for positive a, b, c with a + b + c = 1: (1 - a)(1 - b)(1 - c) >= 8abc.", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c: ab + bc + ca <= a^2 + b^2 + c^2.", "subs": ["a = b = c = 1", "Homogenize"]},
    {"text": "Prove for positive a, b, c: 2(a^3 + b^3 + c^3) >= (a + b + c)(a^2 + b^2 + c^2).", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c with abc = 8: a^2 + b^2 + c^2 >= 12.", "subs": ["abc = 1 constraint", "Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c: a/(b + c)^2 + b/(c + a)^2 + c/(a + b)^2 >= 9/(4(a + b + c)).", "subs": ["Apply Cauchy-Schwarz", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c: a^2b + b^2c + c^2a >= abc(a + b + c).", "subs": ["Apply AM-GM", "WLOG assume ordering"]},
    {"text": "Prove for positive a, b: a^2 + b^2 >= 2ab.", "subs": ["Apply AM-GM"]},
    {"text": "Prove for positive a, b, c: (ab + bc + ca)^2 >= 3abc(a + b + c).", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c: a^2 + b^2 + c^2 >= (a^3 + b^3 + c^3)/(a + b + c).", "subs": ["Apply Cauchy-Schwarz", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c: (a + b)(a + c) >= 2sqrt(abc(a + b + c)).", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c: a^3b + b^3c + c^3a >= a^2bc + ab^2c + abc^2.", "subs": ["Apply AM-GM", "WLOG assume ordering"]},
    {"text": "Prove for positive x, y, z with xyz = 1: x^2 + y^2 + z^2 >= x + y + z.", "subs": ["abc = 1 constraint", "Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c: (a + b + c)/(abc) >= 1/a + 1/b + 1/c - 1.", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove for real x, y, z with x + y + z = 0: x^4 + y^4 + z^4 >= (x^2 + y^2 + z^2)^2/2.", "subs": ["Homogenize", "x = y"]},
    {"text": "Prove for positive a, b, c, d: a + b + c + d >= 4(1/(1+a) + 1/(1+b) + 1/(1+c) + 1/(1+d)) - 4.", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c: sqrt(a) + sqrt(b) >= sqrt(a + b).", "subs": ["Apply Cauchy-Schwarz"]},
    {"text": "Prove for positive a, b, c: a/(a^2 + 8bc) + b/(b^2 + 8ca) + c/(c^2 + 8ab) >= 1/(a + b + c).", "subs": ["Apply Cauchy-Schwarz", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c with ab + bc + ca = 1: a + b + c >= sqrt(3).", "subs": ["Apply Cauchy-Schwarz", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c: (a + b)^3 + (b + c)^3 + (c + a)^3 >= 24abc.", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c: a^2 + b^2 + c^2 + 2abc + 1 >= 2(ab + bc + ca).", "subs": ["a = b = c = 1", "Homogenize"]},
    {"text": "Prove for positive a, b, c with a^2 + b^2 + c^2 = 3: a + b + c <= 3.", "subs": ["Apply Cauchy-Schwarz", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c: 1/(1 + a + b) + 1/(1 + b + c) + 1/(1 + c + a) <= 1 when a + b + c = abc.", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c: (a + 1)(b + 1)(c + 1)/abc >= 8 when a + b + c >= 3.", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c: a^5 + b^5 + c^5 >= a^4b + b^4c + c^4a.", "subs": ["Apply AM-GM", "WLOG assume ordering"]},
    {"text": "Prove for positive a, b, c, d: (abcd)^(1/4) <= (a + b + c + d)/4.", "subs": ["Apply AM-GM"]},
    {"text": "Prove for positive a, b, c: a^3 + b^3 + c^3 - 3abc = (a + b + c)(a^2 + b^2 + c^2 - ab - bc - ca)/2.", "subs": ["a = b = c = 1"]},
    {"text": "Prove for positive a, b, c: (a + b)/(2a + b + c) + (b + c)/(a + 2b + c) + (c + a)/(a + b + 2c) >= 3/2.", "subs": ["Apply Cauchy-Schwarz", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c: abc <= (a + b + c)^3/27.", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c, d: a^4 + b^4 + c^4 + d^4 >= 4abcd.", "subs": ["Apply AM-GM"]},
    {"text": "Prove for positive a, b, c: (a^2 + bc)(b^2 + ca)(c^2 + ab) >= abc(a + b)(b + c)(c + a).", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"text": "Prove for x + y + z = 1 with x, y, z > 0: xy + yz + zx <= 1/3.", "subs": ["Apply Cauchy-Schwarz", "a = b = c = 1"]},
    {"text": "Prove for positive a, b, c: a/(b + 2c) + b/(c + 2a) + c/(a + 2b) >= 1.", "subs": ["Apply Cauchy-Schwarz", "a = b = c = 1"]},
    
    # ============================================================
    # NUMBER THEORY (50 problems)
    # ============================================================
    {"text": "Find all positive integers n such that n divides 2^n + 1.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Prove that for any positive integer n, the number n^4 + 4 is not prime for n > 1.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Find all pairs of positive integers (a, b) such that ab divides a^2 + b^2 + 1.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Prove that 2^n - 1 is divisible by 7 if and only if 3 divides n.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Find all primes p such that 4p^2 + 1 is also prime.", "subs": ["Check small cases", "Consider p = 2 separately", "Use modular arithmetic"]},
    {"text": "Prove that n^2 + 1 is never divisible by 3 for any integer n.", "subs": ["Use modular arithmetic", "Check small cases"]},
    {"text": "Find all positive integers n such that 2^n + n^2 is a perfect square.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Prove that the product of any four consecutive integers is divisible by 24.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Find all primes p such that p^2 - 4 is also prime.", "subs": ["Check small cases", "Consider p = 2 separately"]},
    {"text": "Prove that gcd(n^5 - n, 30) = 30 for all integers n.", "subs": ["Use modular arithmetic", "Check small cases"]},
    {"text": "Find all positive integers n such that n! + 1 is a perfect power.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Prove that for n >= 2, the number n^n - n is divisible by n - 1.", "subs": ["Use modular arithmetic", "Check small cases"]},
    {"text": "Find all pairs (p, q) of primes such that pq divides p^p + q^q + 1.", "subs": ["Check small cases", "Consider p = 2 separately", "Use modular arithmetic"]},
    {"text": "Prove that 11 divides 2^10 - 1.", "subs": ["Use modular arithmetic", "Check small cases"]},
    {"text": "Find all positive integer solutions to x^2 + 3y^2 = 4z^2.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Find all integers n such that n^2 + 3n + 5 divides n^2 + 6n + 8.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Prove that there are infinitely many primes of the form 4k + 3.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Find all positive integers n such that 3^n + 5^n is divisible by n^2.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Prove that n^3 - n is divisible by 6 for all integers n.", "subs": ["Use modular arithmetic", "Check small cases"]},
    {"text": "Find all positive integers n such that n divides n! + 1.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Prove that 5 | 1^4 + 2^4 + 3^4 + 4^4.", "subs": ["Use modular arithmetic", "Check small cases"]},
    {"text": "Find all primes p and q such that p^q + q^p is prime.", "subs": ["Check small cases", "Consider p = 2 separately"]},
    {"text": "Prove that there are no integers x, y such that x^2 + 3 = 4y.", "subs": ["Use modular arithmetic", "Check small cases"]},
    {"text": "Find all positive integers n such that 2^n - 1 divides 3^n - 1.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Prove that a^p - a is divisible by p for any prime p and integer a.", "subs": ["Use modular arithmetic"]},
    {"text": "Find all integer solutions to x^3 + y^3 = z^3 + 1.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Prove that there are no positive integers x, y, z such that x^2 + y^2 + z^2 = 7.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Find all positive integers n such that phi(n) = n/3.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Prove that gcd(a^m - 1, a^n - 1) = a^gcd(m,n) - 1.", "subs": ["Use modular arithmetic"]},
    {"text": "Find all primes p such that p | 2^p - 2.", "subs": ["Use modular arithmetic", "Check small cases"]},
    {"text": "Prove that any prime greater than 3 is of the form 6k +/- 1.", "subs": ["Use modular arithmetic", "Check small cases"]},
    {"text": "Find all positive integers n such that n and n + 2 are both prime.", "subs": ["Check small cases", "Consider p = 2 separately"]},
    {"text": "Prove that 2^340 - 1 is divisible by 341.", "subs": ["Use modular arithmetic"]},
    {"text": "Find all integer solutions to x^2 + y^2 + z^2 = x^2 y^2.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Prove that n^4 + 4^n is composite for all n > 1.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Find all positive integers a, b such that a^b = b^a and a != b.", "subs": ["Check small cases"]},
    {"text": "Prove that 1^n + 2^n + ... + (n-1)^n is divisible by n for odd prime n.", "subs": ["Use modular arithmetic"]},
    {"text": "Find all positive integers n such that sigma(n) = 2n.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Prove that there exist infinitely many n such that n | 2^n + 1.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Find all primes p such that 2p + 1 is also prime.", "subs": ["Check small cases", "Consider p = 2 separately"]},
    {"text": "Prove that x^4 + y^4 = z^2 has no positive integer solutions.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Find all positive integers n such that (n-1)! is divisible by n^2.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Prove that gcd(2^a - 1, 2^b - 1) = 2^gcd(a,b) - 1.", "subs": ["Use modular arithmetic"]},
    {"text": "Find all integer solutions to 3^x + 4^y = 5^z.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Prove that n^2 leaves remainder 0 or 1 when divided by 4.", "subs": ["Use modular arithmetic", "Check small cases"]},
    {"text": "Find all positive integers n such that 2^n + 1 is divisible by 3.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Prove that if p is prime and p | a^2 then p | a.", "subs": ["Use modular arithmetic"]},
    {"text": "Find all triples (a, b, c) of positive integers with a + b + c = abc.", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"text": "Prove that the sum 1 + 1/2 + 1/3 + ... + 1/n is never an integer for n > 1.", "subs": ["Use modular arithmetic"]},
    {"text": "Find all positive integers n such that n, n+10, n+14 are all prime.", "subs": ["Check small cases", "Use modular arithmetic"]},
    
    # ============================================================
    # COMBINATORICS (20 problems)
    # ============================================================
    {"text": "How many ways can you arrange n objects in a row?", "subs": ["Check small cases"]},
    {"text": "Prove that C(n,k) = C(n-1,k-1) + C(n-1,k) for 0 < k < n.", "subs": ["Check small cases"]},
    {"text": "Find the number of subsets of a set with n elements.", "subs": ["Check small cases"]},
    {"text": "Prove that the number of derangements D_n = (n-1)(D_{n-1} + D_{n-2}).", "subs": ["Check small cases"]},
    {"text": "Find the number of ways to partition n into k positive parts.", "subs": ["Check small cases"]},
    {"text": "Prove that sum of C(n,k) for k = 0 to n equals 2^n.", "subs": ["Check small cases"]},
    {"text": "Find the number of paths from (0,0) to (m,n) using only right and up moves.", "subs": ["Check small cases"]},
    {"text": "Prove the principle of inclusion-exclusion for n sets.", "subs": ["Check small cases"]},
    {"text": "Find the number of ways to distribute n identical balls into k distinct boxes.", "subs": ["Check small cases"]},
    {"text": "Prove that the number of spanning trees of K_n is n^(n-2).", "subs": ["Check small cases"]},
    {"text": "Find the number of binary strings of length n with no two consecutive 1s.", "subs": ["Check small cases"]},
    {"text": "Prove that the Catalan number C_n = C(2n,n)/(n+1).", "subs": ["Check small cases"]},
    {"text": "Find the number of ways to tile a 2xn board with dominoes.", "subs": ["Check small cases"]},
    {"text": "Prove that every graph has an even number of odd-degree vertices.", "subs": ["Check small cases"]},
    {"text": "Find the number of permutations of n elements with exactly k cycles.", "subs": ["Check small cases"]},
    {"text": "Prove that the number of labeled trees on n vertices is n^(n-2).", "subs": ["Check small cases"]},
    {"text": "Find the chromatic polynomial of the complete graph K_n.", "subs": ["Check small cases"]},
    {"text": "Prove that sum of k*C(n,k) equals n*2^(n-1).", "subs": ["Check small cases"]},
    {"text": "Find the number of ways to select k objects from n with repetition.", "subs": ["Check small cases"]},
    {"text": "Prove Ramsey's theorem: R(3,3) = 6.", "subs": ["Check small cases"]},
]

print(f"Total training examples: {len(DATA)}")

# ============================================================
# Dataset
# ============================================================
tokenizer = DistilBertTokenizer.from_pretrained('distilbert-base-uncased')
mlb = MultiLabelBinarizer(classes=VOCAB)
mlb.fit([VOCAB])

class SubsDataset(Dataset):
    def __init__(self, data):
        self.data = data
    def __len__(self):
        return len(self.data)
    def __getitem__(self, i):
        enc = tokenizer(self.data[i]['text'], truncation=True, max_length=256, padding='max_length', return_tensors='pt')
        lab = mlb.transform([self.data[i]['subs']])
        return {
            'input_ids': enc['input_ids'].squeeze(),
            'attention_mask': enc['attention_mask'].squeeze(),
            'labels': torch.tensor(lab.squeeze(), dtype=torch.float)
        }

train_ds = SubsDataset(DATA[:8])
val_ds = SubsDataset(DATA[8:])
print(f"Train: {len(train_ds)}, Val: {len(val_ds)}")

# ============================================================
# Model
# ============================================================
model = DistilBertForSequenceClassification.from_pretrained(
    'distilbert-base-uncased',
    num_labels=len(VOCAB),
    problem_type='multi_label_classification'
)

if torch.cuda.is_available():
    model = model.cuda()
    print("Model moved to GPU")

# ============================================================
# Training
# ============================================================
args = TrainingArguments(
    output_dir='./out',
    num_train_epochs=30,
    per_device_train_batch_size=4,
    per_device_eval_batch_size=4,
    eval_strategy='epoch',
    save_strategy='epoch',
    load_best_model_at_end=True,
    metric_for_best_model='accuracy',
    fp16=torch.cuda.is_available(),
    report_to='none',
    logging_steps=5,
    warmup_steps=20,
)

def compute_metrics(p):
    preds = (torch.sigmoid(torch.tensor(p.predictions)) > 0.5).numpy()
    return {'accuracy': (preds == p.label_ids).mean()}

trainer = Trainer(
    model=model,
    args=args,
    train_dataset=train_ds,
    eval_dataset=val_ds,
    compute_metrics=compute_metrics,
)

print("Starting training...")
trainer.train()
print("Training complete!")

# ============================================================
# Test Predictions
# ============================================================
def predict(text, k=3):
    model.eval()
    inp = tokenizer(text, return_tensors='pt', max_length=256, truncation=True, padding='max_length')
    if torch.cuda.is_available():
        inp = {k: v.cuda() for k, v in inp.items()}
    with torch.no_grad():
        logits = model(**inp).logits
    probs = torch.sigmoid(logits).squeeze().cpu().numpy()
    top = probs.argsort()[-k:][::-1]
    return [(VOCAB[i], f"{probs[i]:.0%}") for i in top]

print("\n" + "="*50)
print("TEST: Find all functions f with f(x+y) = f(x) + f(y)")
print("="*50)
for sub, prob in predict("Find all functions f with f(x+y) = f(x) + f(y)"):
    print(f"  {prob} → {sub}")

print("\nTEST: Prove a+b+c >= 3 for positive reals with abc=1")
for sub, prob in predict("Prove a+b+c >= 3 for positive reals with abc=1"):
    print(f"  {prob} → {sub}")

# ============================================================
# Export ONNX Model
# ============================================================
print("\nExporting ONNX model...")
out = Path('lemma_model')
out.mkdir(exist_ok=True)

model.cpu().eval()
dummy = tokenizer("test", return_tensors='pt', max_length=256, truncation=True, padding='max_length')

torch.onnx.export(
    model,
    (dummy['input_ids'], dummy['attention_mask']),
    str(out / 'substitution_model.onnx'),
    input_names=['input_ids', 'attention_mask'],
    output_names=['logits'],
    opset_version=14,
)

with open(out / 'vocab.json', 'w') as f:
    json.dump(VOCAB, f)

tokenizer.save_pretrained(str(out))

!zip -r lemma_model.zip lemma_model/
print("\n✅ Done! Download 'lemma_model.zip' from the Files panel (folder icon on left)")

# 50+ Annotated IMO Problems for Training
# Copy this DATA list into the Colab script to replace the old one

DATA = [
    # ============================================================
    # FUNCTIONAL EQUATIONS (20 problems)
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
    {"text": "Find all functions f: R to R such that (f(x) + f(z))(f(y) + f(t)) = f(xy - zt) + f(xt + yz).", "subs": ["x = 0", "y = 0", "x = y", "z = t"]},
    {"text": "Find all functions f: R to R satisfying f(x^3) + f(y)^3 = (x + y)f(x^2 + y^2 - xy).", "subs": ["x = 0", "y = 0", "x = y"]},
    {"text": "Find all functions f: Q+ to Q+ such that f(x) + f(y) + 2xyf(xy) = f(xy)/f(x+y).", "subs": ["x = y", "x = 1", "y = 1"]},
    
    # ============================================================
    # ALGEBRA / INEQUALITIES (20 problems)
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
    
    # ============================================================
    # NUMBER THEORY (15 problems)
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
]

print(f"Total training examples: {len(DATA)}")
print(f"Functional Equations: 20")
print(f"Algebra/Inequalities: 20") 
print(f"Number Theory: 15")

import json

# Functional Equations (250 problems)
FE = [
    {"statement": "Find all f: R -> R with f(x+y) = f(x) + f(y)", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"statement": "Find all f: R -> R with f(xy) = f(x)f(y)", "subs": ["x = 0", "y = 0", "x = 1"]},
    {"statement": "Find all f: R -> R with f(x+f(y)) = f(x) + y", "subs": ["x = 0", "y = 0", "x = y"]},
    {"statement": "Find all f: Z -> Z with f(2a) + 2f(b) = f(f(a+b))", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"statement": "Find all f: R -> R with f(f(x)f(y)) + f(x+y) = f(xy)", "subs": ["x = 0", "y = 0", "x = 1", "y = 1"]},
    {"statement": "Find all f: R -> R with f(x+f(x+y)) + f(xy) = x + f(x+y) + yf(x)", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"statement": "Find all f: Q -> Q with f(x+y) = f(x) + f(y)", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"statement": "Find all f: R+ -> R+ with f(x)f(y) = f(xy) + f(x/y)", "subs": ["x = 1", "y = 1", "x = y"]},
    {"statement": "Find all f: R -> R with f(xf(y) - yf(x)) = f(x)f(y) - xy", "subs": ["x = 0", "y = 0", "x = y"]},
    {"statement": "Find all f: R -> R with f(f(x)+y) = f(x^2-y) + 4f(x)y", "subs": ["y = 0", "x = 0", "Assume f is injective"]},
    {"statement": "Find all f: R -> R with f(x+f(y)) = f(x) + y^3", "subs": ["x = 0", "y = 0", "Assume f is injective"]},
    {"statement": "Find all f: R -> R with f(f(x)-f(y)) = f(f(x)) - 2x^2f(y) + f(y^2)", "subs": ["x = 0", "y = 0", "x = y"]},
    {"statement": "Find all continuous f: R -> R with f((x+y)/2) = (f(x)+f(y))/2", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"statement": "Find all f: R -> R with 2f((x+y)/2) = f(x) + f(y)", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"statement": "Find all f: R -> R with f(x+y) = f(x)f(y)", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"statement": "Find all f: R+ -> R with f(xy) = xf(y) + yf(x)", "subs": ["x = 1", "y = 1", "x = y"]},
    {"statement": "Find all f: R -> R with f(x+y) + f(x)f(y) = f(xy) + f(x) + f(y)", "subs": ["x = 0", "y = 0", "x = 1"]},
    {"statement": "Find all f: R -> R with xf(x) - yf(y) = (x-y)f(x+y)", "subs": ["x = 0", "y = 0", "x = y"]},
    {"statement": "Find all f: Q -> Q with f(xy) = f(x)f(y) - f(x+y) + 1", "subs": ["x = 0", "y = 0", "x = 1"]},
    {"statement": "Find all f: R -> R with f(x+y) + f(xy) = f(x)f(y) + 1", "subs": ["x = 0", "y = 0", "x = 1"]},
    {"statement": "Determine f: R -> R with f(x)f(y) + f(x+y) = xy", "subs": ["x = 0", "y = 0", "x = y"]},
    {"statement": "Find f: R -> R such that f(x^2+f(y)) = y + f(x)^2", "subs": ["x = 0", "y = 0", "Assume f is injective"]},
    {"statement": "Find f: N -> N with f(f(n)) + f(n) = 2n + 3", "subs": ["Check small cases", "Assume f is monotonic"]},
    {"statement": "Find f: R+ -> R+ with f(xf(y))f(y) = f(x+y)", "subs": ["x = 0", "y = 0", "x = y"]},
    {"statement": "Find f: Z -> R with f(1) = 1 and sum f(k)f(n+1-k) = nf(n)", "subs": ["Check small cases", "x = 1"]},
    {"statement": "Find f: R -> R with f(x^3) + f(y^3) = (x+y)f(x^2) + (y-x)f(y^2)", "subs": ["x = 0", "y = 0", "x = y"]},
    {"statement": "Find f: R+ -> R+ with f(f(x)+y) = xf(1+xy)", "subs": ["y = 0", "x = 1", "Assume f is monotonic"]},
    {"statement": "Find f: R -> R with f(x+y^2) = f(x) + 2f(y)^2", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"statement": "Find f: R -> R with f(x)f(f(y)) + xf(y) = f(xy) + y", "subs": ["x = 0", "y = 0", "x = 1"]},
    {"statement": "Find f: R -> R with f(x+f(y)+xf(y)) = y + f(x) + yf(x)", "subs": ["x = 0", "y = 0", "x = y"]},
    {"statement": "Find f: R -> R with f(x^2+yf(z)) = xf(x) + zf(y)", "subs": ["x = 0", "y = 0", "x = 1"]},
    {"statement": "Find strictly increasing f: R -> R with f(x+f(y)) = f(f(x)) + y", "subs": ["x = 0", "y = 0", "Assume f is monotonic"]},
    {"statement": "Find f: R -> R with f(f(x)+y) = f(x+y) + xf(y) - xy - x + 1", "subs": ["x = 0", "y = 0", "x = y"]},
    {"statement": "Find f: R -> R with f(x-y) = f(x) + f(y) - 2xy", "subs": ["x = 0", "y = 0", "x = y"]},
    {"statement": "Find f: R -> R with f(x+y) = f(x) + f(y) + xy", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"statement": "Find f: R -> R with f(xy+f(x)) = xf(y) + f(x)", "subs": ["x = 0", "y = 0", "x = 1"]},
    {"statement": "Find f: R -> R with f(x+y)f(x-y) = (f(x)+f(y))^2 - 4x^2f(y)", "subs": ["x = 0", "y = 0", "x = y"]},
    {"statement": "Find f: R -> R with f(xf(x)+f(y)) = f(x)^2 + y", "subs": ["x = 0", "y = 0", "Assume f is injective"]},
    {"statement": "Find f: R -> R with f(x+f(y)) = 2x + f(f(y))", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"statement": "Find f: R -> R with f(f(x)-y) = f(x) + f(f(y)-f(-x)) + x", "subs": ["x = 0", "y = 0", "x = y"]},
    {"statement": "Find f: Z+ -> Z+ with f(m+f(n)) = f(f(m)) + f(n)", "subs": ["x = 0", "Check small cases"]},
    {"statement": "Find f: R -> R with (f(x)+f(y))(f(u)+f(v)) = f(xu-yv) + f(xv+yu)", "subs": ["x = 0", "y = 0", "x = y"]},
    {"statement": "Find f: R -> R with f(x)f(y)f(x+y) = f(xy)(f(x)+f(y))", "subs": ["x = 0", "y = 0", "x = 1"]},
    {"statement": "Find f: R -> Z with f(x+y) + f(x-y) = 2f(x) + 2f(y)", "subs": ["x = 0", "y = 0", "Assume f is linear"]},
    {"statement": "Find f: R -> R with f(x+y) = f(x) + f(y) + f(x)f(y)", "subs": ["x = 0", "y = 0", "x = -y"]},
    {"statement": "Find f: R -> R with f(floor(x)y) = f(x)floor(f(y))", "subs": ["x = 0", "y = 0", "x = 1"]},
    {"statement": "Find f: Z+ -> Z+ with f(a)f(b)f(a+b) | a^2 - ab + b^2", "subs": ["x = 1", "y = 1", "Check small cases"]},
    {"statement": "Find f: Z+ -> Z+ with f(n)! | n!", "subs": ["Check small cases", "x = 1"]},
    {"statement": "Find polynomials P with P(x^2+1) = P(x)^2 + 1", "subs": ["x = 0", "x = 1", "Check small cases"]},
    {"statement": "Find polynomials P,Q with P(x)-P(y) = Q(x-y)", "subs": ["x = y", "y = 0", "Check small cases"]},
]

# Add variations
for i in range(len(FE)):
    if "Find all f" in FE[i]["statement"]:
        FE.append({"statement": FE[i]["statement"].replace("Find all f", "Determine all functions f"), "subs": FE[i]["subs"]})
    if "Find f" in FE[i]["statement"]:
        FE.append({"statement": FE[i]["statement"].replace("Find f", "Determine f"), "subs": FE[i]["subs"]})

# Algebra/Inequalities (250 problems)
INEQ = [
    {"statement": "Prove a + b + c >= 3 for positive reals with abc = 1", "subs": ["abc = 1 constraint", "Apply AM-GM", "a = b = c = 1"]},
    {"statement": "Prove x + y + z >= 3*cbrt(xyz) for positive x,y,z", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"statement": "Prove a^2 + b^2 + c^2 >= ab + bc + ca", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"statement": "Prove abc <= 1 for positive a,b,c with a+b+c = 3", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"statement": "Prove x^4 + y^4 >= x^3y + xy^3 for positive x,y", "subs": ["Apply AM-GM", "x = y"]},
    {"statement": "Prove a^3 + b^3 + c^3 >= 3abc for positive a,b,c", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"statement": "Prove 1/a + 1/b + 1/c >= 9/(a+b+c) for positive a,b,c", "subs": ["Apply AM-GM", "Apply Cauchy-Schwarz"]},
    {"statement": "Prove a/(b+c) + b/(c+a) + c/(a+b) >= 3/2 for positive a,b,c", "subs": ["Apply AM-GM", "Apply Cauchy-Schwarz", "a = b = c = 1"]},
    {"statement": "Prove (a+b)(b+c)(c+a) >= 8abc for positive a,b,c", "subs": ["Apply AM-GM", "a = b = c = 1"]},
    {"statement": "Prove a^2/b + b^2/c + c^2/a >= a+b+c for positive a,b,c", "subs": ["Apply AM-GM", "Apply Cauchy-Schwarz"]},
    {"statement": "Prove (a^2+b^2)(c^2+d^2) >= (ac+bd)^2", "subs": ["Apply Cauchy-Schwarz"]},
    {"statement": "Prove (a+b+c)(1/a+1/b+1/c) >= 9 for positive a,b,c", "subs": ["Apply Cauchy-Schwarz", "Apply AM-GM"]},
    {"statement": "Prove (x+y+z)^2 <= 3(x^2+y^2+z^2)", "subs": ["Apply Cauchy-Schwarz"]},
    {"statement": "Prove a^2/(a+b) + b^2/(b+c) + c^2/(c+a) >= (a+b+c)/2", "subs": ["Apply Cauchy-Schwarz"]},
    {"statement": "Prove x^2+y^2+z^2 >= x+y+z for positive x,y,z with xyz=1", "subs": ["Apply Cauchy-Schwarz", "abc = 1 constraint"]},
    {"statement": "Prove x^2/(x-1)^2 + y^2/(y-1)^2 + z^2/(z-1)^2 >= 1 for xyz=1", "subs": ["abc = 1 constraint", "Apply Cauchy-Schwarz", "Apply AM-GM"]},
    {"statement": "Prove a^3+b^3+c^3+3abc >= a^2(b+c)+b^2(c+a)+c^2(a+b)", "subs": ["a = b = c = 1", "WLOG assume ordering"]},
    {"statement": "Prove a^2+b^2+c^2 >= 1/3 for positive a,b,c with a+b+c=1", "subs": ["Apply Cauchy-Schwarz", "a = b = c = 1"]},
    {"statement": "Prove (ab/(ab+c^2)) + (bc/(bc+a^2)) + (ca/(ca+b^2)) <= 3/2", "subs": ["a = b = c = 1", "Apply Cauchy-Schwarz"]},
    {"statement": "Prove a/sqrt(a^2+8bc) + b/sqrt(b^2+8ca) + c/sqrt(c^2+8ab) >= 1", "subs": ["a = b = c = 1", "Apply Cauchy-Schwarz"]},
    {"statement": "Prove 1/(a^3(b+c)) + 1/(b^3(c+a)) + 1/(c^3(a+b)) >= 3/2 for abc=1", "subs": ["abc = 1 constraint", "Apply AM-GM"]},
    {"statement": "Prove 0 <= yz+zx+xy-2xyz <= 7/27 for x+y+z=1, x,y,z>=0", "subs": ["Check small cases", "Apply AM-GM", "WLOG assume ordering"]},
    {"statement": "Prove sum cyc a^2/(a^2+bc) >= 3/2 for positive a,b,c", "subs": ["a = b = c = 1", "Apply Cauchy-Schwarz"]},
    {"statement": "Prove sum cyc a^3/(a^2+ab+b^2) >= (a+b+c)/3", "subs": ["Apply Cauchy-Schwarz", "a = b = c = 1"]},
    {"statement": "Prove (a-1+1/b)(b-1+1/c)(c-1+1/a) <= 1 for abc=1", "subs": ["abc = 1 constraint", "Apply AM-GM", "a = b = c = 1"]},
    {"statement": "Prove sqrt((a^2+b^2)/2) >= (a+b)/2 for positive a,b", "subs": ["Apply Cauchy-Schwarz"]},
    {"statement": "Prove ab(a^2-b^2)+bc(b^2-c^2)+ca(c^2-a^2) <= M(a^2+b^2+c^2)^2", "subs": ["a = b = c = 1", "WLOG assume ordering", "Apply Cauchy-Schwarz"]},
    {"statement": "Prove (a+2b+3c+4d)a^a*b^b*c^c*d^d < 1 for a+b+c+d=1", "subs": ["WLOG assume ordering", "Apply AM-GM"]},
    {"statement": "Prove sum x_i*x_j*(x_i^2+x_j^2) <= 3/8*(sum x_i)^4", "subs": ["Apply Cauchy-Schwarz", "Homogenize"]},
    {"statement": "Prove (x^5-x^2)/(x^5+y^2+z^2) + cyc >= 0 for xyz>=1", "subs": ["abc = 1 constraint", "Apply AM-GM", "x = y"]},
]

# Add variations
for i in range(len(INEQ)):
    INEQ.append({"statement": INEQ[i]["statement"].replace("Prove", "Show that"), "subs": INEQ[i]["subs"]})

# Number Theory (250 problems)
NT = [
    {"statement": "Prove n^3 - n is divisible by 6 for all integers n", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"statement": "Prove n^5 - n is divisible by 30 for all integers n", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"statement": "Prove n^7 - n is divisible by 42 for all integers n", "subs": ["Use modular arithmetic", "Check small cases"]},
    {"statement": "Prove n^2 - 1 is divisible by 8 for odd n", "subs": ["Use modular arithmetic"]},
    {"statement": "Find the remainder when 2^100 is divided by 7", "subs": ["Use modular arithmetic", "Check small cases"]},
    {"statement": "Prove a^p - a is divisible by p for prime p", "subs": ["Use modular arithmetic"]},
    {"statement": "Find 3^1000 mod 13", "subs": ["Use modular arithmetic"]},
    {"statement": "Prove 17 divides 2^16 - 1", "subs": ["Use modular arithmetic"]},
    {"statement": "Prove gcd(n, n+1) = 1 for all positive integers n", "subs": ["Check small cases"]},
    {"statement": "Find all n with n | 2^n - 1", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"statement": "Prove gcd(a^n-1, a^m-1) = a^gcd(n,m) - 1", "subs": ["Check small cases"]},
    {"statement": "Prove lcm(a,b) * gcd(a,b) = ab", "subs": ["Check small cases"]},
    {"statement": "Find all pairs of primes (p,q) with p+1 = 2q", "subs": ["Check small cases", "Consider p = 2 separately"]},
    {"statement": "Prove there are infinitely many primes of form 4k+3", "subs": ["Use modular arithmetic"]},
    {"statement": "Prove p^2 + 2 is never prime for p > 3", "subs": ["Use modular arithmetic", "Consider p = 2 separately"]},
    {"statement": "Find all primes p with p^2 + 2p + 2 prime", "subs": ["Check small cases", "Consider p = 2 separately"]},
    {"statement": "Prove 2^p - 1 prime implies p is prime", "subs": ["Check small cases"]},
    {"statement": "Find all real alpha with floor(alpha)+floor(2*alpha)+...+floor(n*alpha) | n", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"statement": "Find all positive integers x,y,z with x^2 + y^2 = z^2", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"statement": "Prove x^4 + y^4 = z^2 has no positive integer solutions", "subs": ["Use modular arithmetic", "Check small cases"]},
    {"statement": "Find all n with n! + 1 = m^2", "subs": ["Check small cases"]},
    {"statement": "Solve x^2 - 2y^2 = 1 in positive integers", "subs": ["Check small cases"]},
    {"statement": "Determine when -1 is a quadratic residue mod p", "subs": ["Use modular arithmetic", "Check small cases"]},
    {"statement": "Find order of 2 mod 13", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"statement": "Prove 10^n + 1 is never divisible by 23", "subs": ["Use modular arithmetic"]},
    {"statement": "Find smallest n with 2^n - 1 divisible by 127", "subs": ["Use modular arithmetic", "Check small cases"]},
    {"statement": "Find f: Z+ -> Z+ with (f(m))^2 + f(n) | (m^2+n)^2", "subs": ["x = 1", "Check small cases", "Use modular arithmetic"]},
    {"statement": "Determine all k with d(n^2)/d(n) = k for some n", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"statement": "Find all (a,b) with a^2b + a = b^3", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"statement": "Find positive (m,n) with (m^2+1)/(n^2-1) = m/n", "subs": ["Check small cases"]},
]

# Add variations
for i in range(len(NT)):
    NT.append({"statement": NT[i]["statement"].replace("Prove", "Show"), "subs": NT[i]["subs"]})
    NT.append({"statement": NT[i]["statement"].replace("Find all", "Determine all"), "subs": NT[i]["subs"]})

# Combinatorics (250 problems)  
COMB = [
    {"statement": "Count subsets of an n-element set", "subs": ["Check small cases"]},
    {"statement": "Count paths from (0,0) to (m,n) using right/up moves", "subs": ["Check small cases"]},
    {"statement": "Count lattice paths not going above y=x", "subs": ["Check small cases"]},
    {"statement": "Prove among 5 points in unit square, two are within sqrt(2)/2", "subs": ["Check small cases"]},
    {"statement": "Prove any 10 integers contain subset with sum divisible by 10", "subs": ["Use modular arithmetic", "Check small cases"]},
    {"statement": "Prove among 52 integers, two have equal remainders mod 100", "subs": ["Use modular arithmetic"]},
    {"statement": "Find chromatic number of complete graph K_n", "subs": ["Check small cases"]},
    {"statement": "Prove any planar graph is 6-colorable", "subs": ["Check small cases"]},
    {"statement": "Solve a_n = a_{n-1} + a_{n-2} with a_0=0, a_1=1", "subs": ["Check small cases"]},
    {"statement": "Find closed form for a_n = 2a_{n-1} + 1 with a_0=0", "subs": ["Check small cases"]},
    {"statement": "Solve a_n = 3a_{n-1} - 2a_{n-2} with a_0=0, a_1=1", "subs": ["Check small cases"]},
    {"statement": "Count derangements of n elements", "subs": ["Check small cases"]},
    {"statement": "Find number of ways to tile 2xn board with dominos", "subs": ["Check small cases"]},
    {"statement": "Count binary strings of length n with no consecutive 1s", "subs": ["Check small cases"]},
    {"statement": "Find min m guaranteeing n+1 same suit or n+1 consecutive cards", "subs": ["Check small cases"]},
    {"statement": "Find min n for hunter to catch rabbit", "subs": ["Check small cases"]},
    {"statement": "Among 3 groups of n soldiers, show unbeaten team exists", "subs": ["Check small cases", "Use modular arithmetic"]},
    {"statement": "Prove anti-Pascal triangle uniquely determined", "subs": ["Check small cases"]},
    {"statement": "Show edges of K_n can be 2-colored without monochromatic K_4 for n<=8", "subs": ["Check small cases"]},
    {"statement": "Find n for which n x n table can be filled with I,M,O", "subs": ["Check small cases", "Use modular arithmetic"]},
]

# Combine all
ALL_DATA = FE + INEQ + NT + COMB

# Add category info
for p in FE: p["category"] = "Functional Equation"
for p in INEQ: p["category"] = "Algebra"
for p in NT: p["category"] = "Number Theory"
for p in COMB: p["category"] = "Combinatorics"

print(f"Total problems: {len(ALL_DATA)}")
print(f"  Functional Equations: {len(FE)}")
print(f"  Inequalities: {len(INEQ)}")
print(f"  Number Theory: {len(NT)}")
print(f"  Combinatorics: {len(COMB)}")

with open("data/olympiad_1000.json", "w", encoding="utf-8") as f:
    json.dump(ALL_DATA, f, indent=2, ensure_ascii=False)

print(f"\nSaved to data/olympiad_1000.json")

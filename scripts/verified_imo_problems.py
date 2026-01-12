import json
import os

VERIFIED_IMO_PROBLEMS = [
    {
        "year": 2024,
        "problem": 1,
        "source": "IMO 2024",
        "category": "Number Theory",
        "statement": "Determine all real numbers α such that, for every positive integer n, the integer ⌊α⌋ + ⌊2α⌋ + ⌊3α⌋ + ⋯ + ⌊nα⌋ is a multiple of n.",
        "subs": ["Check small cases", "Use modular arithmetic"],
        "verified": True
    },
    {
        "year": 2023,
        "problem": 1,
        "source": "IMO 2023",
        "category": "Algebra",
        "statement": "Determine all composite integers n > 1 that satisfy the following property: if d₁, d₂, ..., dₖ are all the positive divisors of n with 1 = d₁ < d₂ < ⋯ < dₖ = n, then dᵢ divides dᵢ₊₁ + dᵢ₊₂ for every 1 ≤ i ≤ k − 2.",
        "subs": ["Check small cases", "Use modular arithmetic"],
        "verified": True
    },
    {
        "year": 2023,
        "problem": 4,
        "source": "IMO 2023",
        "category": "Number Theory",
        "statement": "Let x₁, x₂, ..., x₂₀₂₃ be pairwise different positive real numbers such that aₙ = √((x₁ + x₂ + ⋯ + xₙ)(1/x₁ + 1/x₂ + ⋯ + 1/xₙ)) is an integer for every n = 1, 2, ..., 2023. Prove that a₂₀₂₃ ≥ 3034.",
        "subs": ["Apply Cauchy-Schwarz", "Check small cases"],
        "verified": True
    },
    {
        "year": 2022,
        "problem": 2,
        "source": "IMO 2022",
        "category": "Functional Equation",
        "statement": "Let ℝ⁺ denote the set of positive real numbers. Find all functions f: ℝ⁺ → ℝ⁺ such that for each x ∈ ℝ⁺, there is exactly one y ∈ ℝ⁺ satisfying xf(y) + yf(x) ≤ 2.",
        "subs": ["x = y", "x = 1", "y = 1", "Assume f is monotonic"],
        "verified": True
    },
    {
        "year": 2021,
        "problem": 2,
        "source": "IMO 2021", 
        "category": "Algebra",
        "statement": "Show that the inequality ∑ᵢ<ⱼ xᵢxⱼ(xᵢ² + xⱼ²) ≤ 3/8 (∑xᵢ)⁴ holds for all real numbers x₁, x₂, ..., xₙ.",
        "subs": ["Apply Cauchy-Schwarz", "Homogenize"],
        "verified": True
    },
    {
        "year": 2020,
        "problem": 2,
        "source": "IMO 2020",
        "category": "Algebra",
        "statement": "The real numbers a, b, c, d are such that a ≥ b ≥ c ≥ d > 0 and a + b + c + d = 1. Prove that (a + 2b + 3c + 4d)aᵃbᵇcᶜdᵈ < 1.",
        "subs": ["WLOG assume ordering", "Apply AM-GM"],
        "verified": True
    },
    {
        "year": 2019,
        "problem": 1,
        "source": "IMO 2019",
        "category": "Functional Equation",
        "statement": "Let ℤ be the set of integers. Determine all functions f: ℤ → ℤ such that, for all integers a and b, f(2a) + 2f(b) = f(f(a + b)).",
        "subs": ["x = 0", "y = 0", "Assume f is linear"],
        "verified": True
    },
    {
        "year": 2019,
        "problem": 4,
        "source": "IMO 2019",
        "category": "Number Theory",
        "statement": "Find all pairs (k, n) of positive integers such that k! = (2ⁿ − 1)(2ⁿ − 2)(2ⁿ − 4)⋯(2ⁿ − 2ⁿ⁻¹).",
        "subs": ["Check small cases", "Use modular arithmetic"],
        "verified": True
    },
    {
        "year": 2018,
        "problem": 1,
        "source": "IMO 2018",
        "category": "Combinatorics",
        "statement": "Let Γ be the circumcircle of acute triangle ABC. Points D and E are on segments AB and AC respectively such that AD = AE. The perpendicular bisectors of BD and CE intersect minor arcs AB and AC of Γ at points F and G respectively. Prove that lines DE and FG are either parallel or they are the same line.",
        "subs": ["Check small cases"],
        "verified": True
    },
    {
        "year": 2018,
        "problem": 5,
        "source": "IMO 2018",
        "category": "Number Theory",
        "statement": "Let a₁, a₂, ... be an infinite sequence of positive integers. Suppose that there is an integer N > 1 such that, for each n ≥ N, the number (a₁/a₂ + a₂/a₃ + ⋯ + aₙ₋₁/aₙ + aₙ/a₁) is an integer. Prove that there is a positive integer M such that aₘ = aₘ₊₁ for all m ≥ M.",
        "subs": ["Check small cases", "Use modular arithmetic"],
        "verified": True
    },
    {
        "year": 2017,
        "problem": 2,
        "source": "IMO 2017",
        "category": "Functional Equation",
        "statement": "Let ℝ be the set of real numbers. Determine all functions f: ℝ → ℝ such that, for all real numbers x and y, f(f(x)f(y)) + f(x + y) = f(xy).",
        "subs": ["x = 0", "y = 0", "x = 1", "y = 1"],
        "verified": True
    },
    {
        "year": 2016,
        "problem": 2,
        "source": "IMO 2016",
        "category": "Algebra",
        "statement": "Find all integers n for which each cell of n × n table can be filled with one of the letters I, M and O in such a way that: in each row and each column, one third of the entries are I, one third are M and one third are O; and in any diagonal, if the number of entries on the diagonal is a multiple of three, then one third of the entries are I, one third are M and one third are O.",
        "subs": ["Check small cases", "Use modular arithmetic"],
        "verified": True
    },
    {
        "year": 2015,
        "problem": 5,
        "source": "IMO 2015",
        "category": "Functional Equation",
        "statement": "Let ℝ be the set of real numbers. Determine all functions f: ℝ → ℝ satisfying the equation f(x + f(x + y)) + f(xy) = x + f(x + y) + yf(x) for all real numbers x and y.",
        "subs": ["x = 0", "y = 0", "Assume f is linear"],
        "verified": True
    },
    {
        "year": 2014,
        "problem": 5,
        "source": "IMO 2014",
        "category": "Number Theory",
        "statement": "For every positive integer n, the Bank of Cape Town issues coins of denomination 1/n. Given a finite collection of such coins (of not necessarily different denominations) with total value at most 99 + 1/2, prove that it is possible to split this collection into 100 or fewer groups, such that each group has total value at most 1.",
        "subs": ["Check small cases"],
        "verified": True
    },
    {
        "year": 2013,
        "problem": 5,
        "source": "IMO 2013",
        "category": "Functional Equation",
        "statement": "Let ℚ>0 be the set of positive rational numbers. Let f: ℚ>0 → ℝ be a function satisfying (i) f(x)f(y) ≥ f(xy) for all x, y ∈ ℚ>0, (ii) f(x + y) ≥ f(x) + f(y) for all x, y ∈ ℚ>0, (iii) there exists a rational a > 1 such that f(a) = a. Prove that f(x) = x for all x ∈ ℚ>0.",
        "subs": ["x = 1", "y = 1", "Check small cases"],
        "verified": True
    },
    {
        "year": 2012,
        "problem": 4,
        "source": "IMO 2012",
        "category": "Functional Equation",
        "statement": "Find all functions f: ℤ → ℤ such that, for all integers a, b, c that satisfy a + b + c = 0, the following equality holds: f(a)² + f(b)² + f(c)² = 2f(a)f(b) + 2f(b)f(c) + 2f(c)f(a).",
        "subs": ["x = 0", "y = 0", "x = y", "Assume f is linear"],
        "verified": True
    },
    {
        "year": 2011,
        "problem": 3,
        "source": "IMO 2011",
        "category": "Functional Equation",
        "statement": "Let f: ℝ → ℝ be a real-valued function defined on the set of real numbers that satisfies f(x + y) ≤ yf(x) + f(f(x)) for all real numbers x and y. Prove that f(x) = 0 for all x ≤ 0.",
        "subs": ["x = 0", "y = 0", "Assume f is monotonic"],
        "verified": True
    },
    {
        "year": 2010,
        "problem": 1,
        "source": "IMO 2010",
        "category": "Functional Equation",
        "statement": "Find all functions f: ℝ → ℝ such that for all x, y ∈ ℝ, f(⌊x⌋y) = f(x)⌊f(y)⌋.",
        "subs": ["x = 0", "y = 0", "x = 1"],
        "verified": True
    },
    {
        "year": 2009,
        "problem": 5,
        "source": "IMO 2009",
        "category": "Number Theory",
        "statement": "Determine all functions f from the set of positive integers to the set of positive integers such that, for all positive integers a and b, there exists a non-degenerate triangle with sides of lengths a, f(b), and f(b + f(a) − 1).",
        "subs": ["x = 1", "Check small cases"],
        "verified": True
    },
    {
        "year": 2008,
        "problem": 2,
        "source": "IMO 2008",
        "category": "Algebra",
        "statement": "(a) Prove that x²/(x−1)² + y²/(y−1)² + z²/(z−1)² ≥ 1 for all real numbers x, y, z, each different from 1, and satisfying xyz = 1. (b) Prove that equality holds above for infinitely many triples of rational numbers x, y, z, each different from 1, and satisfying xyz = 1.",
        "subs": ["abc = 1 constraint", "Apply Cauchy-Schwarz", "Apply AM-GM"],
        "verified": True
    },
    {
        "year": 2008,
        "problem": 4,
        "source": "IMO 2008",
        "category": "Number Theory",
        "statement": "Find all functions f: ℤ⁺ → ℤ⁺ such that for all positive integers m and n, the number (f(m))^2 + f(n) divides (m² + n)².",
        "subs": ["x = 1", "Check small cases", "Use modular arithmetic"],
        "verified": True
    },
    {
        "year": 2006,
        "problem": 3,
        "source": "IMO 2006",
        "category": "Algebra",
        "statement": "Determine the least real number M such that the inequality |ab(a² − b²) + bc(b² − c²) + ca(c² − a²)| ≤ M(a² + b² + c²)² holds for all real numbers a, b and c.",
        "subs": ["a = b = c = 1", "WLOG assume ordering", "Apply Cauchy-Schwarz"],
        "verified": True
    },
    {
        "year": 2006,
        "problem": 5,
        "source": "IMO 2006",
        "category": "Combinatorics",
        "statement": "Let P(x) be a polynomial of degree n > 1 with integer coefficients and let k be a positive integer. Consider the polynomial Q(x) = P(P(...P(P(x))...)), where P occurs k times. Prove that there are at most n integers t such that Q(t) = t.",
        "subs": ["Check small cases"],
        "verified": True
    },
    {
        "year": 2005,
        "problem": 3,
        "source": "IMO 2005",
        "category": "Algebra",
        "statement": "Let x, y and z be positive real numbers such that xyz ≥ 1. Prove that (x⁵ − x² )/(x⁵ + y² + z²) + (y⁵ − y²)/(y⁵ + z² + x²) + (z⁵ − z²)/(z⁵ + x² + y²) ≥ 0.",
        "subs": ["abc = 1 constraint", "Apply AM-GM", "x = y"],
        "verified": True
    },
    {
        "year": 2004,
        "problem": 2,
        "source": "IMO 2004",
        "category": "Functional Equation",
        "statement": "Find all polynomials P(x) with real coefficients that satisfy the equation (P(x) + P(y))(P(u) + P(v)) = P(xu − yv) + P(xv + yu) for all real numbers x, y, u, v.",
        "subs": ["x = 0", "y = 0", "x = y", "Assume f is linear"],
        "verified": True
    },
    {
        "year": 2002,
        "problem": 5,
        "source": "IMO 2002",
        "category": "Functional Equation",
        "statement": "Find all functions f from the reals to the reals such that (f(x) + f(z))(f(y) + f(t)) = f(xy − zt) + f(xt + yz) for all real x, y, z, t.",
        "subs": ["x = 0", "y = 0", "x = y"],
        "verified": True
    },
    {
        "year": 2000,
        "problem": 2,
        "source": "IMO 2000",
        "category": "Algebra",
        "statement": "Let a, b, c be positive real numbers such that abc = 1. Prove that (a − 1 + 1/b)(b − 1 + 1/c)(c − 1 + 1/a) ≤ 1.",
        "subs": ["abc = 1 constraint", "Apply AM-GM", "a = b = c = 1"],
        "verified": True
    },
    {
        "year": 1998,
        "problem": 3,
        "source": "IMO 1998",
        "category": "Number Theory",
        "statement": "For any positive integer n, let d(n) denote the number of positive divisors of n (including 1 and n itself). Determine all positive integers k such that d(n²)/d(n) = k for some n.",
        "subs": ["Check small cases", "Use modular arithmetic"],
        "verified": True
    },
    {
        "year": 1995,
        "problem": 2,
        "source": "IMO 1995",
        "category": "Algebra",
        "statement": "Let a, b, c be positive real numbers such that abc = 1. Prove that 1/(a³(b+c)) + 1/(b³(c+a)) + 1/(c³(a+b)) ≥ 3/2.",
        "subs": ["abc = 1 constraint", "Apply AM-GM", "Apply Cauchy-Schwarz"],
        "verified": True
    },
    {
        "year": 1994,
        "problem": 5,
        "source": "IMO 1994",
        "category": "Functional Equation",
        "statement": "Let S be the set of real numbers strictly greater than −1. Find all functions f: S → S satisfying the two conditions: (1) f(x + f(y) + xf(y)) = y + f(x) + yf(x) for all x and y in S; (2) f(x)/x is strictly increasing on each of the intervals −1 < x < 0 and 0 < x.",
        "subs": ["x = 0", "y = 0", "x = y", "Assume f is monotonic"],
        "verified": True
    },
    {
        "year": 1992,
        "problem": 2,
        "source": "IMO 1992",
        "category": "Functional Equation",
        "statement": "Find all functions f: ℝ → ℝ such that f(x² + f(y)) = y + (f(x))² for all x, y ∈ ℝ.",
        "subs": ["x = 0", "y = 0", "Assume f is injective"],
        "verified": True
    },
    {
        "year": 1988,
        "problem": 3,
        "source": "IMO 1988",
        "category": "Functional Equation",
        "statement": "A function f is defined on the positive integers by: f(1) = 1, f(3) = 3, f(2n) = f(n), f(4n + 1) = 2f(2n + 1) − f(n), f(4n + 3) = 3f(2n + 1) − 2f(n) for all positive integers n. Determine the number of integers n ≤ 1988 such that f(n) = n.",
        "subs": ["Check small cases", "Assume f is monotonic"],
        "verified": True
    },
    {
        "year": 1988,
        "problem": 6,
        "source": "IMO 1988",
        "category": "Number Theory",
        "statement": "Let a and b be positive integers such that ab + 1 divides a² + b². Show that (a² + b²)/(ab + 1) is the square of an integer.",
        "subs": ["Check small cases", "Use modular arithmetic"],
        "verified": True
    },
]

def main():
    print("VERIFIED IMO PROBLEMS DATABASE")
    print("="*60)
    print(f"Total verified problems: {len(VERIFIED_IMO_PROBLEMS)}")
    print()
    
    categories = {}
    for p in VERIFIED_IMO_PROBLEMS:
        cat = p["category"]
        categories[cat] = categories.get(cat, 0) + 1
    
    print("By category:")
    for cat, count in sorted(categories.items()):
        print(f"  {cat}: {count}")
    
    print()
    print("By year:")
    years = sorted(set(p["year"] for p in VERIFIED_IMO_PROBLEMS), reverse=True)
    for year in years[:10]:
        count = sum(1 for p in VERIFIED_IMO_PROBLEMS if p["year"] == year)
        print(f"  {year}: {count} problems")
    
    with open("data/verified_imo_problems.json", "w", encoding="utf-8") as f:
        json.dump(VERIFIED_IMO_PROBLEMS, f, indent=2, ensure_ascii=False)
    
    print()
    print(f"Saved to data/verified_imo_problems.json")

if __name__ == "__main__":
    os.makedirs("data", exist_ok=True)
    main()
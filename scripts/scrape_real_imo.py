"""
Scrape REAL IMO Problems from Art of Problem Solving Wiki
"""

import json
import time
import urllib.request
import re

BASE_URL = "https://artofproblemsolving.com/wiki/index.php"

# Years with 6 problems each (1959-2024, excluding 1980)
YEARS = list(range(1959, 2025))
YEARS.remove(1980)  # No IMO in 1980

def fetch_problem(year, problem_num):
    """Fetch a single IMO problem from AoPS Wiki."""
    url = f"{BASE_URL}/{year}_IMO_Problems/Problem_{problem_num}"
    try:
        with urllib.request.urlopen(url, timeout=10) as response:
            html = response.read().decode('utf-8')
            
        # Extract problem text between "Problem" header and "Solution" header
        # This is a simplified extraction - may need refinement
        match = re.search(r'<h2[^>]*>.*?Problem.*?</h2>(.*?)<h2', html, re.DOTALL | re.IGNORECASE)
        if match:
            text = match.group(1)
            # Clean HTML tags
            text = re.sub(r'<[^>]+>', ' ', text)
            text = re.sub(r'\s+', ' ', text).strip()
            # Remove LaTeX artifacts
            text = text.replace('\\\\', ' ').replace('\\', '')
            return text[:1000]  # Limit length
        return None
    except Exception as e:
        print(f"Error fetching {year} P{problem_num}: {e}")
        return None

def classify_problem(text):
    """Classify problem type and suggest substitutions."""
    text_lower = text.lower()
    
    if 'function' in text_lower or 'f(' in text_lower:
        category = "Functional Equation"
        subs = ["x = 0", "y = 0", "x = y"]
        if 'injective' in text_lower:
            subs.append("Assume f is injective")
        if 'linear' in text_lower:
            subs.append("Assume f is linear")
    elif 'prove' in text_lower and ('>' in text or '<' in text or 'geq' in text_lower or 'leq' in text_lower):
        category = "Algebra"
        subs = ["Apply AM-GM", "Apply Cauchy-Schwarz"]
        if 'abc' in text_lower and '1' in text:
            subs.append("abc = 1 constraint")
        subs.append("a = b = c = 1")
    elif 'integer' in text_lower or 'divis' in text_lower or 'prime' in text_lower or 'mod' in text_lower:
        category = "Number Theory"
        subs = ["Check small cases", "Use modular arithmetic"]
        if 'prime' in text_lower:
            subs.append("Consider p = 2 separately")
    elif 'triangle' in text_lower or 'circle' in text_lower or 'point' in text_lower:
        category = "Geometry"
        subs = ["Check small cases"]
    else:
        category = "Combinatorics"
        subs = ["Check small cases"]
    
    return category, subs

def main():
    problems = []
    
    print("Fetching real IMO problems from AoPS Wiki...")
    print("This will take a while (1 request per second to be polite)")
    
    for year in YEARS:
        # Most years have 6 problems, early years had more
        max_problems = 7 if year <= 1963 else 6
        
        for pnum in range(1, max_problems + 1):
            print(f"  {year} Problem {pnum}...", end=" ")
            
            text = fetch_problem(year, pnum)
            if text and len(text) > 50:
                category, subs = classify_problem(text)
                problems.append({
                    "year": year,
                    "problem": pnum,
                    "source": f"IMO {year} P{pnum}",
                    "category": category,
                    "statement": text,
                    "subs": subs,
                    "verified": True
                })
                print(f"OK ({len(text)} chars)")
            else:
                print("SKIP")
            
            time.sleep(1)  # Be polite to the server
        
        # Save progress every 5 years
        if year % 5 == 0:
            with open("data/imo_problems_progress.json", "w", encoding="utf-8") as f:
                json.dump(problems, f, indent=2, ensure_ascii=False)
            print(f"  Progress saved: {len(problems)} problems")
    
    # Final save
    with open("data/real_imo_problems.json", "w", encoding="utf-8") as f:
        json.dump(problems, f, indent=2, ensure_ascii=False)
    
    print(f"\nDone! Fetched {len(problems)} real IMO problems")
    print(f"Saved to data/real_imo_problems.json")
    
    # Summary
    categories = {}
    for p in problems:
        c = p["category"]
        categories[c] = categories.get(c, 0) + 1
    
    print("\nBy category:")
    for c, n in sorted(categories.items()):
        print(f"  {c}: {n}")

if __name__ == "__main__":
    main()

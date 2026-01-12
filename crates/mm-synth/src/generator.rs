//! Core problem generator framework

use rand::Rng;
use serde::{Deserialize, Serialize};

/// A synthetically generated problem with solution trace
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SyntheticProblem {
    /// Problem statement
    pub statement: String,
    /// Problem category
    pub category: ProblemCategory,
    /// Solution steps (proof trace)
    pub solution_steps: Vec<SolutionStep>,
    /// Suggested substitutions (labels for training)
    pub substitutions: Vec<String>,
    /// Difficulty estimate (1-10)
    pub difficulty: u8,
}

/// Problem categories
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProblemCategory {
    FunctionalEquation,
    Algebra,
    Inequality,
    NumberTheory,
    Combinatorics,
}

/// A single step in the solution
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SolutionStep {
    /// Action taken (substitution, rule application, etc.)
    pub action: String,
    /// Result after this step
    pub result: String,
    /// Rule or technique used
    pub technique: String,
}

/// Configuration for problem generation
#[derive(Clone, Debug)]
pub struct GeneratorConfig {
    /// Number of problems to generate
    pub num_problems: usize,
    /// Random seed for reproducibility
    pub seed: u64,
    /// Maximum expression depth
    pub max_depth: usize,
    /// Categories to generate
    pub categories: Vec<ProblemCategory>,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        GeneratorConfig {
            num_problems: 100_000,
            seed: 42,
            max_depth: 5,
            categories: vec![
                ProblemCategory::FunctionalEquation,
                ProblemCategory::Algebra,
                ProblemCategory::Inequality,
                ProblemCategory::NumberTheory,
            ],
        }
    }
}

/// Main problem generator
pub struct ProblemGenerator {
    config: GeneratorConfig,
    rng: rand::rngs::StdRng,
}

impl ProblemGenerator {
    /// Create a new generator with config
    pub fn new(config: GeneratorConfig) -> Self {
        use rand::SeedableRng;
        ProblemGenerator {
            rng: rand::rngs::StdRng::seed_from_u64(config.seed),
            config,
        }
    }

    /// Generate all problems
    pub fn generate_all(&mut self) -> Vec<SyntheticProblem> {
        let mut problems = Vec::with_capacity(self.config.num_problems);

        for i in 0..self.config.num_problems {
            let category = &self.config.categories[i % self.config.categories.len()];
            let problem = match category {
                ProblemCategory::FunctionalEquation => self.gen_functional_equation(),
                ProblemCategory::Algebra => self.gen_algebra(),
                ProblemCategory::Inequality => self.gen_inequality(),
                ProblemCategory::NumberTheory => self.gen_number_theory(),
                ProblemCategory::Combinatorics => self.gen_combinatorics(),
            };
            problems.push(problem);

            if i % 10000 == 0 && i > 0 {
                eprintln!("Generated {} problems...", i);
            }
        }

        problems
    }

    /// Generate a functional equation problem
    fn gen_functional_equation(&mut self) -> SyntheticProblem {
        let templates = [
            (
                "f(x + y) = f(x) + f(y)",
                vec!["x = 0", "y = 0", "Assume f is linear"],
            ),
            ("f(xy) = f(x)f(y)", vec!["x = 0", "y = 0", "x = 1"]),
            ("f(x + f(y)) = f(x) + y", vec!["x = 0", "y = 0", "x = y"]),
            ("f(f(x)) = x", vec!["x = 0", "Assume f is injective"]),
            (
                "f(x + y) = f(x)f(y)",
                vec!["x = 0", "y = 0", "Assume f is linear"],
            ),
            ("f(xy) = xf(y) + yf(x)", vec!["x = 1", "y = 1", "x = y"]),
            ("f(x)f(y) = f(xy) + f(x/y)", vec!["x = y", "x = 1"]),
            (
                "f(x^2 + f(y)) = y + f(x)^2",
                vec!["x = 0", "y = 0", "Assume f is injective"],
            ),
            (
                "f(x + y) + f(xy) = f(x)f(y) + 1",
                vec!["x = 0", "y = 0", "x = 1"],
            ),
            (
                "xf(x) - yf(y) = (x-y)f(x+y)",
                vec!["x = 0", "y = 0", "x = y"],
            ),
        ];

        let idx = self.rng.gen_range(0..templates.len());
        let (equation, subs) = &templates[idx];

        // Generate domain variations
        let domains = ["R", "R+", "Q", "Z", "N"];
        let domain = domains[self.rng.gen_range(0..domains.len())];

        let statement = format!(
            "Find all functions f: {} → {} such that {} for all x, y.",
            domain, domain, equation
        );

        SyntheticProblem {
            statement,
            category: ProblemCategory::FunctionalEquation,
            solution_steps: vec![SolutionStep {
                action: subs[0].to_string(),
                result: "Simplified form".to_string(),
                technique: "Substitution".to_string(),
            }],
            substitutions: subs.iter().map(|s| s.to_string()).collect(),
            difficulty: self.rng.gen_range(3..8),
        }
    }

    /// Generate an algebra problem
    fn gen_algebra(&mut self) -> SyntheticProblem {
        let a = self.rng.gen_range(2..6);
        let b = self.rng.gen_range(2..6);

        let templates = [
            format!(
                "Prove that a^{} + b^{} >= a^{}b + ab^{} for positive a, b.",
                a,
                b,
                a - 1,
                b - 1
            ),
            format!(
                "Find all real x satisfying x^{} - {}x + {} = 0.",
                a,
                a + b,
                b
            ),
            format!("Prove (a+b)^{} <= {}(a^{} + b^{}).", a, 1 << (a - 1), a, a),
        ];

        let statement = templates[self.rng.gen_range(0..templates.len())].clone();

        SyntheticProblem {
            statement,
            category: ProblemCategory::Algebra,
            solution_steps: vec![],
            substitutions: vec!["Apply AM-GM".to_string(), "x = y".to_string()],
            difficulty: self.rng.gen_range(2..7),
        }
    }

    /// Generate an inequality problem
    fn gen_inequality(&mut self) -> SyntheticProblem {
        let templates = [
            (
                "a + b + c >= 3",
                "abc = 1",
                vec!["abc = 1 constraint", "Apply AM-GM", "a = b = c = 1"],
            ),
            (
                "a^2 + b^2 + c^2 >= ab + bc + ca",
                "positive reals",
                vec!["Apply Cauchy-Schwarz"],
            ),
            (
                "1/a + 1/b + 1/c >= 9/(a+b+c)",
                "positive a,b,c",
                vec!["Apply Cauchy-Schwarz", "Apply AM-GM"],
            ),
            (
                "(a+b)(b+c)(c+a) >= 8abc",
                "positive a,b,c",
                vec!["Apply AM-GM"],
            ),
            (
                "a/(b+c) + b/(c+a) + c/(a+b) >= 3/2",
                "positive a,b,c",
                vec!["Apply Cauchy-Schwarz", "Apply AM-GM"],
            ),
        ];

        let idx = self.rng.gen_range(0..templates.len());
        let (ineq, cond, subs) = &templates[idx];

        let statement = format!("For {} with {}, prove {}.", cond, cond, ineq);

        SyntheticProblem {
            statement,
            category: ProblemCategory::Inequality,
            solution_steps: vec![],
            substitutions: subs.iter().map(|s| s.to_string()).collect(),
            difficulty: self.rng.gen_range(3..8),
        }
    }

    /// Generate a number theory problem
    fn gen_number_theory(&mut self) -> SyntheticProblem {
        let n = self.rng.gen_range(3..12);
        let m = self.rng.gen_range(2..10);

        let templates = [
            format!(
                "Prove that n^{} - n is divisible by {} for all integers n.",
                n,
                n.max(6)
            ),
            format!(
                "Find the remainder when {}^{} is divided by {}.",
                m,
                n * 10,
                n + 2
            ),
            format!("Find all primes p such that p divides {}^p + {}.", m, m + 1),
            format!(
                "Prove gcd(n^{} - 1, n^{} - 1) = n^gcd({},{}) - 1.",
                n, m, n, m
            ),
        ];

        let statement = templates[self.rng.gen_range(0..templates.len())].clone();

        SyntheticProblem {
            statement,
            category: ProblemCategory::NumberTheory,
            solution_steps: vec![],
            substitutions: vec![
                "Check small cases".to_string(),
                "Use modular arithmetic".to_string(),
            ],
            difficulty: self.rng.gen_range(3..8),
        }
    }

    /// Generate a combinatorics problem
    fn gen_combinatorics(&mut self) -> SyntheticProblem {
        let n = self.rng.gen_range(4..12);

        let statement = format!(
            "Count the number of ways to arrange {} distinct objects such that no two adjacent objects are consecutive integers.",
            n
        );

        SyntheticProblem {
            statement,
            category: ProblemCategory::Combinatorics,
            solution_steps: vec![],
            substitutions: vec!["Check small cases".to_string()],
            difficulty: self.rng.gen_range(4..9),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_basic() {
        let config = GeneratorConfig {
            num_problems: 100,
            ..Default::default()
        };
        let mut gen = ProblemGenerator::new(config);
        let problems = gen.generate_all();

        assert_eq!(problems.len(), 100);
        assert!(!problems[0].statement.is_empty());
    }
}

//! Comprehensive synthetic training data generation.
//!
//! Generates 10K+ training examples covering all 29 rules:
//! - Algebra: constant folding, identities, distribution, factoring
//! - Calculus: power, sum, product, quotient, chain rules
//! - Trig: sin/cos derivatives, identities
//! - Equations: linear, quadratic solving

use candle_core::Device;
use mm_core::{Expr, Rational, SymbolTable};
use rand::prelude::*;

use crate::encoder::ExpressionEncoder;
use crate::training::TrainingExample;

/// Rule ID constants matching those in mm-rules
mod rule_ids {
    // Algebra rules (1-10)
    pub const CONST_FOLD: u32 = 0;
    pub const IDENTITY_ADD_ZERO: u32 = 1;
    pub const IDENTITY_MUL_ONE: u32 = 2;
    pub const ZERO_MUL: u32 = 3;
    pub const COLLECT_LIKE_TERMS: u32 = 4;
    pub const DISTRIBUTE: u32 = 5;
    pub const FACTOR_COMMON: u32 = 6;
    pub const DIFF_OF_SQUARES: u32 = 7;
    pub const PERFECT_SQUARE_SUM: u32 = 8;
    pub const PERFECT_SQUARE_DIFF: u32 = 9;

    // Calculus rules (10-18)
    pub const POWER_RULE: u32 = 10;
    pub const CONSTANT_RULE: u32 = 11;
    pub const SUM_RULE: u32 = 12;
    pub const PRODUCT_RULE: u32 = 13;
    pub const QUOTIENT_RULE: u32 = 14;
    pub const SIN_DERIVATIVE: u32 = 15;
    pub const COS_DERIVATIVE: u32 = 16;
    pub const EXP_DERIVATIVE: u32 = 17;
    pub const LN_DERIVATIVE: u32 = 18;

    // Trig rules (19-20)
    pub const PYTHAGOREAN: u32 = 19;
    pub const DOUBLE_ANGLE_SIN: u32 = 20;

    // Equation rules (21-27)
    pub const ISOLATE_VARIABLE: u32 = 21;
    pub const CANCEL_ADDITION: u32 = 22;
    pub const CANCEL_SUBTRACTION: u32 = 23;
    pub const CANCEL_MULTIPLICATION: u32 = 24;
    pub const CANCEL_DIVISION: u32 = 25;
    pub const LINEAR_SOLVE: u32 = 26;
    pub const QUADRATIC_FORMULA: u32 = 27;

    // No-op for negative examples
    pub const NO_OP: u32 = 28;
}

/// Generator for comprehensive synthetic training data.
pub struct DataGenerator {
    encoder: ExpressionEncoder,
    symbols: SymbolTable,
    rng: StdRng,
    x: mm_core::Symbol,
    y: mm_core::Symbol,
    z: mm_core::Symbol,
}

impl DataGenerator {
    /// Create a new data generator.
    pub fn new(device: Device) -> Self {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        let y = symbols.intern("y");
        let z = symbols.intern("z");

        Self {
            encoder: ExpressionEncoder::new(device),
            symbols,
            rng: StdRng::seed_from_u64(42),
            x,
            y,
            z,
        }
    }

    /// Create with a specific random seed.
    pub fn with_seed(device: Device, seed: u64) -> Self {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        let y = symbols.intern("y");
        let z = symbols.intern("z");

        Self {
            encoder: ExpressionEncoder::new(device),
            symbols,
            rng: StdRng::seed_from_u64(seed),
            x,
            y,
            z,
        }
    }

    fn make_example(&self, expr: &Expr, rule: u32, value: f32) -> TrainingExample {
        let tokens = self.encoder.encode_tokens(&self.encoder.tokenize(expr));
        TrainingExample {
            tokens,
            target_rule: rule,
            target_value: value,
        }
    }

    fn rand_small(&mut self) -> i64 {
        self.rng.gen_range(1..15)
    }

    fn rand_nonzero(&mut self) -> i64 {
        let v = self.rng.gen_range(1..20);
        if self.rng.gen_bool(0.5) {
            v
        } else {
            -v
        }
    }

    // =========================================================================
    // ALGEBRA RULES
    // =========================================================================

    /// Constant folding: 2 + 3 → 5, 4 * 5 → 20
    pub fn generate_constant_folding(&mut self, count: usize) -> Vec<TrainingExample> {
        let mut examples = Vec::new();

        for _ in 0..count {
            let a = self.rand_small();
            let b = self.rand_small();

            // Addition
            examples.push(self.make_example(
                &Expr::Add(Box::new(Expr::int(a)), Box::new(Expr::int(b))),
                rule_ids::CONST_FOLD,
                1.0,
            ));

            // Subtraction
            examples.push(self.make_example(
                &Expr::Sub(Box::new(Expr::int(a + b)), Box::new(Expr::int(b))),
                rule_ids::CONST_FOLD,
                1.0,
            ));

            // Multiplication
            examples.push(self.make_example(
                &Expr::Mul(Box::new(Expr::int(a)), Box::new(Expr::int(b))),
                rule_ids::CONST_FOLD,
                1.0,
            ));

            // Division (avoid zero)
            if b != 0 {
                examples.push(self.make_example(
                    &Expr::Div(Box::new(Expr::int(a * b)), Box::new(Expr::int(b))),
                    rule_ids::CONST_FOLD,
                    1.0,
                ));
            }

            // Power
            let exp = self.rng.gen_range(0..5);
            examples.push(self.make_example(
                &Expr::Pow(Box::new(Expr::int(2)), Box::new(Expr::int(exp))),
                rule_ids::CONST_FOLD,
                1.0,
            ));
        }

        examples
    }

    /// Identity rules: x + 0 → x, x * 1 → x, x * 0 → 0
    pub fn generate_identity_rules(&mut self, count: usize) -> Vec<TrainingExample> {
        let mut examples = Vec::new();
        let vars = [self.x, self.y, self.z];

        for i in 0..count {
            let v = vars[i % 3];

            // x + 0 → x
            examples.push(self.make_example(
                &Expr::Add(Box::new(Expr::Var(v)), Box::new(Expr::int(0))),
                rule_ids::IDENTITY_ADD_ZERO,
                1.0,
            ));

            // 0 + x → x
            examples.push(self.make_example(
                &Expr::Add(Box::new(Expr::int(0)), Box::new(Expr::Var(v))),
                rule_ids::IDENTITY_ADD_ZERO,
                1.0,
            ));

            // x * 1 → x
            examples.push(self.make_example(
                &Expr::Mul(Box::new(Expr::Var(v)), Box::new(Expr::int(1))),
                rule_ids::IDENTITY_MUL_ONE,
                1.0,
            ));

            // 1 * x → x
            examples.push(self.make_example(
                &Expr::Mul(Box::new(Expr::int(1)), Box::new(Expr::Var(v))),
                rule_ids::IDENTITY_MUL_ONE,
                1.0,
            ));

            // x * 0 → 0
            examples.push(self.make_example(
                &Expr::Mul(Box::new(Expr::Var(v)), Box::new(Expr::int(0))),
                rule_ids::ZERO_MUL,
                1.0,
            ));

            // 0 * x → 0
            examples.push(self.make_example(
                &Expr::Mul(Box::new(Expr::int(0)), Box::new(Expr::Var(v))),
                rule_ids::ZERO_MUL,
                1.0,
            ));
        }

        examples
    }

    /// Distribution: a(b + c) → ab + ac
    pub fn generate_distribute(&mut self, count: usize) -> Vec<TrainingExample> {
        let mut examples = Vec::new();

        for _ in 0..count {
            let a = self.rand_small();

            // a * (x + y)
            examples.push(self.make_example(
                &Expr::Mul(
                    Box::new(Expr::int(a)),
                    Box::new(Expr::Add(
                        Box::new(Expr::Var(self.x)),
                        Box::new(Expr::Var(self.y)),
                    )),
                ),
                rule_ids::DISTRIBUTE,
                1.0,
            ));

            // (x + y) * a
            examples.push(self.make_example(
                &Expr::Mul(
                    Box::new(Expr::Add(
                        Box::new(Expr::Var(self.x)),
                        Box::new(Expr::Var(self.y)),
                    )),
                    Box::new(Expr::int(a)),
                ),
                rule_ids::DISTRIBUTE,
                1.0,
            ));

            // a * (x - y)
            examples.push(self.make_example(
                &Expr::Mul(
                    Box::new(Expr::int(a)),
                    Box::new(Expr::Add(
                        Box::new(Expr::Var(self.x)),
                        Box::new(Expr::Neg(Box::new(Expr::Var(self.y)))),
                    )),
                ),
                rule_ids::DISTRIBUTE,
                1.0,
            ));
        }

        examples
    }

    /// Factor common: ab + ac → a(b + c)
    pub fn generate_factor_common(&mut self, count: usize) -> Vec<TrainingExample> {
        let mut examples = Vec::new();

        for _ in 0..count {
            let a = self.rand_small();

            // ax + ay (common factor a)
            examples.push(self.make_example(
                &Expr::Add(
                    Box::new(Expr::Mul(
                        Box::new(Expr::int(a)),
                        Box::new(Expr::Var(self.x)),
                    )),
                    Box::new(Expr::Mul(
                        Box::new(Expr::int(a)),
                        Box::new(Expr::Var(self.y)),
                    )),
                ),
                rule_ids::FACTOR_COMMON,
                1.0,
            ));
        }

        examples
    }

    /// Difference of squares: a² - b² → (a+b)(a-b)
    pub fn generate_difference_of_squares(&mut self, count: usize) -> Vec<TrainingExample> {
        let mut examples = Vec::new();

        for _ in 0..count {
            // x² - y²
            examples.push(self.make_example(
                &Expr::Sub(
                    Box::new(Expr::Pow(
                        Box::new(Expr::Var(self.x)),
                        Box::new(Expr::int(2)),
                    )),
                    Box::new(Expr::Pow(
                        Box::new(Expr::Var(self.y)),
                        Box::new(Expr::int(2)),
                    )),
                ),
                rule_ids::DIFF_OF_SQUARES,
                1.0,
            ));

            // x² - 4 (where 4 = 2²)
            let a = self.rand_small();
            examples.push(self.make_example(
                &Expr::Sub(
                    Box::new(Expr::Pow(
                        Box::new(Expr::Var(self.x)),
                        Box::new(Expr::int(2)),
                    )),
                    Box::new(Expr::Pow(Box::new(Expr::int(a)), Box::new(Expr::int(2)))),
                ),
                rule_ids::DIFF_OF_SQUARES,
                1.0,
            ));
        }

        examples
    }

    /// Collect like terms: ax + bx → (a+b)x
    pub fn generate_collect_like_terms(&mut self, count: usize) -> Vec<TrainingExample> {
        let mut examples = Vec::new();

        for _ in 0..count {
            let a = self.rand_small();
            let b = self.rand_small();

            // ax + bx
            examples.push(self.make_example(
                &Expr::Add(
                    Box::new(Expr::Mul(
                        Box::new(Expr::int(a)),
                        Box::new(Expr::Var(self.x)),
                    )),
                    Box::new(Expr::Mul(
                        Box::new(Expr::int(b)),
                        Box::new(Expr::Var(self.x)),
                    )),
                ),
                rule_ids::COLLECT_LIKE_TERMS,
                1.0,
            ));

            // x + x → 2x
            examples.push(self.make_example(
                &Expr::Add(Box::new(Expr::Var(self.x)), Box::new(Expr::Var(self.x))),
                rule_ids::COLLECT_LIKE_TERMS,
                1.0,
            ));
        }

        examples
    }

    // =========================================================================
    // CALCULUS RULES
    // =========================================================================

    /// Power rule: d/dx(x^n) = n*x^(n-1)
    pub fn generate_power_rule(&mut self, count: usize) -> Vec<TrainingExample> {
        let mut examples = Vec::new();

        for i in 0..count {
            let n = (i % 10) as i64 + 1;

            // d/dx(x^n)
            examples.push(self.make_example(
                &Expr::Derivative {
                    expr: Box::new(Expr::Pow(
                        Box::new(Expr::Var(self.x)),
                        Box::new(Expr::int(n)),
                    )),
                    var: self.x,
                },
                rule_ids::POWER_RULE,
                1.0,
            ));

            // d/dx(x) = 1
            examples.push(self.make_example(
                &Expr::Derivative {
                    expr: Box::new(Expr::Var(self.x)),
                    var: self.x,
                },
                rule_ids::POWER_RULE,
                1.0,
            ));
        }

        examples
    }

    /// Constant rule: d/dx(c) = 0
    pub fn generate_constant_rule(&mut self, count: usize) -> Vec<TrainingExample> {
        let mut examples = Vec::new();

        for _ in 0..count {
            let c = self.rand_small();

            examples.push(self.make_example(
                &Expr::Derivative {
                    expr: Box::new(Expr::int(c)),
                    var: self.x,
                },
                rule_ids::CONSTANT_RULE,
                1.0,
            ));

            // d/dy(x) where y is different variable
            examples.push(self.make_example(
                &Expr::Derivative {
                    expr: Box::new(Expr::Var(self.x)),
                    var: self.y,
                },
                rule_ids::CONSTANT_RULE,
                1.0,
            ));
        }

        examples
    }

    /// Sum rule: d/dx(f + g) = f' + g'
    pub fn generate_sum_rule(&mut self, count: usize) -> Vec<TrainingExample> {
        let mut examples = Vec::new();

        for i in 0..count {
            let n = (i % 5) as i64 + 1;
            let m = (i % 4) as i64 + 2;

            // d/dx(x^n + x^m)
            examples.push(self.make_example(
                &Expr::Derivative {
                    expr: Box::new(Expr::Add(
                        Box::new(Expr::Pow(
                            Box::new(Expr::Var(self.x)),
                            Box::new(Expr::int(n)),
                        )),
                        Box::new(Expr::Pow(
                            Box::new(Expr::Var(self.x)),
                            Box::new(Expr::int(m)),
                        )),
                    )),
                    var: self.x,
                },
                rule_ids::SUM_RULE,
                1.0,
            ));

            // d/dx(x + c)
            let c = self.rand_small();
            examples.push(self.make_example(
                &Expr::Derivative {
                    expr: Box::new(Expr::Add(
                        Box::new(Expr::Var(self.x)),
                        Box::new(Expr::int(c)),
                    )),
                    var: self.x,
                },
                rule_ids::SUM_RULE,
                1.0,
            ));
        }

        examples
    }

    /// Product rule: d/dx(fg) = f'g + fg'
    pub fn generate_product_rule(&mut self, count: usize) -> Vec<TrainingExample> {
        let mut examples = Vec::new();

        for i in 0..count {
            let n = (i % 5) as i64 + 1;

            // d/dx(x * x^n) = d/dx(x^(n+1))
            examples.push(self.make_example(
                &Expr::Derivative {
                    expr: Box::new(Expr::Mul(
                        Box::new(Expr::Var(self.x)),
                        Box::new(Expr::Pow(
                            Box::new(Expr::Var(self.x)),
                            Box::new(Expr::int(n)),
                        )),
                    )),
                    var: self.x,
                },
                rule_ids::PRODUCT_RULE,
                1.0,
            ));

            // d/dx(x * sin(x))
            examples.push(self.make_example(
                &Expr::Derivative {
                    expr: Box::new(Expr::Mul(
                        Box::new(Expr::Var(self.x)),
                        Box::new(Expr::Sin(Box::new(Expr::Var(self.x)))),
                    )),
                    var: self.x,
                },
                rule_ids::PRODUCT_RULE,
                1.0,
            ));
        }

        examples
    }

    /// Quotient rule: d/dx(f/g) = (f'g - fg')/g²
    pub fn generate_quotient_rule(&mut self, count: usize) -> Vec<TrainingExample> {
        let mut examples = Vec::new();

        for i in 0..count {
            let n = (i % 5) as i64 + 2;

            // d/dx(x / x^n)
            examples.push(self.make_example(
                &Expr::Derivative {
                    expr: Box::new(Expr::Div(
                        Box::new(Expr::Var(self.x)),
                        Box::new(Expr::Pow(
                            Box::new(Expr::Var(self.x)),
                            Box::new(Expr::int(n)),
                        )),
                    )),
                    var: self.x,
                },
                rule_ids::QUOTIENT_RULE,
                1.0,
            ));

            // d/dx(1 / x)
            examples.push(self.make_example(
                &Expr::Derivative {
                    expr: Box::new(Expr::Div(
                        Box::new(Expr::int(1)),
                        Box::new(Expr::Var(self.x)),
                    )),
                    var: self.x,
                },
                rule_ids::QUOTIENT_RULE,
                1.0,
            ));

            // d/dx(x / (x + 1))
            examples.push(self.make_example(
                &Expr::Derivative {
                    expr: Box::new(Expr::Div(
                        Box::new(Expr::Var(self.x)),
                        Box::new(Expr::Add(
                            Box::new(Expr::Var(self.x)),
                            Box::new(Expr::int(1)),
                        )),
                    )),
                    var: self.x,
                },
                rule_ids::QUOTIENT_RULE,
                1.0,
            ));
        }

        examples
    }

    /// Trig derivatives: d/dx(sin(x)) = cos(x), d/dx(cos(x)) = -sin(x)
    pub fn generate_trig_derivatives(&mut self, count: usize) -> Vec<TrainingExample> {
        let mut examples = Vec::new();

        for _ in 0..count {
            // d/dx(sin(x)) = cos(x)
            examples.push(self.make_example(
                &Expr::Derivative {
                    expr: Box::new(Expr::Sin(Box::new(Expr::Var(self.x)))),
                    var: self.x,
                },
                rule_ids::SIN_DERIVATIVE,
                1.0,
            ));

            // d/dx(cos(x)) = -sin(x)
            examples.push(self.make_example(
                &Expr::Derivative {
                    expr: Box::new(Expr::Cos(Box::new(Expr::Var(self.x)))),
                    var: self.x,
                },
                rule_ids::COS_DERIVATIVE,
                1.0,
            ));

            // d/dx(exp(x)) = exp(x)
            examples.push(self.make_example(
                &Expr::Derivative {
                    expr: Box::new(Expr::Exp(Box::new(Expr::Var(self.x)))),
                    var: self.x,
                },
                rule_ids::EXP_DERIVATIVE,
                1.0,
            ));

            // d/dx(ln(x)) = 1/x
            examples.push(self.make_example(
                &Expr::Derivative {
                    expr: Box::new(Expr::Ln(Box::new(Expr::Var(self.x)))),
                    var: self.x,
                },
                rule_ids::LN_DERIVATIVE,
                1.0,
            ));
        }

        examples
    }

    // =========================================================================
    // EQUATION SOLVING RULES
    // =========================================================================

    /// Cancel addition: x + a = b → x = b - a
    pub fn generate_equation_addition(&mut self, count: usize) -> Vec<TrainingExample> {
        let mut examples = Vec::new();

        for _ in 0..count {
            let a = self.rand_nonzero();
            let b = self.rand_nonzero();

            // x + a = b
            examples.push(self.make_example(
                &Expr::Equation {
                    lhs: Box::new(Expr::Add(
                        Box::new(Expr::Var(self.x)),
                        Box::new(Expr::int(a)),
                    )),
                    rhs: Box::new(Expr::int(b)),
                },
                rule_ids::CANCEL_ADDITION,
                1.0,
            ));

            // a + x = b
            examples.push(self.make_example(
                &Expr::Equation {
                    lhs: Box::new(Expr::Add(
                        Box::new(Expr::int(a)),
                        Box::new(Expr::Var(self.x)),
                    )),
                    rhs: Box::new(Expr::int(b)),
                },
                rule_ids::CANCEL_ADDITION,
                1.0,
            ));
        }

        examples
    }

    /// Cancel subtraction: x - a = b → x = b + a
    pub fn generate_equation_subtraction(&mut self, count: usize) -> Vec<TrainingExample> {
        let mut examples = Vec::new();

        for _ in 0..count {
            let a = self.rand_nonzero();
            let b = self.rand_nonzero();

            // x - a = b
            examples.push(self.make_example(
                &Expr::Equation {
                    lhs: Box::new(Expr::Sub(
                        Box::new(Expr::Var(self.x)),
                        Box::new(Expr::int(a)),
                    )),
                    rhs: Box::new(Expr::int(b)),
                },
                rule_ids::CANCEL_SUBTRACTION,
                1.0,
            ));
        }

        examples
    }

    /// Cancel multiplication: ax = b → x = b/a
    pub fn generate_equation_multiplication(&mut self, count: usize) -> Vec<TrainingExample> {
        let mut examples = Vec::new();

        for _ in 0..count {
            let a = self.rand_small();
            let b = self.rand_nonzero();

            if a != 0 {
                // ax = b
                examples.push(self.make_example(
                    &Expr::Equation {
                        lhs: Box::new(Expr::Mul(
                            Box::new(Expr::int(a)),
                            Box::new(Expr::Var(self.x)),
                        )),
                        rhs: Box::new(Expr::int(b)),
                    },
                    rule_ids::CANCEL_MULTIPLICATION,
                    1.0,
                ));
            }
        }

        examples
    }

    /// Cancel division: x/a = b → x = ab
    pub fn generate_equation_division(&mut self, count: usize) -> Vec<TrainingExample> {
        let mut examples = Vec::new();

        for _ in 0..count {
            let a = self.rand_small();
            let b = self.rand_nonzero();

            if a != 0 {
                // x/a = b
                examples.push(self.make_example(
                    &Expr::Equation {
                        lhs: Box::new(Expr::Div(
                            Box::new(Expr::Var(self.x)),
                            Box::new(Expr::int(a)),
                        )),
                        rhs: Box::new(Expr::int(b)),
                    },
                    rule_ids::CANCEL_DIVISION,
                    1.0,
                ));
            }
        }

        examples
    }

    /// Linear solve: ax + b = c → x = (c-b)/a
    pub fn generate_linear_equations(&mut self, count: usize) -> Vec<TrainingExample> {
        let mut examples = Vec::new();

        for _ in 0..count {
            let a = self.rand_small();
            let b = self.rand_nonzero();
            let c = self.rand_nonzero();

            if a != 0 {
                // ax + b = c
                examples.push(self.make_example(
                    &Expr::Equation {
                        lhs: Box::new(Expr::Add(
                            Box::new(Expr::Mul(
                                Box::new(Expr::int(a)),
                                Box::new(Expr::Var(self.x)),
                            )),
                            Box::new(Expr::int(b)),
                        )),
                        rhs: Box::new(Expr::int(c)),
                    },
                    rule_ids::LINEAR_SOLVE,
                    1.0,
                ));

                // ax - b = c
                examples.push(self.make_example(
                    &Expr::Equation {
                        lhs: Box::new(Expr::Sub(
                            Box::new(Expr::Mul(
                                Box::new(Expr::int(a)),
                                Box::new(Expr::Var(self.x)),
                            )),
                            Box::new(Expr::int(b)),
                        )),
                        rhs: Box::new(Expr::int(c)),
                    },
                    rule_ids::ISOLATE_VARIABLE,
                    1.0,
                ));
            }
        }

        examples
    }

    /// Quadratic equations: ax² + bx + c = 0
    pub fn generate_quadratic_equations(&mut self, count: usize) -> Vec<TrainingExample> {
        let mut examples = Vec::new();

        for _ in 0..count {
            let a = self.rand_small();
            let b = self.rand_nonzero();
            let c = self.rand_nonzero();

            if a != 0 {
                // ax² + bx + c = 0
                examples.push(self.make_example(
                    &Expr::Equation {
                        lhs: Box::new(Expr::Add(
                            Box::new(Expr::Add(
                                Box::new(Expr::Mul(
                                    Box::new(Expr::int(a)),
                                    Box::new(Expr::Pow(
                                        Box::new(Expr::Var(self.x)),
                                        Box::new(Expr::int(2)),
                                    )),
                                )),
                                Box::new(Expr::Mul(
                                    Box::new(Expr::int(b)),
                                    Box::new(Expr::Var(self.x)),
                                )),
                            )),
                            Box::new(Expr::int(c)),
                        )),
                        rhs: Box::new(Expr::int(0)),
                    },
                    rule_ids::QUADRATIC_FORMULA,
                    1.0,
                ));

                // x² = c (simple quadratic)
                examples.push(self.make_example(
                    &Expr::Equation {
                        lhs: Box::new(Expr::Pow(
                            Box::new(Expr::Var(self.x)),
                            Box::new(Expr::int(2)),
                        )),
                        rhs: Box::new(Expr::int(c.abs())),
                    },
                    rule_ids::QUADRATIC_FORMULA,
                    1.0,
                ));
            }
        }

        examples
    }

    // =========================================================================
    // NEGATIVE EXAMPLES
    // =========================================================================

    /// Negative examples: expressions that don't simplify
    pub fn generate_negative_examples(&mut self, count: usize) -> Vec<TrainingExample> {
        let mut examples = Vec::new();

        for _ in 0..count {
            let n = self.rand_small();

            // x + n (doesn't simplify when n ≠ 0)
            if n != 0 {
                examples.push(self.make_example(
                    &Expr::Add(Box::new(Expr::Var(self.x)), Box::new(Expr::int(n))),
                    rule_ids::NO_OP,
                    -0.3,
                ));
            }

            // x * n (doesn't simplify when n ≠ 0, 1)
            if n > 1 {
                examples.push(self.make_example(
                    &Expr::Mul(Box::new(Expr::Var(self.x)), Box::new(Expr::int(n))),
                    rule_ids::NO_OP,
                    -0.3,
                ));
            }

            // x^n (doesn't simplify when n > 1)
            if n > 1 {
                examples.push(self.make_example(
                    &Expr::Pow(Box::new(Expr::Var(self.x)), Box::new(Expr::int(n))),
                    rule_ids::NO_OP,
                    -0.2,
                ));
            }

            // sin(x) (already simplified)
            examples.push(self.make_example(
                &Expr::Sin(Box::new(Expr::Var(self.x))),
                rule_ids::NO_OP,
                0.0,
            ));
        }

        examples
    }

    // =========================================================================
    // DATASET GENERATION
    // =========================================================================

    /// Generate a complete 10K training dataset covering all rules.
    pub fn generate_dataset(&mut self, samples_per_category: usize) -> Vec<TrainingExample> {
        let mut all_examples = Vec::new();

        // Algebra (target: ~3000)
        println!("Generating algebra examples...");
        all_examples.extend(self.generate_constant_folding(samples_per_category));
        all_examples.extend(self.generate_identity_rules(samples_per_category));
        all_examples.extend(self.generate_distribute(samples_per_category / 2));
        all_examples.extend(self.generate_factor_common(samples_per_category / 2));
        all_examples.extend(self.generate_difference_of_squares(samples_per_category / 2));
        all_examples.extend(self.generate_collect_like_terms(samples_per_category / 2));

        // Calculus (target: ~3000)
        println!("Generating calculus examples...");
        all_examples.extend(self.generate_power_rule(samples_per_category));
        all_examples.extend(self.generate_constant_rule(samples_per_category));
        all_examples.extend(self.generate_sum_rule(samples_per_category / 2));
        all_examples.extend(self.generate_product_rule(samples_per_category / 2));
        all_examples.extend(self.generate_quotient_rule(samples_per_category / 2));
        all_examples.extend(self.generate_trig_derivatives(samples_per_category));

        // Equations (target: ~3000)
        println!("Generating equation solving examples...");
        all_examples.extend(self.generate_equation_addition(samples_per_category / 2));
        all_examples.extend(self.generate_equation_subtraction(samples_per_category / 2));
        all_examples.extend(self.generate_equation_multiplication(samples_per_category / 2));
        all_examples.extend(self.generate_equation_division(samples_per_category / 2));
        all_examples.extend(self.generate_linear_equations(samples_per_category));
        all_examples.extend(self.generate_quadratic_equations(samples_per_category / 2));

        // Negative examples (target: ~1000)
        println!("Generating negative examples...");
        all_examples.extend(self.generate_negative_examples(samples_per_category / 2));

        println!("Total training examples: {}", all_examples.len());

        // Shuffle
        all_examples.shuffle(&mut self.rng);

        all_examples
    }

    /// Generate a small test dataset for validation.
    pub fn generate_validation_set(&mut self, size: usize) -> Vec<TrainingExample> {
        let per_cat = size / 20;
        self.generate_dataset(per_cat)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_constant_folding() {
        let mut gen = DataGenerator::new(Device::Cpu);
        let examples = gen.generate_constant_folding(10);
        assert!(examples.len() >= 40);
        assert!(examples
            .iter()
            .all(|e| e.target_rule == rule_ids::CONST_FOLD));
    }

    #[test]
    fn test_generate_dataset() {
        let mut gen = DataGenerator::new(Device::Cpu);
        let examples = gen.generate_dataset(50);

        // Should have around 1000+ examples with 50 samples_per_category
        assert!(examples.len() > 500);
        println!("Generated {} examples", examples.len());
    }

    #[test]
    fn test_equation_examples() {
        let mut gen = DataGenerator::new(Device::Cpu);
        let examples = gen.generate_linear_equations(10);
        assert!(!examples.is_empty());
    }

    #[test]
    fn test_quotient_rule_examples() {
        let mut gen = DataGenerator::new(Device::Cpu);
        let examples = gen.generate_quotient_rule(10);
        assert!(examples.len() >= 20);
    }
}

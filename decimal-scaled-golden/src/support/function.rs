// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

/// The functions the golden set covers. `arity` is how many inputs precede
/// the output on a golden file line.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Function {
    // unary
    Sqrt, Cbrt, Exp, Ln, Log2, Log10, Exp2,
    Sin, Cos, Tan, Atan, Asin, Acos,
    Sinh, Cosh, Tanh, Asinh, Acosh, Atanh,
    // binary
    Log, Atan2, Powf, Hypot, Add, Sub, Mul, Div, Rem,
}

impl Function {
    pub fn arity(self) -> usize {
        use Function::*;
        match self {
            Log | Atan2 | Powf | Hypot | Add | Sub | Mul | Div | Rem => 2,
            _ => 1,
        }
    }
    /// Canonical lowercase name, matching the golden file naming.
    pub fn name(self) -> &'static str {
        use Function::*;
        match self {
            Sqrt => "sqrt", Cbrt => "cbrt", Exp => "exp", Ln => "ln",
            Log2 => "log2", Log10 => "log10", Exp2 => "exp2",
            Sin => "sin", Cos => "cos", Tan => "tan",
            Atan => "atan", Asin => "asin", Acos => "acos",
            Sinh => "sinh", Cosh => "cosh", Tanh => "tanh",
            Asinh => "asinh", Acosh => "acosh", Atanh => "atanh",
            Log => "log", Atan2 => "atan2", Powf => "powf", Hypot => "hypot",
            Add => "add", Sub => "sub", Mul => "mul", Div => "div", Rem => "rem",
        }
    }
}

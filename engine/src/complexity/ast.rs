use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DimensionVar {
    N,
    M,
    Q,
    K,
    V,
    E,
    A,
}

impl DimensionVar {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::N => "n",
            Self::M => "m",
            Self::Q => "q",
            Self::K => "k",
            Self::V => "V",
            Self::E => "E",
            Self::A => "A",
        }
    }
}

impl fmt::Display for DimensionVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ComplexityExpr {
    Const(u64),
    Var(DimensionVar),
    Log(Box<ComplexityExpr>),
    Sqrt(Box<ComplexityExpr>),
    Pow(Box<ComplexityExpr>, u32),
    Mul(Box<ComplexityExpr>, Box<ComplexityExpr>),
    Add(Box<ComplexityExpr>, Box<ComplexityExpr>),
    Max(Box<ComplexityExpr>, Box<ComplexityExpr>),
    Unknown,
}

impl ComplexityExpr {
    pub fn one() -> Self {
        Self::Const(1)
    }

    pub fn var(v: DimensionVar) -> Self {
        Self::Var(v)
    }

    pub fn log(e: Self) -> Self {
        Self::Log(Box::new(e))
    }

    pub fn sqrt(e: Self) -> Self {
        Self::Sqrt(Box::new(e))
    }

    pub fn pow(e: Self, exp: u32) -> Self {
        Self::Pow(Box::new(e), exp)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn mul(a: Self, b: Self) -> Self {
        Self::Mul(Box::new(a), Box::new(b))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(a: Self, b: Self) -> Self {
        Self::Add(Box::new(a), Box::new(b))
    }

    pub fn max(a: Self, b: Self) -> Self {
        Self::Max(Box::new(a), Box::new(b))
    }

    pub fn is_const_one(&self) -> bool {
        matches!(self, Self::Const(1))
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

impl fmt::Display for ComplexityExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::Const(c) => write!(f, "O({c})"),
            expr => write!(f, "O({})", expr.format_inner()),
        }
    }
}

impl ComplexityExpr {
    pub fn format_inner(&self) -> String {
        match self {
            Self::Unknown => "Unknown".to_string(),
            Self::Const(c) => format!("{c}"),
            Self::Var(v) => v.as_str().to_string(),
            Self::Log(inner) => format!("log {}", inner.format_inner()),
            Self::Sqrt(inner) => format!("sqrt {}", inner.format_inner()),
            Self::Pow(base, exp) => format!("{}^{exp}", base.format_inner()),
            Self::Mul(lhs, rhs) => {
                let lhs_str = match &**lhs {
                    Self::Add(..) => format!("({})", lhs.format_inner()),
                    _ => lhs.format_inner(),
                };
                let rhs_str = match &**rhs {
                    Self::Add(..) => format!("({})", rhs.format_inner()),
                    _ => rhs.format_inner(),
                };
                if lhs_str == "1" {
                    rhs_str
                } else if rhs_str == "1" {
                    lhs_str
                } else {
                    format!("{lhs_str} {rhs_str}")
                }
            }
            Self::Add(lhs, rhs) => {
                format!("{} + {}", lhs.format_inner(), rhs.format_inner())
            }
            Self::Max(lhs, rhs) => {
                format!("max({}, {})", lhs.format_inner(), rhs.format_inner())
            }
        }
    }
}

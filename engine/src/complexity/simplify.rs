use super::ast::{ComplexityExpr, DimensionVar};
use std::cmp::Ordering;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Monomial {
    pub vars: BTreeMap<DimensionVar, u32>,
    pub logs: BTreeMap<DimensionVar, u32>,
    pub is_log_log: bool,
}

impl Monomial {
    pub fn one() -> Self {
        Self {
            vars: BTreeMap::new(),
            logs: BTreeMap::new(),
            is_log_log: false,
        }
    }

    pub fn from_var(v: DimensionVar) -> Self {
        let mut m = Self::one();
        m.vars.insert(v, 1);
        m
    }

    pub fn from_log(v: DimensionVar) -> Self {
        let mut m = Self::one();
        m.logs.insert(v, 1);
        m
    }

    pub fn mul(&self, other: &Self) -> Self {
        let mut res = self.clone();
        for (v, exp) in &other.vars {
            *res.vars.entry(*v).or_insert(0) += exp;
        }
        for (v, exp) in &other.logs {
            *res.logs.entry(*v).or_insert(0) += exp;
        }
        res.is_log_log = self.is_log_log || other.is_log_log;
        res
    }

    pub fn dominates(&self, other: &Self) -> bool {
        if self == other {
            return true;
        }
        if other.is_one() {
            return !self.is_one();
        }
        if self.is_one() {
            return false;
        }

        // Check if self and other share the exact same set of variables
        let self_var_keys: Vec<_> = self.vars.keys().collect();
        let other_var_keys: Vec<_> = other.vars.keys().collect();

        if self_var_keys == other_var_keys && !self_var_keys.is_empty() {
            let mut strictly_greater = false;
            let mut strictly_lesser = false;

            for v in self_var_keys {
                let s_exp = *self.vars.get(v).unwrap_or(&0);
                let o_exp = *other.vars.get(v).unwrap_or(&0);
                let s_log = *self.logs.get(v).unwrap_or(&0);
                let o_log = *other.logs.get(v).unwrap_or(&0);

                match s_exp.cmp(&o_exp) {
                    Ordering::Greater => strictly_greater = true,
                    Ordering::Less => strictly_lesser = true,
                    Ordering::Equal => match s_log.cmp(&o_log) {
                        Ordering::Greater => strictly_greater = true,
                        Ordering::Less => strictly_lesser = true,
                        Ordering::Equal => {}
                    },
                }
            }

            if strictly_greater && !strictly_lesser {
                return true;
            }
        }

        false
    }

    pub fn is_one(&self) -> bool {
        self.vars.is_empty() && self.logs.is_empty() && !self.is_log_log
    }

    pub fn to_expr(&self) -> ComplexityExpr {
        if self.is_one() {
            return ComplexityExpr::one();
        }

        let mut parts = Vec::new();
        for (v, exp) in &self.vars {
            let base = ComplexityExpr::var(*v);
            if *exp == 1 {
                parts.push(base);
            } else {
                parts.push(ComplexityExpr::pow(base, *exp));
            }
        }
        for (v, exp) in &self.logs {
            let log_expr = ComplexityExpr::log(ComplexityExpr::var(*v));
            if *exp == 1 {
                parts.push(log_expr);
            } else {
                parts.push(ComplexityExpr::pow(log_expr, *exp));
            }
        }

        let mut iter = parts.into_iter();
        let mut res = iter.next().unwrap_or_else(ComplexityExpr::one);
        for item in iter {
            res = ComplexityExpr::mul(res, item);
        }
        res
    }
}

pub fn simplify(expr: &ComplexityExpr) -> ComplexityExpr {
    match expr {
        ComplexityExpr::Unknown => ComplexityExpr::Unknown,
        ComplexityExpr::Const(_) => ComplexityExpr::one(),
        ComplexityExpr::Var(v) => ComplexityExpr::var(*v),
        ComplexityExpr::Log(inner) => {
            let s_inner = simplify(inner);
            match s_inner {
                ComplexityExpr::Const(_) => ComplexityExpr::one(),
                _ => ComplexityExpr::log(s_inner),
            }
        }
        ComplexityExpr::Pow(base, exp) => {
            if *exp == 0 {
                ComplexityExpr::one()
            } else if *exp == 1 {
                simplify(base)
            } else {
                ComplexityExpr::pow(simplify(base), *exp)
            }
        }
        ComplexityExpr::Mul(lhs, rhs) => {
            let sl = simplify(lhs);
            let sr = simplify(rhs);
            if sl.is_unknown() || sr.is_unknown() {
                return ComplexityExpr::Unknown;
            }
            if sl.is_const_one() {
                return sr;
            }
            if sr.is_const_one() {
                return sl;
            }
            ComplexityExpr::mul(sl, sr)
        }
        ComplexityExpr::Add(lhs, rhs) => {
            let sl = simplify(lhs);
            let sr = simplify(rhs);
            if sl.is_unknown() || sr.is_unknown() {
                return ComplexityExpr::Unknown;
            }
            if sl == sr {
                return sl;
            }
            if sl.is_const_one() {
                return sr;
            }
            if sr.is_const_one() {
                return sl;
            }
            // Check dominance
            let ml = expr_to_monomial(&sl);
            let mr = expr_to_monomial(&sr);
            if let (Some(m1), Some(m2)) = (ml, mr) {
                if m1.dominates(&m2) {
                    return sl;
                }
                if m2.dominates(&m1) {
                    return sr;
                }
            }
            ComplexityExpr::add(sl, sr)
        }
        ComplexityExpr::Max(lhs, rhs) => {
            let sl = simplify(lhs);
            let sr = simplify(rhs);
            if sl == sr {
                return sl;
            }
            let ml = expr_to_monomial(&sl);
            let mr = expr_to_monomial(&sr);
            if let (Some(m1), Some(m2)) = (ml, mr) {
                if m1.dominates(&m2) {
                    return sl;
                }
                if m2.dominates(&m1) {
                    return sr;
                }
            }
            ComplexityExpr::add(sl, sr)
        }
    }
}

fn expr_to_monomial(expr: &ComplexityExpr) -> Option<Monomial> {
    match expr {
        ComplexityExpr::Const(_) => Some(Monomial::one()),
        ComplexityExpr::Var(v) => Some(Monomial::from_var(*v)),
        ComplexityExpr::Log(inner) => {
            if let ComplexityExpr::Var(v) = &**inner {
                Some(Monomial::from_log(*v))
            } else {
                None
            }
        }
        ComplexityExpr::Pow(base, exp) => {
            let m = expr_to_monomial(base)?;
            let mut res = Monomial::one();
            for _ in 0..*exp {
                res = res.mul(&m);
            }
            Some(res)
        }
        ComplexityExpr::Mul(lhs, rhs) => {
            let ml = expr_to_monomial(lhs)?;
            let mr = expr_to_monomial(rhs)?;
            Some(ml.mul(&mr))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simplifies_n_plus_n_to_n() {
        let n = ComplexityExpr::var(DimensionVar::N);
        let expr = ComplexityExpr::add(n.clone(), n.clone());
        assert_eq!(simplify(&expr), n);
    }

    #[test]
    fn preserves_n_plus_q() {
        let n = ComplexityExpr::var(DimensionVar::N);
        let q = ComplexityExpr::var(DimensionVar::Q);
        let expr = ComplexityExpr::add(n.clone(), q.clone());
        let res = simplify(&expr);
        assert_eq!(res, ComplexityExpr::add(n, q));
    }

    #[test]
    fn dominates_n_squared_over_n() {
        let n = ComplexityExpr::var(DimensionVar::N);
        let n2 = ComplexityExpr::pow(n.clone(), 2);
        let expr = ComplexityExpr::add(n2.clone(), n);
        assert_eq!(simplify(&expr), n2);
    }

    #[test]
    fn dominates_n_log_n_over_n() {
        let n = ComplexityExpr::var(DimensionVar::N);
        let log_n = ComplexityExpr::log(n.clone());
        let n_log_n = ComplexityExpr::mul(n.clone(), log_n);
        let expr = ComplexityExpr::add(n_log_n.clone(), n);
        assert_eq!(simplify(&expr), n_log_n);
    }
}

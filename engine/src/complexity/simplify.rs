use super::ast::{ComplexityExpr, DimensionVar};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

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

    pub fn all_vars(&self) -> BTreeSet<DimensionVar> {
        let mut set = BTreeSet::new();
        for v in self.vars.keys() {
            set.insert(*v);
        }
        for v in self.logs.keys() {
            set.insert(*v);
        }
        set
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

        let self_all = self.all_vars();
        let other_all = other.all_vars();

        // If other has any variable that self does not possess, self cannot dominate other
        if !other_all.is_subset(&self_all) {
            return false;
        }

        // For all variables in other, compare powers
        let mut strictly_greater = false;
        let mut strictly_lesser = false;

        for v in &self_all {
            let s_exp = *self.vars.get(v).unwrap_or(&0);
            let o_exp = *other.vars.get(v).unwrap_or(&0);
            let s_log = *self.logs.get(v).unwrap_or(&0);
            let o_log = *other.logs.get(v).unwrap_or(&0);

            if s_exp > o_exp {
                strictly_greater = true;
            } else if s_exp < o_exp {
                strictly_lesser = true;
            } else if s_log > o_log {
                strictly_greater = true;
            } else if s_log < o_log {
                strictly_lesser = true;
            }
        }

        strictly_greater && !strictly_lesser
    }

    pub fn is_one(&self) -> bool {
        self.vars.is_empty() && self.logs.is_empty() && !self.is_log_log
    }

    pub fn total_var_degree(&self) -> u32 {
        self.vars.values().sum()
    }

    pub fn total_log_degree(&self) -> u32 {
        self.logs.values().sum()
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

impl PartialOrd for Monomial {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Monomial {
    fn cmp(&self, other: &Self) -> Ordering {
        // Canonical ordering: higher degree first, then logs, then variable names
        match other.total_var_degree().cmp(&self.total_var_degree()) {
            Ordering::Equal => match other.total_log_degree().cmp(&self.total_log_degree()) {
                Ordering::Equal => {
                    let s_vars: Vec<_> = self.vars.iter().collect();
                    let o_vars: Vec<_> = other.vars.iter().collect();
                    match s_vars.cmp(&o_vars) {
                        Ordering::Equal => {
                            let s_logs: Vec<_> = self.logs.iter().collect();
                            let o_logs: Vec<_> = other.logs.iter().collect();
                            s_logs.cmp(&o_logs)
                        }
                        other_order => other_order,
                    }
                }
                other_order => other_order,
            },
            other_order => other_order,
        }
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
                ComplexityExpr::Unknown => ComplexityExpr::Unknown,
                ComplexityExpr::Pow(base, _) => simplify(&ComplexityExpr::log(*base)),
                _ => ComplexityExpr::log(s_inner),
            }
        }
        ComplexityExpr::Pow(base, exp) => {
            if *exp == 0 {
                ComplexityExpr::one()
            } else if *exp == 1 {
                simplify(base)
            } else {
                let sb = simplify(base);
                if sb.is_unknown() {
                    ComplexityExpr::Unknown
                } else if sb.is_const_one() {
                    ComplexityExpr::one()
                } else {
                    ComplexityExpr::pow(sb, *exp)
                }
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
            if let (Some(ml), Some(mr)) = (expr_to_monomial(&sl), expr_to_monomial(&sr)) {
                return ml.mul(&mr).to_expr();
            }
            let is_l_add = matches!(sl, ComplexityExpr::Add(..));
            let is_r_add = matches!(sr, ComplexityExpr::Add(..));
            if is_r_add && !is_l_add {
                ComplexityExpr::mul(sr, sl)
            } else if is_l_add && !is_r_add {
                ComplexityExpr::mul(sl, sr)
            } else if sl.format_inner() > sr.format_inner() {
                ComplexityExpr::mul(sr, sl)
            } else {
                ComplexityExpr::mul(sl, sr)
            }
        }
        ComplexityExpr::Add(lhs, rhs) | ComplexityExpr::Max(lhs, rhs) => {
            let mut terms = Vec::new();
            collect_additive_terms(lhs, &mut terms);
            collect_additive_terms(rhs, &mut terms);

            let mut simplified_terms: Vec<ComplexityExpr> = Vec::new();
            for t in terms {
                let s = simplify(&t);
                if s.is_unknown() {
                    return ComplexityExpr::Unknown;
                }
                if !s.is_const_one() {
                    simplified_terms.push(s);
                }
            }

            if simplified_terms.is_empty() {
                return ComplexityExpr::one();
            }

            // Convert to monomials where possible to filter dominated terms
            let mut monomials: Vec<Monomial> = Vec::new();
            let mut non_monomials: Vec<ComplexityExpr> = Vec::new();

            for t in simplified_terms {
                if let Some(m) = expr_to_monomial(&t) {
                    monomials.push(m);
                } else {
                    non_monomials.push(t);
                }
            }

            // Filter dominated monomials
            let mut surviving_monomials = Vec::new();
            for i in 0..monomials.len() {
                let mut dominated = false;
                for j in 0..monomials.len() {
                    if i != j && monomials[j].dominates(&monomials[i]) {
                        // Tie breaker: keep earlier index if identical
                        if monomials[j] == monomials[i] && i > j {
                            dominated = true;
                            break;
                        }
                        if monomials[j] != monomials[i] {
                            dominated = true;
                            break;
                        }
                    }
                }
                if !dominated {
                    surviving_monomials.push(monomials[i].clone());
                }
            }

            surviving_monomials.sort();
            surviving_monomials.dedup();

            let mut all_exprs: Vec<ComplexityExpr> = surviving_monomials
                .into_iter()
                .map(|m| m.to_expr())
                .collect();
            all_exprs.extend(non_monomials);
            all_exprs.sort_by_key(|a| a.format_inner());
            all_exprs.dedup();

            if all_exprs.is_empty() {
                return ComplexityExpr::one();
            }

            let mut iter = all_exprs.into_iter();
            let mut res = iter.next().unwrap();
            for next in iter {
                res = ComplexityExpr::add(res, next);
            }
            res
        }
    }
}

fn collect_additive_terms(expr: &ComplexityExpr, terms: &mut Vec<ComplexityExpr>) {
    match expr {
        ComplexityExpr::Add(lhs, rhs) | ComplexityExpr::Max(lhs, rhs) => {
            collect_additive_terms(lhs, terms);
            collect_additive_terms(rhs, terms);
        }
        other => terms.push(other.clone()),
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

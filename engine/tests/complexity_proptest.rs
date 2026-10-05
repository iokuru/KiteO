use kiteo_engine::complexity::ast::{ComplexityExpr, DimensionVar};
use kiteo_engine::complexity::simplify::simplify;
use proptest::prelude::*;

fn arb_dimension_var() -> impl Strategy<Value = DimensionVar> {
    prop_oneof![
        Just(DimensionVar::N),
        Just(DimensionVar::M),
        Just(DimensionVar::Q),
        Just(DimensionVar::K),
        Just(DimensionVar::V),
        Just(DimensionVar::E),
        Just(DimensionVar::A),
    ]
}

fn arb_complexity_expr() -> impl Strategy<Value = ComplexityExpr> {
    let leaf = prop_oneof![
        Just(ComplexityExpr::one()),
        arb_dimension_var().prop_map(ComplexityExpr::var),
    ];
    leaf.prop_recursive(4, 16, 2, |inner| {
        prop_oneof![
            inner.clone().prop_map(ComplexityExpr::log),
            (inner.clone(), 2u32..=3).prop_map(|(e, exp)| ComplexityExpr::pow(e, exp)),
            (inner.clone(), inner.clone()).prop_map(|(a, b)| ComplexityExpr::mul(a, b)),
            (inner.clone(), inner).prop_map(|(a, b)| ComplexityExpr::add(a, b)),
        ]
    })
}

proptest! {
    #[test]
    fn test_idempotence(e in arb_complexity_expr()) {
        let s1 = simplify(&e);
        let s2 = simplify(&s1);
        prop_assert_eq!(s1, s2);
    }

    #[test]
    fn test_multiplication_identity(e in arb_complexity_expr()) {
        let one = ComplexityExpr::one();
        let mul_expr = ComplexityExpr::mul(e.clone(), one);
        prop_assert_eq!(simplify(&mul_expr), simplify(&e));
    }

    #[test]
    fn test_addition_commutativity(a in arb_complexity_expr(), b in arb_complexity_expr()) {
        let ab = simplify(&ComplexityExpr::add(a.clone(), b.clone()));
        let ba = simplify(&ComplexityExpr::add(b, a));
        prop_assert_eq!(ab.to_string(), ba.to_string());
    }

    #[test]
    fn test_multiplication_commutativity(a in arb_complexity_expr(), b in arb_complexity_expr()) {
        let ab = simplify(&ComplexityExpr::mul(a.clone(), b.clone()));
        let ba = simplify(&ComplexityExpr::mul(b, a));
        prop_assert_eq!(ab.to_string(), ba.to_string());
    }

    #[test]
    fn test_addition_associativity(a in arb_complexity_expr(), b in arb_complexity_expr(), c in arb_complexity_expr()) {
        let ab_c = simplify(&ComplexityExpr::add(ComplexityExpr::add(a.clone(), b.clone()), c.clone()));
        let a_bc = simplify(&ComplexityExpr::add(a, ComplexityExpr::add(b, c)));
        prop_assert_eq!(ab_c.to_string(), a_bc.to_string());
    }
}

use kiteo_engine::cost_model::CostModel;

#[test]
fn test_cpp_cost_lookups_and_hidden_costs() {
    let model = CostModel::new();

    // Vector push_back: O(1) amortized
    let pb = model
        .lookup_cpp_cost("vector", "push_back")
        .expect("vector::push_back must exist");
    assert_eq!(pb.time, "O(1)");
    assert!(pb.amortized);

    // Vector erase_begin: O(n) hidden cost
    let erase = model
        .lookup_cpp_cost("vector", "erase_begin")
        .expect("vector::erase_begin must exist");
    assert_eq!(erase.time, "O(n)");
    assert!(erase.hidden_cost.is_some());

    // Unordered map: O(1) expected, O(n) worst
    let umap = model
        .lookup_cpp_cost("unordered_map", "find")
        .expect("unordered_map::find must exist");
    assert_eq!(umap.time, "O(1)");
    assert!(umap.expected);
    assert_eq!(umap.worst.as_deref(), Some("O(n)"));

    // String substr: O(L) hidden cost
    let substr = model
        .lookup_cpp_cost("string", "substr")
        .expect("string::substr must exist");
    assert_eq!(substr.time, "O(L)");
    assert!(substr.hidden_cost.is_some());
}

#[test]
fn test_java_cost_lookups() {
    let model = CostModel::new();

    let al_get = model
        .lookup_java_cost("ArrayList", "get")
        .expect("ArrayList::get must exist");
    assert_eq!(al_get.time, "O(1)");

    let ll_get = model
        .lookup_java_cost("LinkedList", "get")
        .expect("LinkedList::get must exist");
    assert_eq!(ll_get.time, "O(n)");
    assert!(ll_get.hidden_cost.is_some());

    let hmap = model
        .lookup_java_cost("HashMap", "get")
        .expect("HashMap::get must exist");
    assert_eq!(hmap.time, "O(1)");
    assert!(hmap.expected);
}

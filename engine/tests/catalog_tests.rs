use kiteo_engine::algorithms::{
    apply_selection_rules, get_catalog, select_labels, AlgorithmDetector, AllowedAlgorithm,
};
use std::collections::HashSet;

#[test]
fn test_catalog_validation_rules() {
    let catalog = get_catalog();

    // 1. Exactly 68 closed canonical algorithms exist
    assert_eq!(
        catalog.len(),
        68,
        "Catalog must contain exactly 68 closed canonical algorithms"
    );

    // 2. Every catalog label has a valid category
    let valid_categories = ["algorithm", "technique", "data_structure", "paradigm"];
    for item in catalog {
        assert!(
            valid_categories.contains(&item.category.as_str()),
            "Algorithm '{}' has invalid category '{}'",
            item.name,
            item.category
        );
    }

    // 3. Every catalog label has a valid status
    let valid_statuses = ["approved", "candidate"];
    for item in catalog {
        assert!(
            valid_statuses.contains(&item.status.as_str()),
            "Algorithm '{}' has invalid status '{}'",
            item.name,
            item.status
        );
    }

    // 4. Every priority is valid (1..=68) and unique
    let mut seen_priorities = HashSet::new();
    for item in catalog {
        assert!(
            item.priority >= 1 && item.priority <= 68,
            "Algorithm '{}' has out-of-range priority {}",
            item.name,
            item.priority
        );
        assert!(
            seen_priorities.insert(item.priority),
            "Duplicate priority {} found for algorithm '{}'",
            item.priority,
            item.name
        );
    }

    // 5. Every suppression target exists in the catalog
    let catalog_names: HashSet<&str> = catalog.iter().map(|item| item.name.as_str()).collect();
    for item in catalog {
        for target in &item.suppresses {
            assert!(
                catalog_names.contains(target.as_str()),
                "Algorithm '{}' suppresses non-existent label '{}'",
                item.name,
                target
            );
        }
    }

    // 6. No duplicate canonical names exist
    let mut seen_names = HashSet::new();
    for item in catalog {
        assert!(
            seen_names.insert(&item.name),
            "Duplicate canonical name '{}' in catalog",
            item.name
        );
    }

    // 7. Every catalog entry corresponds to an AllowedAlgorithm enum
    for item in catalog {
        let algo = AllowedAlgorithm::from_canonical_name(&item.name);
        assert!(
            algo.is_some(),
            "Catalog algorithm '{}' must exist in AllowedAlgorithm enum",
            item.name
        );
    }

    // 8. Every AllowedAlgorithm enum exists in catalog
    assert_eq!(AllowedAlgorithm::ALL.len(), 68);
    for algo in AllowedAlgorithm::ALL {
        let name = algo.as_str();
        assert!(
            catalog.iter().any(|item| item.name == name),
            "AllowedAlgorithm '{name}' must exist in catalog.toml"
        );
    }

    // 9. Intentionally invalid label rejection
    assert!(AllowedAlgorithm::from_canonical_name("DisjointSetUnion").is_none());
    assert!(AllowedAlgorithm::from_canonical_name("SCC").is_none());
    assert!(AllowedAlgorithm::from_canonical_name("ArbitraryAlgorithm").is_none());
}

#[test]
fn test_suppression_rules() {
    // 1. Tree DP suppresses Dynamic Programming and DFS
    let detected = vec![
        AllowedAlgorithm::TreeDp,
        AllowedAlgorithm::DynamicProgramming,
        AllowedAlgorithm::Dfs,
    ];
    let filtered = select_labels(&detected, "", false);
    assert!(filtered.contains(&AllowedAlgorithm::TreeDp));
    assert!(!filtered.contains(&AllowedAlgorithm::DynamicProgramming));
    assert!(!filtered.contains(&AllowedAlgorithm::Dfs));

    // 2. Bitmask DP suppresses Dynamic Programming
    let detected = vec![
        AllowedAlgorithm::BitmaskDp,
        AllowedAlgorithm::DynamicProgramming,
    ];
    let filtered = select_labels(&detected, "", false);
    assert!(filtered.contains(&AllowedAlgorithm::BitmaskDp));
    assert!(!filtered.contains(&AllowedAlgorithm::DynamicProgramming));

    // 3. Dijkstra suppresses BFS and Heap / Priority Queue
    let detected = vec![
        AllowedAlgorithm::Dijkstra,
        AllowedAlgorithm::Bfs,
        AllowedAlgorithm::HeapPriorityQueue,
    ];
    let filtered = select_labels(&detected, "", false);
    assert!(filtered.contains(&AllowedAlgorithm::Dijkstra));
    assert!(!filtered.contains(&AllowedAlgorithm::Bfs));
    assert!(!filtered.contains(&AllowedAlgorithm::HeapPriorityQueue));

    // 4. Topological Sort suppresses BFS and DFS
    let detected = vec![
        AllowedAlgorithm::TopologicalSort,
        AllowedAlgorithm::Bfs,
        AllowedAlgorithm::Dfs,
    ];
    let filtered = select_labels(&detected, "", false);
    assert!(filtered.contains(&AllowedAlgorithm::TopologicalSort));
    assert!(!filtered.contains(&AllowedAlgorithm::Bfs));
    assert!(!filtered.contains(&AllowedAlgorithm::Dfs));

    // 5. Strongly Connected Components suppresses DFS
    let detected = vec![
        AllowedAlgorithm::StronglyConnectedComponents,
        AllowedAlgorithm::Dfs,
    ];
    let filtered = select_labels(&detected, "", false);
    assert!(filtered.contains(&AllowedAlgorithm::StronglyConnectedComponents));
    assert!(!filtered.contains(&AllowedAlgorithm::Dfs));

    // 6. Binary Search on Answer suppresses Binary Search
    let detected = vec![
        AllowedAlgorithm::BinarySearchOnAnswer,
        AllowedAlgorithm::BinarySearch,
    ];
    let filtered = select_labels(&detected, "", false);
    assert!(filtered.contains(&AllowedAlgorithm::BinarySearchOnAnswer));
    assert!(!filtered.contains(&AllowedAlgorithm::BinarySearch));

    // 7. Sliding Window suppresses Two Pointers
    let detected = vec![
        AllowedAlgorithm::SlidingWindow,
        AllowedAlgorithm::TwoPointers,
    ];
    let filtered = select_labels(&detected, "", false);
    assert!(filtered.contains(&AllowedAlgorithm::SlidingWindow));
    assert!(!filtered.contains(&AllowedAlgorithm::TwoPointers));

    // 8. Top K and Two Heaps suppress Heap / Priority Queue
    let detected = vec![
        AllowedAlgorithm::TopK,
        AllowedAlgorithm::TwoHeaps,
        AllowedAlgorithm::HeapPriorityQueue,
    ];
    let filtered = select_labels(&detected, "", false);
    assert!(filtered.contains(&AllowedAlgorithm::TopK));
    assert!(filtered.contains(&AllowedAlgorithm::TwoHeaps));
    assert!(!filtered.contains(&AllowedAlgorithm::HeapPriorityQueue));

    // 9. Monotonic Stack suppresses Stack
    let detected = vec![AllowedAlgorithm::MonotonicStack, AllowedAlgorithm::Stack];
    let filtered = select_labels(&detected, "", false);
    assert!(filtered.contains(&AllowedAlgorithm::MonotonicStack));
    assert!(!filtered.contains(&AllowedAlgorithm::Stack));

    // 10. Monotonic Queue suppresses Queue
    let detected = vec![AllowedAlgorithm::MonotonicQueue, AllowedAlgorithm::Queue];
    let filtered = select_labels(&detected, "", false);
    assert!(filtered.contains(&AllowedAlgorithm::MonotonicQueue));
    assert!(!filtered.contains(&AllowedAlgorithm::Queue));

    // 11. Lazy Segment Tree suppresses Segment Tree
    let detected = vec![
        AllowedAlgorithm::LazySegmentTree,
        AllowedAlgorithm::SegmentTree,
    ];
    let filtered = select_labels(&detected, "", false);
    assert!(filtered.contains(&AllowedAlgorithm::LazySegmentTree));
    assert!(!filtered.contains(&AllowedAlgorithm::SegmentTree));

    // 12. Segment Tree suppresses Prefix Sum only when prefix is computed in tree
    let detected = vec![AllowedAlgorithm::SegmentTree, AllowedAlgorithm::PrefixSum];
    let filtered_tree_prefix = select_labels(&detected, "tree.query()", false);
    assert!(filtered_tree_prefix.contains(&AllowedAlgorithm::SegmentTree));
    assert!(!filtered_tree_prefix.contains(&AllowedAlgorithm::PrefixSum));

    // But if code explicitly uses pref array, Prefix Sum is preserved alongside
    let filtered_both = select_labels(&detected, "pref[i] = pref[i-1] + a[i];", false);
    assert!(filtered_both.contains(&AllowedAlgorithm::SegmentTree));
    assert!(filtered_both.contains(&AllowedAlgorithm::PrefixSum));
}

#[test]
fn test_priority_cap_at_three() {
    // 5 algorithms with distinct priorities:
    // Dijkstra (priority 5), MST (6), DSU (14), Binary Search (42), Sorting (66)
    let detected = vec![
        AllowedAlgorithm::Sorting,
        AllowedAlgorithm::BinarySearch,
        AllowedAlgorithm::Dsu,
        AllowedAlgorithm::Mst,
        AllowedAlgorithm::Dijkstra,
    ];
    let filtered = select_labels(&detected, "", false);
    assert_eq!(
        filtered.len(),
        3,
        "Output must be capped at maximum 3 labels"
    );
    assert_eq!(filtered[0], AllowedAlgorithm::Dijkstra);
    assert_eq!(filtered[1], AllowedAlgorithm::Mst);
    assert_eq!(filtered[2], AllowedAlgorithm::Dsu);
}

#[test]
fn test_deterministic_ordering() {
    let list_a = vec![
        AllowedAlgorithm::Sorting,
        AllowedAlgorithm::Dijkstra,
        AllowedAlgorithm::Sieve,
    ];
    let list_b = vec![
        AllowedAlgorithm::Dijkstra,
        AllowedAlgorithm::Sorting,
        AllowedAlgorithm::Sieve,
    ];
    let out_a = select_labels(&list_a, "", false);
    let out_b = select_labels(&list_b, "", false);
    assert_eq!(
        out_a, out_b,
        "Output ordering must be deterministic regardless of input order"
    );
    assert_eq!(out_a[0], AllowedAlgorithm::Dijkstra);
    assert_eq!(out_a[1], AllowedAlgorithm::Sieve);
    assert_eq!(out_a[2], AllowedAlgorithm::Sorting);
}

#[test]
fn test_status_gate_candidate_not_output() {
    // Dynamic Programming is marked as "candidate" in catalog.toml
    assert!(!AllowedAlgorithm::DynamicProgramming.is_approved());
    let detected = vec![AllowedAlgorithm::DynamicProgramming];
    let filtered = apply_selection_rules(&detected, "");
    assert!(
        filtered.is_empty(),
        "Candidate algorithm must not be emitted as an approved label"
    );
}

#[test]
fn test_selection_pipeline_counts() {
    // 0 labels -> empty
    assert!(select_labels(&[], "", false).is_empty());

    // 1 label -> 1 label
    let one = vec![AllowedAlgorithm::Dijkstra];
    assert_eq!(select_labels(&one, "", false).len(), 1);

    // 2 labels -> 2 labels
    let two = vec![AllowedAlgorithm::Dijkstra, AllowedAlgorithm::Sorting];
    assert_eq!(select_labels(&two, "", false).len(), 2);

    // 3 labels -> 3 labels
    let three = vec![
        AllowedAlgorithm::Dijkstra,
        AllowedAlgorithm::Sieve,
        AllowedAlgorithm::Sorting,
    ];
    assert_eq!(select_labels(&three, "", false).len(), 3);

    // 4+ labels -> capped at 3
    let four = vec![
        AllowedAlgorithm::Dijkstra,
        AllowedAlgorithm::Sieve,
        AllowedAlgorithm::Sorting,
        AllowedAlgorithm::Bfs,
    ];
    assert_eq!(select_labels(&four, "", false).len(), 3);
}

#[test]
fn test_kadane_detector_positive_and_negative() {
    let positive_cpp = r#"
    int maxSubArray(vector<int>& nums) {
        int max_so_far = nums[0], cur_max = nums[0];
        for (int i = 1; i < nums.size(); i++) {
            cur_max = max(nums[i], cur_max + nums[i]);
            max_so_far = max(max_so_far, cur_max);
        }
        return max_so_far;
    }
    "#;
    assert!(
        AlgorithmDetector::detect_raw(positive_cpp).contains(&AllowedAlgorithm::KadanesAlgorithm)
    );
    assert!(
        !AlgorithmDetector::detect(positive_cpp).contains(&AllowedAlgorithm::KadanesAlgorithm),
        "Candidate algorithm must not be emitted by detect() until approved"
    );

    let positive_java = r#"
    class Solution {
        public int maxSubArray(int[] nums) {
            int max_so_far = nums[0], cur_max = nums[0];
            for (int i = 1; i < nums.length; i++) {
                cur_max = Math.max(nums[i], cur_max + nums[i]);
                max_so_far = Math.max(max_so_far, cur_max);
            }
            return max_so_far;
        }
    }
    "#;
    assert!(
        AlgorithmDetector::detect_raw(positive_java).contains(&AllowedAlgorithm::KadanesAlgorithm)
    );
    assert!(!AlgorithmDetector::detect(positive_java).contains(&AllowedAlgorithm::KadanesAlgorithm));

    // Negative look-alike 1: Plain prefix sum (running sum without reset)
    let negative_prefix_sum = r#"
    int sum = 0;
    for (int i = 0; i < nums.size(); i++) {
        sum += nums[i];
        pref[i] = sum;
    }
    "#;
    assert!(!AlgorithmDetector::detect_raw(negative_prefix_sum)
        .contains(&AllowedAlgorithm::KadanesAlgorithm));

    // Negative look-alike 2: Finding array maximum element without running sum
    let negative_array_max = r#"
    int max_val = nums[0];
    for (int i = 1; i < nums.size(); i++) {
        max_val = max(max_val, nums[i]);
    }
    "#;
    assert!(!AlgorithmDetector::detect_raw(negative_array_max)
        .contains(&AllowedAlgorithm::KadanesAlgorithm));
}

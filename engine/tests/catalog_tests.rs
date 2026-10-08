use kiteo_engine::algorithms::{
    apply_selection_rules, get_catalog, AlgorithmDetector, AllowedAlgorithm,
};

#[test]
fn test_no_label_outside_catalog() {
    let catalog = get_catalog();
    assert_eq!(
        catalog.len(),
        39,
        "Catalog must contain exactly 39 closed canonical algorithms"
    );

    // Every catalog entry corresponds to an AllowedAlgorithm enum
    for item in catalog {
        let algo = AllowedAlgorithm::from_canonical_name(&item.name);
        assert!(
            algo.is_some(),
            "Catalog algorithm '{}' must exist in AllowedAlgorithm enum",
            item.name
        );
    }

    // Every AllowedAlgorithm enum exists in catalog
    for algo in AllowedAlgorithm::ALL {
        let name = algo.as_str();
        assert!(
            catalog.iter().any(|item| item.name == name),
            "AllowedAlgorithm '{name}' must exist in catalog.toml"
        );
    }
}

#[test]
fn test_suppression_rules() {
    // 1. Tree DP suppresses Dynamic Programming and DFS
    let detected = vec![
        AllowedAlgorithm::TreeDp,
        AllowedAlgorithm::DynamicProgramming,
        AllowedAlgorithm::Dfs,
    ];
    let filtered = apply_selection_rules(&detected, "");
    assert!(filtered.contains(&AllowedAlgorithm::TreeDp));
    assert!(!filtered.contains(&AllowedAlgorithm::DynamicProgramming));
    assert!(!filtered.contains(&AllowedAlgorithm::Dfs));

    // 2. Bitmask DP suppresses Dynamic Programming
    let detected = vec![
        AllowedAlgorithm::BitmaskDp,
        AllowedAlgorithm::DynamicProgramming,
    ];
    let filtered = apply_selection_rules(&detected, "");
    assert!(filtered.contains(&AllowedAlgorithm::BitmaskDp));
    assert!(!filtered.contains(&AllowedAlgorithm::DynamicProgramming));

    // 3. Dijkstra suppresses BFS and Heap / Priority Queue
    let detected = vec![
        AllowedAlgorithm::Dijkstra,
        AllowedAlgorithm::Bfs,
        AllowedAlgorithm::HeapPriorityQueue,
    ];
    let filtered = apply_selection_rules(&detected, "");
    assert!(filtered.contains(&AllowedAlgorithm::Dijkstra));
    assert!(!filtered.contains(&AllowedAlgorithm::Bfs));
    assert!(!filtered.contains(&AllowedAlgorithm::HeapPriorityQueue));

    // 4. Topological Sort suppresses BFS and DFS
    let detected = vec![
        AllowedAlgorithm::TopologicalSort,
        AllowedAlgorithm::Bfs,
        AllowedAlgorithm::Dfs,
    ];
    let filtered = apply_selection_rules(&detected, "");
    assert!(filtered.contains(&AllowedAlgorithm::TopologicalSort));
    assert!(!filtered.contains(&AllowedAlgorithm::Bfs));
    assert!(!filtered.contains(&AllowedAlgorithm::Dfs));

    // 5. Binary Search on Answer suppresses Binary Search
    let detected = vec![
        AllowedAlgorithm::BinarySearchOnAnswer,
        AllowedAlgorithm::BinarySearch,
    ];
    let filtered = apply_selection_rules(&detected, "");
    assert!(filtered.contains(&AllowedAlgorithm::BinarySearchOnAnswer));
    assert!(!filtered.contains(&AllowedAlgorithm::BinarySearch));

    // 6. Sliding Window suppresses Two Pointers
    let detected = vec![
        AllowedAlgorithm::SlidingWindow,
        AllowedAlgorithm::TwoPointers,
    ];
    let filtered = apply_selection_rules(&detected, "");
    assert!(filtered.contains(&AllowedAlgorithm::SlidingWindow));
    assert!(!filtered.contains(&AllowedAlgorithm::TwoPointers));
}

#[test]
fn test_priority_cap_at_three() {
    // 5 algorithms with distinct priorities:
    // Dijkstra (priority 4), MST (5), DSU (11), Binary Search (33), Sorting (39)
    let detected = vec![
        AllowedAlgorithm::Sorting,
        AllowedAlgorithm::BinarySearch,
        AllowedAlgorithm::Dsu,
        AllowedAlgorithm::Mst,
        AllowedAlgorithm::Dijkstra,
    ];
    let filtered = apply_selection_rules(&detected, "");
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
    let out_a = apply_selection_rules(&list_a, "");
    let out_b = apply_selection_rules(&list_b, "");
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
    // Divide and Conquer is marked as "candidate" in catalog.toml
    assert!(!AllowedAlgorithm::DivideAndConquer.is_approved());
    let detected = vec![AllowedAlgorithm::DivideAndConquer];
    let filtered = apply_selection_rules(&detected, "");
    assert!(
        filtered.is_empty(),
        "Candidate algorithm must not be emitted as an approved label"
    );
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
    assert!(AlgorithmDetector::detect(positive_cpp).contains(&AllowedAlgorithm::KadanesAlgorithm));

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
    assert!(AlgorithmDetector::detect(positive_java).contains(&AllowedAlgorithm::KadanesAlgorithm));

    // Negative look-alike 1: Plain prefix sum (running sum without reset)
    let negative_prefix_sum = r#"
    int sum = 0;
    for (int i = 0; i < nums.size(); i++) {
        sum += nums[i];
        pref[i] = sum;
    }
    "#;
    assert!(!AlgorithmDetector::detect(negative_prefix_sum)
        .contains(&AllowedAlgorithm::KadanesAlgorithm));

    // Negative look-alike 2: Finding array maximum element without running sum
    let negative_array_max = r#"
    int max_val = nums[0];
    for (int i = 1; i < nums.size(); i++) {
        max_val = max(max_val, nums[i]);
    }
    "#;
    assert!(!AlgorithmDetector::detect(negative_array_max)
        .contains(&AllowedAlgorithm::KadanesAlgorithm));
}

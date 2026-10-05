use kiteo_engine::analyze;

#[test]
fn test_standard_binary_search_cpp() {
    let code = r#"
    int binarySearch(const vector<int>& a, int target) {
        int l = 0, r = (int)a.size() - 1;
        while (l <= r) {
            int mid = l + (r - l) / 2;
            if (a[mid] == target) return mid;
            if (a[mid] < target) l = mid + 1;
            else r = mid - 1;
        }
        return -1;
    }
    "#;

    let res = analyze(code, "cpp");
    assert_eq!(res.tc, "O(log n)");
    assert_eq!(res.sc, "O(1)");
    assert_eq!(res.algorithms, vec!["Binary Search"]);
}

#[test]
fn test_binary_search_on_answer_cpp() {
    let code = r#"
    bool check(int mid, const vector<int>& a, int n);
    int solve(int n, int A, const vector<int>& a) {
        int low = 1, high = A, ans = -1;
        while (low <= high) {
            int mid = low + (high - low) / 2;
            if (check(mid, a, n)) {
                ans = mid;
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        return ans;
    }
    "#;

    let res = analyze(code, "cpp");
    assert_eq!(res.tc, "O(n log A)");
    assert_eq!(res.sc, "O(1)");
    assert_eq!(res.algorithms, vec!["Binary Search on Answer"]);
}

#[test]
fn test_negative_lookalike_non_binary_search() {
    // Look-alike: uses while (l <= r) but increments by 1 without halving mid
    let lookalike = r#"
    int countPairs(const vector<int>& a, int k) {
        int l = 0, r = a.size() - 1;
        int count = 0;
        while (l <= r) {
            count += a[l];
            l++;
        }
        return count;
    }
    "#;

    let res = analyze(lookalike, "cpp");
    assert!(!res.algorithms.contains(&"Binary Search".to_string()));
    assert!(!res
        .algorithms
        .contains(&"Binary Search on Answer".to_string()));
}

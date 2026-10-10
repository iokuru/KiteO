use kiteo_engine::algorithms::{AlgorithmDetector, AllowedAlgorithm};

#[test]
fn test_two_pointers_positive_and_negative() {
    let positive = r#"
    int l = 0, r = n - 1;
    while (l < r) {
        if (a[l] + a[r] == target) return true;
        else if (a[l] + a[r] < target) l++;
        else r--;
    }
    "#;
    assert!(AlgorithmDetector::detect_raw(positive).contains(&AllowedAlgorithm::TwoPointers));
    assert!(!AlgorithmDetector::detect(positive).contains(&AllowedAlgorithm::TwoPointers));

    let negative_nested = r#"
    for (int i = 0; i < n; i++) {
        for (int j = i + 1; j < n; j++) {
            if (a[i] + a[j] == target) return true;
        }
    }
    "#;
    assert!(!AlgorithmDetector::detect_raw(negative_nested).contains(&AllowedAlgorithm::TwoPointers));
}

#[test]
fn test_sieve_positive_and_negative() {
    let positive = r#"
    vector<bool> is_prime(n + 1, true);
    for (int p = 2; p * p <= n; p++) {
        if (is_prime[p]) {
            for (int i = p * p; i <= n; i += p) is_prime[i] = false;
        }
    }
    "#;
    assert!(AlgorithmDetector::detect_raw(positive).contains(&AllowedAlgorithm::Sieve));
    assert!(!AlgorithmDetector::detect(positive).contains(&AllowedAlgorithm::Sieve));

    let negative_factor_count = r#"
    int divisors = 0;
    for (int i = 1; i <= n; i++) {
        if (n % i == 0) divisors++;
    }
    "#;
    assert!(!AlgorithmDetector::detect_raw(negative_factor_count).contains(&AllowedAlgorithm::Sieve));
}

#[test]
fn test_dsu_positive_and_negative() {
    let positive = r#"
    struct DSU {
        vector<int> parent;
        int find(int i) {
            if (parent[i] == i) return i;
            return parent[i] = find(parent[i]);
        }
        void unite(int i, int j) {
            int root_i = find(i), root_j = find(j);
            if (root_i != root_j) parent[root_i] = root_j;
        }
    };
    "#;
    assert!(AlgorithmDetector::detect_raw(positive).contains(&AllowedAlgorithm::Dsu));
    assert!(!AlgorithmDetector::detect(positive).contains(&AllowedAlgorithm::Dsu));

    let negative_parent_pointer = r#"
    struct TreeNode {
        TreeNode* parent;
        int val;
    };
    "#;
    assert!(!AlgorithmDetector::detect_raw(negative_parent_pointer).contains(&AllowedAlgorithm::Dsu));
}

#[test]
fn test_monotonic_stack_positive_and_negative() {
    let positive = r#"
    stack<int> st;
    for (int i = 0; i < n; i++) {
        while (!st.empty() && a[st.top()] < a[i]) {
            st.pop();
        }
        st.push(i);
    }
    "#;
    assert!(AlgorithmDetector::detect_raw(positive).contains(&AllowedAlgorithm::MonotonicStack));
    assert!(!AlgorithmDetector::detect(positive).contains(&AllowedAlgorithm::MonotonicStack));

    let negative_plain_stack = r#"
    stack<char> st;
    for (char c : s) {
        if (c == '(') st.push(c);
        else if (!st.empty()) st.pop();
    }
    "#;
    assert!(!AlgorithmDetector::detect_raw(negative_plain_stack)
        .contains(&AllowedAlgorithm::MonotonicStack));
}

#[test]
fn test_dijkstra_positive_and_negative() {
    let positive = r#"
    priority_queue<pair<long long, int>, vector<pair<long long, int>>, greater<>> pq;
    vector<long long> dist(V, 1e18);
    dist[start] = 0;
    pq.push({0, start});
    while (!pq.empty()) {
        auto [d, u] = pq.top(); pq.pop();
        if (d > dist[u]) continue;
        for (auto [v, weight] : g[u]) {
            if (dist[u] + weight < dist[v]) {
                dist[v] = dist[u] + weight;
                pq.push({dist[v], v});
            }
        }
    }
    "#;
    assert!(AlgorithmDetector::detect_raw(positive).contains(&AllowedAlgorithm::Dijkstra));
    assert!(!AlgorithmDetector::detect(positive).contains(&AllowedAlgorithm::Dijkstra));

    let negative_max_heap = r#"
    priority_queue<int> pq;
    for (int x : nums) pq.push(x);
    int top = pq.top();
    "#;
    assert!(!AlgorithmDetector::detect_raw(negative_max_heap).contains(&AllowedAlgorithm::Dijkstra));
}

#[test]
fn test_grid_dfs_positive() {
    let code = r#"
    void dfs(vector<vector<char>>& grid, int i, int j) {
        if (i < 0 || j < 0 || i >= grid.size() || j >= grid[0].size() || grid[i][j] == '0') return;
        grid[i][j] = '0';
        dfs(grid, i + 1, j);
        dfs(grid, i - 1, j);
    }
    "#;
    assert!(AlgorithmDetector::detect_raw(code).contains(&AllowedAlgorithm::Dfs));
}

#[test]
fn test_python_bfs_num_islands() {
    let code = r#"
    class Solution:
        def numIslands(self, grid: List[List[str]]) -> int:
            visited = set()
            def bfs(r, c):
                q = collections.deque()
                visited.add((r, c))
                q.append((r, c))
                while q:
                    row, col = q.popleft()
    "#;
    assert!(AlgorithmDetector::detect_raw(code).contains(&AllowedAlgorithm::Bfs));
}


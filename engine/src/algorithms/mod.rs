use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AllowedAlgorithm {
    BinarySearch,
    BinarySearchOnAnswer,
    TwoPointers,
    SlidingWindow,
    PrefixSum,
    Sorting,
    Bfs,
    Dfs,
    TopologicalSort,
    Dsu,
    Dijkstra,
    Mst,
    FenwickTree,
    SegmentTree,
    SparseTable,
    MonotonicStack,
    MonotonicQueue,
    BinaryLifting,
    Lca,
    TreeDp,
    BitmaskDp,
    Sieve,
    Kmp,
    ZAlgorithm,
    RollingHash,
    Trie,
}

impl AllowedAlgorithm {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BinarySearch => "Binary Search",
            Self::BinarySearchOnAnswer => "Binary Search on Answer",
            Self::TwoPointers => "Two Pointers",
            Self::SlidingWindow => "Sliding Window",
            Self::PrefixSum => "Prefix Sum",
            Self::Sorting => "Sorting",
            Self::Bfs => "BFS",
            Self::Dfs => "DFS",
            Self::TopologicalSort => "Topological Sort",
            Self::Dsu => "DSU",
            Self::Dijkstra => "Dijkstra",
            Self::Mst => "MST",
            Self::FenwickTree => "Fenwick Tree",
            Self::SegmentTree => "Segment Tree",
            Self::SparseTable => "Sparse Table",
            Self::MonotonicStack => "Monotonic Stack",
            Self::MonotonicQueue => "Monotonic Queue",
            Self::BinaryLifting => "Binary Lifting",
            Self::Lca => "LCA",
            Self::TreeDp => "Tree DP",
            Self::BitmaskDp => "Bitmask DP",
            Self::Sieve => "Sieve",
            Self::Kmp => "KMP",
            Self::ZAlgorithm => "Z Algorithm",
            Self::RollingHash => "Rolling Hash",
            Self::Trie => "Trie",
        }
    }
}

pub struct AlgorithmDetector;

impl AlgorithmDetector {
    pub fn detect(source: &str) -> Vec<AllowedAlgorithm> {
        let mut detected = Vec::new();

        // 1. Sieve
        if (source.contains("p * p")
            || source.contains("p*p")
            || source.contains("isPrime")
            || source.contains("is_prime"))
            && (source.contains("+= p") || source.contains("+=p"))
        {
            detected.push(AllowedAlgorithm::Sieve);
        }

        // 2. Segment Tree
        if (source.contains("4 * n")
            || source.contains("4*n")
            || source.contains("2 * node")
            || source.contains("2*node"))
            && (source.contains("mid") || source.contains("update") || source.contains("query"))
        {
            detected.push(AllowedAlgorithm::SegmentTree);
        }

        // 3. Fenwick Tree
        if (source.contains("i & -i") || source.contains("x & -x") || source.contains("idx & -idx"))
            && !detected.contains(&AllowedAlgorithm::SegmentTree)
            && (source.contains("Fenwick")
                || ((source.contains("add(") || source.contains("update("))
                    && source.contains("query(")))
        {
            detected.push(AllowedAlgorithm::FenwickTree);
        }

        // 4. Sparse Table
        if (source.contains("st[") || source.contains("st.") || source.contains("SparseTable"))
            && (source.contains("1 << j")
                || source.contains("1<<j")
                || source.contains("31 - __builtin_clz")
                || source.contains("numberOfLeadingZeros"))
        {
            detected.push(AllowedAlgorithm::SparseTable);
        }

        // 5. DSU & MST
        let has_dsu = (source.contains("parent")
            || source.contains("parent[")
            || source.contains("root"))
            && (source.contains("find(") || source.contains("unite(") || source.contains("union("));

        if (has_dsu
            && (source.contains("weight")
                || source.contains("cost")
                || source.contains("Edge")
                || source.contains("edge"))
            && (source.contains("sort")
                || source.contains("mst")
                || source.contains("MST")
                || source.contains("Kruskal")
                || source.contains("kruskal")))
            || (source.contains("prim(")
                || source.contains("Prim(")
                || source.contains("prims(")
                || source.contains("Prims(")
                || source.contains("prim_mst")
                || source.contains("kruskal_mst"))
        {
            detected.push(AllowedAlgorithm::Mst);
        } else if has_dsu {
            detected.push(AllowedAlgorithm::Dsu);
        }

        // 6. Binary Lifting & LCA
        let has_binary_lifting =
            (source.contains("up[") || source.contains("ancestor[") || source.contains("jump["))
                && (source.contains("1 <<")
                    || source.contains("1<<")
                    || source.contains("LOG")
                    || source.contains("LOGN")
                    || source.contains("20"));
        if has_binary_lifting
            && (source.contains("lca(")
                || source.contains("LCA")
                || source.contains("depth[")
                || source.contains("tin["))
        {
            detected.push(AllowedAlgorithm::Lca);
            detected.push(AllowedAlgorithm::BinaryLifting);
        } else if has_binary_lifting {
            detected.push(AllowedAlgorithm::BinaryLifting);
        }

        // 7. Z Algorithm
        if (source.contains("z[")
            || source.contains("z_algorithm")
            || source.contains("zAlgorithm")
            || source.contains("z_box"))
            && (source.contains("r - i + 1")
                || (source.contains("l = i") && source.contains("r = "))
                || source.contains("z[k]"))
        {
            detected.push(AllowedAlgorithm::ZAlgorithm);
        }

        // 8. Rolling Hash
        if (source.contains("hash") || source.contains("Hash") || source.contains("poly_hash"))
            && (source.contains("p_pow")
                || source.contains("power[")
                || source.contains("base")
                || source.contains("BASE"))
            && (source.contains("%") || source.contains("MOD"))
        {
            detected.push(AllowedAlgorithm::RollingHash);
        }

        // 9. Dijkstra
        if (source.contains("priority_queue") || source.contains("PriorityQueue"))
            && (source.contains("dist[") || source.contains("dist.") || source.contains("weight"))
        {
            detected.push(AllowedAlgorithm::Dijkstra);
        }

        // 10. Topological Sort
        if (source.contains("in_degree")
            || source.contains("inDegree")
            || source.contains("indegree"))
            && (source.contains("queue")
                || source.contains("Queue")
                || source.contains("q.push")
                || source.contains("q.add"))
        {
            detected.push(AllowedAlgorithm::TopologicalSort);
        }

        // 11. BFS
        if (source.contains("queue") || source.contains("Queue") || source.contains("LinkedList"))
            && (source.contains("vis[") || source.contains("visited") || source.contains("vis."))
            && !detected.contains(&AllowedAlgorithm::TopologicalSort)
            && !detected.contains(&AllowedAlgorithm::Dijkstra)
        {
            detected.push(AllowedAlgorithm::Bfs);
        }

        // 12. Tree DP
        if (source.contains("v != p")
            || source.contains("p != v")
            || source.contains("v != parent"))
            && (source.contains("dp[") || source.contains("sz[") || source.contains("sz."))
        {
            detected.push(AllowedAlgorithm::TreeDp);
        }

        // 13. DFS (if not Tree DP)
        if !detected.contains(&AllowedAlgorithm::TreeDp)
            && (source.contains("dfs(") || source.contains("dfs ("))
            && (source.contains("vis[") || source.contains("vis.") || source.contains("visited"))
        {
            detected.push(AllowedAlgorithm::Dfs);
        }

        // 14. Bitmask DP
        if (source.contains("1 << n")
            || source.contains("1<<n")
            || source.contains("1 << ")
            || source.contains("1<<"))
            && (source.contains("mask") || source.contains("dp["))
        {
            detected.push(AllowedAlgorithm::BitmaskDp);
        }

        // 15. Monotonic Stack
        if (source.contains("stack<")
            || source.contains("ArrayDeque")
            || source.contains("st.push")
            || source.contains("st.pop"))
            && (source.contains("while (!st.empty()") || source.contains("while (!st.isEmpty()"))
            && (source.contains("< a[i]")
                || source.contains("> a[i]")
                || source.contains("< a[")
                || source.contains("> a["))
        {
            detected.push(AllowedAlgorithm::MonotonicStack);
        }

        // 16. Monotonic Queue
        if (source.contains("deque<") || source.contains("Deque") || source.contains("ArrayDeque"))
            && (source.contains("pop_front") || source.contains("pollFirst"))
            && (source.contains("pop_back") || source.contains("pollLast"))
        {
            detected.push(AllowedAlgorithm::MonotonicQueue);
        }

        // 17. Prefix Sum
        if (source.contains("pref[i - 1]")
            || source.contains("pref[i-1]")
            || source.contains("prefix[i - 1]")
            || source.contains("prefix[i-1]"))
            && !detected.contains(&AllowedAlgorithm::FenwickTree)
        {
            detected.push(AllowedAlgorithm::PrefixSum);
        }

        // 18. Binary Search on Answer
        if (source.contains("check(")
            || source.contains("isPossible(")
            || source.contains("isValid("))
            && (source.contains("low <= high") || source.contains("l <= r"))
            && (source.contains("mid = low +")
                || source.contains("mid = (low +")
                || source.contains("mid = l +")
                || source.contains("mid = (l +"))
        {
            detected.push(AllowedAlgorithm::BinarySearchOnAnswer);
        }

        // 19. Binary Search (if not Binary Search on Answer)
        if !detected.contains(&AllowedAlgorithm::BinarySearchOnAnswer)
            && (source.contains("low <= high") || source.contains("l <= r"))
            && (source.contains("mid = low +")
                || source.contains("mid = (low +")
                || source.contains("mid = l +")
                || source.contains("mid = (l +")
                || source.contains("mid ="))
            && !detected.contains(&AllowedAlgorithm::SegmentTree)
        {
            detected.push(AllowedAlgorithm::BinarySearch);
        }

        // 20. Two Pointers
        if (source.contains("l < r") || source.contains("left < right"))
            && (source.contains("l++") || source.contains("left++"))
            && (source.contains("r--") || source.contains("right--"))
        {
            detected.push(AllowedAlgorithm::TwoPointers);
        }

        // 21. Sliding Window
        if (source.contains("for (int r = 0") || source.contains("for (int right = 0"))
            && (source.contains("while (") && (source.contains("l++") || source.contains("left++")))
        {
            detected.push(AllowedAlgorithm::SlidingWindow);
        }

        // 22. KMP
        if (source.contains("pi[") || source.contains("pi.") || source.contains("lps["))
            && (source.contains("pattern") || source.contains("needle") || source.contains("match"))
        {
            detected.push(AllowedAlgorithm::Kmp);
        }

        // 23. Trie
        if source.contains("child[26]")
            || source.contains("Node[26]")
            || source.contains("is_end")
            || source.contains("isEnd")
        {
            detected.push(AllowedAlgorithm::Trie);
        }

        // 24. Sorting (only when main technique)
        if (source.contains("std::sort")
            || source.contains("Arrays.sort")
            || source.contains("Collections.sort"))
            && detected.is_empty()
        {
            detected.push(AllowedAlgorithm::Sorting);
        }

        // Output at most 3 labels
        detected.truncate(3);
        detected
    }
}

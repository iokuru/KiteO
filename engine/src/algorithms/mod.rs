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

        // 5. DSU
        if (source.contains("parent") || source.contains("parent[") || source.contains("root"))
            && (source.contains("find(") || source.contains("unite(") || source.contains("union("))
        {
            detected.push(AllowedAlgorithm::Dsu);
        }

        // 6. Dijkstra
        if (source.contains("priority_queue") || source.contains("PriorityQueue"))
            && (source.contains("dist[") || source.contains("dist.") || source.contains("weight"))
        {
            detected.push(AllowedAlgorithm::Dijkstra);
        }

        // 7. Topological Sort
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

        // 8. BFS
        if (source.contains("queue") || source.contains("Queue") || source.contains("LinkedList"))
            && (source.contains("vis[") || source.contains("visited") || source.contains("vis."))
            && !detected.contains(&AllowedAlgorithm::TopologicalSort)
            && !detected.contains(&AllowedAlgorithm::Dijkstra)
        {
            detected.push(AllowedAlgorithm::Bfs);
        }

        // 9. Tree DP
        if (source.contains("v != p")
            || source.contains("p != v")
            || source.contains("v != parent"))
            && (source.contains("dp[") || source.contains("sz[") || source.contains("sz."))
        {
            detected.push(AllowedAlgorithm::TreeDp);
        }

        // 10. DFS (if not Tree DP)
        if !detected.contains(&AllowedAlgorithm::TreeDp)
            && (source.contains("dfs(") || source.contains("dfs ("))
            && (source.contains("vis[") || source.contains("vis.") || source.contains("visited"))
        {
            detected.push(AllowedAlgorithm::Dfs);
        }

        // 11. Bitmask DP
        if (source.contains("1 << n")
            || source.contains("1<<n")
            || source.contains("1 << ")
            || source.contains("1<<"))
            && (source.contains("mask") || source.contains("dp["))
        {
            detected.push(AllowedAlgorithm::BitmaskDp);
        }

        // 12. Monotonic Stack
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

        // 13. Monotonic Queue
        if (source.contains("deque<") || source.contains("Deque") || source.contains("ArrayDeque"))
            && (source.contains("pop_front") || source.contains("pollFirst"))
            && (source.contains("pop_back") || source.contains("pollLast"))
        {
            detected.push(AllowedAlgorithm::MonotonicQueue);
        }

        // 14. Prefix Sum
        if (source.contains("pref[i - 1]")
            || source.contains("pref[i-1]")
            || source.contains("prefix[i - 1]")
            || source.contains("prefix[i-1]"))
            && !detected.contains(&AllowedAlgorithm::FenwickTree)
        {
            detected.push(AllowedAlgorithm::PrefixSum);
        }

        // 15. Binary Search on Answer
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

        // 16. Binary Search (if not Binary Search on Answer)
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

        // 17. Two Pointers
        if (source.contains("l < r") || source.contains("left < right"))
            && (source.contains("l++") || source.contains("left++"))
            && (source.contains("r--") || source.contains("right--"))
        {
            detected.push(AllowedAlgorithm::TwoPointers);
        }

        // 18. Sliding Window
        if (source.contains("for (int r = 0") || source.contains("for (int right = 0"))
            && (source.contains("while (") && (source.contains("l++") || source.contains("left++")))
        {
            detected.push(AllowedAlgorithm::SlidingWindow);
        }

        // 19. KMP
        if (source.contains("pi[") || source.contains("pi.") || source.contains("lps["))
            && (source.contains("pattern") || source.contains("needle") || source.contains("match"))
        {
            detected.push(AllowedAlgorithm::Kmp);
        }

        // 20. Trie
        if source.contains("child[26]")
            || source.contains("Node[26]")
            || source.contains("is_end")
            || source.contains("isEnd")
        {
            detected.push(AllowedAlgorithm::Trie);
        }

        // 21. Sorting (only when main technique)
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

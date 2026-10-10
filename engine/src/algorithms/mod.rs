use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AllowedAlgorithm {
    // Arrays, Hashing and Searching
    TwoPointers,
    FastAndSlowPointers,
    SlidingWindow,
    PrefixSum,
    DifferenceArray,
    BinarySearch,
    BinarySearchOnAnswer,
    Sorting,
    HashMap,
    HashSet,
    FrequencyCounting,
    KadanesAlgorithm,
    Intervals,
    MatrixGrid,
    CoordinateCompression,

    // Stack, Queue and Heap
    Stack,
    MonotonicStack,
    Queue,
    MonotonicQueue,
    HeapPriorityQueue,
    TopK,
    TwoHeaps,

    // Linked Lists
    LinkedList,
    LinkedListReversal,
    CycleDetection,
    MergeLinkedLists,

    // Trees
    Bfs,
    Dfs,
    BinarySearchTree,
    Trie,
    Lca,
    BinaryLifting,
    TreeDp,

    // Graphs
    Dsu,
    Dijkstra,
    Mst,
    TopologicalSort,
    ShortestPath,
    StronglyConnectedComponents,
    BridgesAndArticulationPoints,
    EulerianPath,

    // Dynamic Programming
    DynamicProgramming,
    Memoization,
    OneDDp,
    TwoDDp,
    KnapsackDp,
    BitmaskDp,
    IntervalDp,

    // String Algorithms
    Kmp,
    ZAlgorithm,
    RollingHash,

    // General Algorithms and Techniques
    Greedy,
    Backtracking,
    DivideAndConquer,
    BitManipulation,
    MeetInTheMiddle,
    SweepLine,

    // Advanced Data Structures
    FenwickTree,
    SegmentTree,
    SparseTable,
    LazySegmentTree,

    // Mathematics and Number Theory
    Sieve,
    Gcd,
    Lcm,
    FastExponentiation,
    ModularArithmetic,
    PrimeFactorization,
    Combinatorics,
}

impl AllowedAlgorithm {
    pub const ALL: [AllowedAlgorithm; 68] = [
        AllowedAlgorithm::SegmentTree,
        AllowedAlgorithm::FenwickTree,
        AllowedAlgorithm::SparseTable,
        AllowedAlgorithm::LazySegmentTree,
        AllowedAlgorithm::Dijkstra,
        AllowedAlgorithm::Mst,
        AllowedAlgorithm::StronglyConnectedComponents,
        AllowedAlgorithm::BridgesAndArticulationPoints,
        AllowedAlgorithm::EulerianPath,
        AllowedAlgorithm::TopologicalSort,
        AllowedAlgorithm::Lca,
        AllowedAlgorithm::BinaryLifting,
        AllowedAlgorithm::Trie,
        AllowedAlgorithm::Dsu,
        AllowedAlgorithm::Kmp,
        AllowedAlgorithm::ZAlgorithm,
        AllowedAlgorithm::RollingHash,
        AllowedAlgorithm::Sieve,
        AllowedAlgorithm::KadanesAlgorithm,
        AllowedAlgorithm::TreeDp,
        AllowedAlgorithm::BitmaskDp,
        AllowedAlgorithm::IntervalDp,
        AllowedAlgorithm::KnapsackDp,
        AllowedAlgorithm::TwoDDp,
        AllowedAlgorithm::OneDDp,
        AllowedAlgorithm::DynamicProgramming,
        AllowedAlgorithm::Memoization,
        AllowedAlgorithm::Backtracking,
        AllowedAlgorithm::DivideAndConquer,
        AllowedAlgorithm::BinarySearchOnAnswer,
        AllowedAlgorithm::TwoHeaps,
        AllowedAlgorithm::TopK,
        AllowedAlgorithm::HeapPriorityQueue,
        AllowedAlgorithm::MonotonicStack,
        AllowedAlgorithm::MonotonicQueue,
        AllowedAlgorithm::CycleDetection,
        AllowedAlgorithm::LinkedListReversal,
        AllowedAlgorithm::MergeLinkedLists,
        AllowedAlgorithm::FastAndSlowPointers,
        AllowedAlgorithm::SlidingWindow,
        AllowedAlgorithm::TwoPointers,
        AllowedAlgorithm::BinarySearch,
        AllowedAlgorithm::DifferenceArray,
        AllowedAlgorithm::PrefixSum,
        AllowedAlgorithm::CoordinateCompression,
        AllowedAlgorithm::Intervals,
        AllowedAlgorithm::MatrixGrid,
        AllowedAlgorithm::HashMap,
        AllowedAlgorithm::HashSet,
        AllowedAlgorithm::FrequencyCounting,
        AllowedAlgorithm::Stack,
        AllowedAlgorithm::Queue,
        AllowedAlgorithm::LinkedList,
        AllowedAlgorithm::BinarySearchTree,
        AllowedAlgorithm::ShortestPath,
        AllowedAlgorithm::Greedy,
        AllowedAlgorithm::BitManipulation,
        AllowedAlgorithm::MeetInTheMiddle,
        AllowedAlgorithm::SweepLine,
        AllowedAlgorithm::Gcd,
        AllowedAlgorithm::Lcm,
        AllowedAlgorithm::FastExponentiation,
        AllowedAlgorithm::ModularArithmetic,
        AllowedAlgorithm::PrimeFactorization,
        AllowedAlgorithm::Combinatorics,
        AllowedAlgorithm::Sorting,
        AllowedAlgorithm::Bfs,
        AllowedAlgorithm::Dfs,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SegmentTree => "Segment Tree",
            Self::FenwickTree => "Fenwick Tree",
            Self::SparseTable => "Sparse Table",
            Self::LazySegmentTree => "Lazy Segment Tree",
            Self::Dijkstra => "Dijkstra",
            Self::Mst => "MST",
            Self::StronglyConnectedComponents => "Strongly Connected Components",
            Self::BridgesAndArticulationPoints => "Bridges and Articulation Points",
            Self::EulerianPath => "Eulerian Path",
            Self::TopologicalSort => "Topological Sort",
            Self::Lca => "LCA",
            Self::BinaryLifting => "Binary Lifting",
            Self::Trie => "Trie",
            Self::Dsu => "DSU",
            Self::Kmp => "KMP",
            Self::ZAlgorithm => "Z Algorithm",
            Self::RollingHash => "Rolling Hash",
            Self::Sieve => "Sieve",
            Self::KadanesAlgorithm => "Kadane's Algorithm",
            Self::TreeDp => "Tree DP",
            Self::BitmaskDp => "Bitmask DP",
            Self::IntervalDp => "Interval DP",
            Self::KnapsackDp => "Knapsack DP",
            Self::TwoDDp => "2D DP",
            Self::OneDDp => "1D DP",
            Self::DynamicProgramming => "Dynamic Programming",
            Self::Memoization => "Memoization",
            Self::Backtracking => "Backtracking",
            Self::DivideAndConquer => "Divide and Conquer",
            Self::BinarySearchOnAnswer => "Binary Search on Answer",
            Self::TwoHeaps => "Two Heaps",
            Self::TopK => "Top K",
            Self::HeapPriorityQueue => "Heap / Priority Queue",
            Self::MonotonicStack => "Monotonic Stack",
            Self::MonotonicQueue => "Monotonic Queue",
            Self::CycleDetection => "Cycle Detection",
            Self::LinkedListReversal => "Linked List Reversal",
            Self::MergeLinkedLists => "Merge Linked Lists",
            Self::FastAndSlowPointers => "Fast and Slow Pointers",
            Self::SlidingWindow => "Sliding Window",
            Self::TwoPointers => "Two Pointers",
            Self::BinarySearch => "Binary Search",
            Self::DifferenceArray => "Difference Array",
            Self::PrefixSum => "Prefix Sum",
            Self::CoordinateCompression => "Coordinate Compression",
            Self::Intervals => "Intervals",
            Self::MatrixGrid => "Matrix / Grid",
            Self::HashMap => "Hash Map",
            Self::HashSet => "Hash Set",
            Self::FrequencyCounting => "Frequency Counting",
            Self::Stack => "Stack",
            Self::Queue => "Queue",
            Self::LinkedList => "Linked List",
            Self::BinarySearchTree => "Binary Search Tree",
            Self::ShortestPath => "Shortest Path",
            Self::Greedy => "Greedy",
            Self::BitManipulation => "Bit Manipulation",
            Self::MeetInTheMiddle => "Meet in the Middle",
            Self::SweepLine => "Sweep Line",
            Self::Gcd => "GCD",
            Self::Lcm => "LCM",
            Self::FastExponentiation => "Fast Exponentiation",
            Self::ModularArithmetic => "Modular Arithmetic",
            Self::PrimeFactorization => "Prime Factorization",
            Self::Combinatorics => "Combinatorics",
            Self::Sorting => "Sorting",
            Self::Bfs => "BFS",
            Self::Dfs => "DFS",
        }
    }

    pub fn from_canonical_name(s: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|a| a.as_str() == s)
    }

    pub fn priority(&self) -> u32 {
        let name = self.as_str();
        get_catalog()
            .iter()
            .find(|item| item.name == name)
            .map(|item| item.priority)
            .unwrap_or(999)
    }

    pub fn is_approved(&self) -> bool {
        let name = self.as_str();
        get_catalog()
            .iter()
            .find(|item| item.name == name)
            .map(|item| item.status == "approved")
            .unwrap_or(false)
    }
}

impl std::str::FromStr for AllowedAlgorithm {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_canonical_name(s).ok_or(())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CatalogItem {
    pub name: String,
    pub status: String,
    pub category: String,
    pub priority: u32,
    #[serde(default)]
    pub suppresses: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct CatalogDoc {
    pub algorithm: Vec<CatalogItem>,
}

pub fn get_catalog() -> &'static [CatalogItem] {
    static CATALOG: std::sync::OnceLock<Vec<CatalogItem>> = std::sync::OnceLock::new();
    CATALOG.get_or_init(|| {
        let raw = include_str!("catalog.toml");
        let doc: CatalogDoc = toml::from_str(raw).expect("Failed to parse catalog.toml");
        doc.algorithm
    })
}

pub fn select_labels(
    detected: &[AllowedAlgorithm],
    source: &str,
    require_approved: bool,
) -> Vec<AllowedAlgorithm> {
    let mut current: std::collections::HashSet<AllowedAlgorithm> = if require_approved {
        detected
            .iter()
            .filter(|algo| algo.is_approved())
            .copied()
            .collect()
    } else {
        detected.iter().copied().collect()
    };

    // 1. Most specific wins (suppression rules)
    // Tree DP suppresses Dynamic Programming and DFS
    if current.contains(&AllowedAlgorithm::TreeDp) {
        current.remove(&AllowedAlgorithm::DynamicProgramming);
        current.remove(&AllowedAlgorithm::Dfs);
    }

    // Bitmask DP suppresses Dynamic Programming
    if current.contains(&AllowedAlgorithm::BitmaskDp) {
        current.remove(&AllowedAlgorithm::DynamicProgramming);
    }

    // Interval DP suppresses Dynamic Programming
    if current.contains(&AllowedAlgorithm::IntervalDp) {
        current.remove(&AllowedAlgorithm::DynamicProgramming);
    }

    // Knapsack DP suppresses Dynamic Programming
    if current.contains(&AllowedAlgorithm::KnapsackDp) {
        current.remove(&AllowedAlgorithm::DynamicProgramming);
    }

    // 2D DP suppresses Dynamic Programming
    if current.contains(&AllowedAlgorithm::TwoDDp) {
        current.remove(&AllowedAlgorithm::DynamicProgramming);
    }

    // 1D DP suppresses Dynamic Programming
    if current.contains(&AllowedAlgorithm::OneDDp) {
        current.remove(&AllowedAlgorithm::DynamicProgramming);
    }

    // Dijkstra suppresses BFS and Heap / Priority Queue
    if current.contains(&AllowedAlgorithm::Dijkstra) {
        current.remove(&AllowedAlgorithm::Bfs);
        current.remove(&AllowedAlgorithm::HeapPriorityQueue);
    }

    // Topological Sort suppresses BFS and DFS
    if current.contains(&AllowedAlgorithm::TopologicalSort) {
        current.remove(&AllowedAlgorithm::Bfs);
        current.remove(&AllowedAlgorithm::Dfs);
    }

    // Strongly Connected Components suppresses DFS
    if current.contains(&AllowedAlgorithm::StronglyConnectedComponents) {
        current.remove(&AllowedAlgorithm::Dfs);
    }

    // Bridges and Articulation Points suppresses DFS
    if current.contains(&AllowedAlgorithm::BridgesAndArticulationPoints) {
        current.remove(&AllowedAlgorithm::Dfs);
    }

    // Eulerian Path suppresses DFS
    if current.contains(&AllowedAlgorithm::EulerianPath) {
        current.remove(&AllowedAlgorithm::Dfs);
    }

    // Binary Search on Answer suppresses Binary Search
    if current.contains(&AllowedAlgorithm::BinarySearchOnAnswer) {
        current.remove(&AllowedAlgorithm::BinarySearch);
    }

    // Sliding Window suppresses Two Pointers
    if current.contains(&AllowedAlgorithm::SlidingWindow) {
        current.remove(&AllowedAlgorithm::TwoPointers);
    }

    // Top K and Two Heaps suppress Heap / Priority Queue
    if current.contains(&AllowedAlgorithm::TopK) || current.contains(&AllowedAlgorithm::TwoHeaps) {
        current.remove(&AllowedAlgorithm::HeapPriorityQueue);
    }

    // Monotonic Stack suppresses Stack
    if current.contains(&AllowedAlgorithm::MonotonicStack) {
        current.remove(&AllowedAlgorithm::Stack);
    }

    // Monotonic Queue suppresses Queue
    if current.contains(&AllowedAlgorithm::MonotonicQueue) {
        current.remove(&AllowedAlgorithm::Queue);
    }

    // Lazy Segment Tree suppresses Segment Tree
    if current.contains(&AllowedAlgorithm::LazySegmentTree) {
        current.remove(&AllowedAlgorithm::SegmentTree);
    }

    // Fenwick Tree and Segment Tree suppress Prefix Sum only when the prefix sums are computed through the tree
    if (current.contains(&AllowedAlgorithm::FenwickTree)
        || current.contains(&AllowedAlgorithm::SegmentTree)
        || current.contains(&AllowedAlgorithm::LazySegmentTree))
        && !source.contains("pref[")
        && !source.contains("prefix[")
    {
        current.remove(&AllowedAlgorithm::PrefixSum);
    }

    let mut result: Vec<AllowedAlgorithm> = current.into_iter().collect();

    // Deterministic priority ordering: lower priority number = higher precedence
    result.sort_by_key(|a| a.priority());

    // Keep at most 3 labels with the highest priority
    if result.len() > 3 {
        result.truncate(3);
    }

    result
}

pub fn apply_selection_rules(detected: &[AllowedAlgorithm], source: &str) -> Vec<AllowedAlgorithm> {
    select_labels(detected, source, true)
}

pub struct AlgorithmDetector;

impl AlgorithmDetector {
    pub fn detect(source: &str) -> Vec<AllowedAlgorithm> {
        apply_selection_rules(&Self::detect_raw(source), source)
    }

    pub fn detect_raw(source: &str) -> Vec<AllowedAlgorithm> {
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
        if (source.contains("queue")
            || source.contains("Queue")
            || source.contains("LinkedList")
            || source.contains("deque")
            || source.contains("Deque")
            || source.contains("popleft")
            || source.contains("bfs(")
            || source.contains("bfs (")
            || source.contains("def bfs")
            || source.contains("void bfs"))
            && (source.contains("vis[")
                || source.contains("visited")
                || source.contains("vis.")
                || source.contains("grid[")
                || source.contains("grid.")
                || source.contains("board[")
                || source.contains("matrix["))
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

        // 13. DFS
        if !detected.contains(&AllowedAlgorithm::TreeDp)
            && (source.contains("dfs(") || source.contains("dfs (") || source.contains("dfs "))
            && (source.contains("vis[")
                || source.contains("vis.")
                || source.contains("visited")
                || source.contains("grid[")
                || source.contains("grid.")
                || source.contains("board[")
                || source.contains("matrix[")
                || source.contains("dfs(grid")
                || source.contains("dfs (grid")
                || source.contains("dfs(r")
                || source.contains("dfs(i"))
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

        // 19. Binary Search
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

        // 25. Kadane's Algorithm
        let has_kadane_reset = (source.contains("< 0") || source.contains("<= 0"))
            && (source.contains("= 0;") || source.contains("= 0\n"));
        let has_kadane_max_choice = (source.contains("max(") || source.contains("Math.max("))
            && (source.contains("cur + nums[")
                || source.contains("cur + a[")
                || source.contains("cur + arr[")
                || source.contains("cur_max + nums[")
                || source.contains("cur_max + a[")
                || source.contains("cur_max + arr[")
                || source.contains("max(0,")
                || source.contains("max(0LL,")
                || source.contains("Math.max(0,"));
        let tracks_global_max = (source.contains("max_so_far")
            || source.contains("ans")
            || source.contains("max_sum")
            || source.contains("res")
            || source.contains("max("))
            && (source.contains("max(") || source.contains("Math.max("));
        let not_sliding_window = !source.contains("l++") && !source.contains("left++");

        if (has_kadane_reset || has_kadane_max_choice) && tracks_global_max && not_sliding_window {
            detected.push(AllowedAlgorithm::KadanesAlgorithm);
        }

        detected
    }
}

export type SupportedLanguage = 'cpp' | 'java';

export type AllowedAlgorithm =
  | 'Binary Search'
  | 'Binary Search on Answer'
  | 'Two Pointers'
  | 'Sliding Window'
  | 'Prefix Sum'
  | 'Sorting'
  | 'BFS'
  | 'DFS'
  | 'Topological Sort'
  | 'DSU'
  | 'Dijkstra'
  | 'MST'
  | 'Fenwick Tree'
  | 'Segment Tree'
  | 'Sparse Table'
  | 'Monotonic Stack'
  | 'Monotonic Queue'
  | 'Binary Lifting'
  | 'LCA'
  | 'Tree DP'
  | 'Bitmask DP'
  | 'Sieve'
  | 'KMP'
  | 'Z Algorithm'
  | 'Rolling Hash'
  | 'Trie';

export type ConstraintStatus = 'Within limits' | 'Likely too slow' | "Can't tell";

export interface AnalysisResult {
  tc: string;
  sc: string;
  algorithms: AllowedAlgorithm[];
  constraintStatus?: ConstraintStatus;
}

export interface PlatformAdapter {
  id: string;
  name: string;
  isMatch(url: string): boolean;
  isEditorAvailable(): boolean;
  getSource(): string | null;
  getLanguage(): SupportedLanguage | null;
  getConstraints?(): string | null;
  isLiveContest?(): boolean;
}

import json
import os

HELD_OUT_CASES = [
    # -------------------------------------------------------------
    # Rating: 800 (Codeforces)
    # -------------------------------------------------------------
    {
        "id": "cf_4a_watermelon_cpp",
        "name": "CF 4A Watermelon (C++)",
        "language": "cpp",
        "rating": "800",
        "tags": ["math", "cf"],
        "expected_tc": "O(1)",
        "expected_sc": "O(1)",
        "expected_algorithms": [],
        "code": """#include <iostream>
using namespace std;
int main() {
    int w;
    if (cin >> w) {
        if (w > 2 && w % 2 == 0) cout << "YES\\n";
        else cout << "NO\\n";
    }
    return 0;
}"""
    },
    {
        "id": "cf_4a_watermelon_java",
        "name": "CF 4A Watermelon (Java)",
        "language": "java",
        "rating": "800",
        "tags": ["math", "cf"],
        "expected_tc": "O(1)",
        "expected_sc": "O(1)",
        "expected_algorithms": [],
        "code": """import java.util.Scanner;
public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        if (sc.hasNextInt()) {
            int w = sc.nextInt();
            if (w > 2 && w % 2 == 0) System.out.println("YES");
            else System.out.println("NO");
        }
    }
}"""
    },
    {
        "id": "cf_231a_team_cpp",
        "name": "CF 231A Team (C++)",
        "language": "cpp",
        "rating": "800",
        "tags": ["linear", "cf"],
        "expected_tc": "O(n)",
        "expected_sc": "O(1)",
        "expected_algorithms": [],
        "code": """#include <iostream>
using namespace std;
int main() {
    int n;
    if (!(cin >> n)) return 0;
    int ans = 0;
    for (int i = 0; i < n; i++) {
        int a, b, c;
        cin >> a >> b >> c;
        if (a + b + c >= 2) ans++;
    }
    cout << ans << "\\n";
    return 0;
}"""
    },
    {
        "id": "cf_231a_team_java",
        "name": "CF 231A Team (Java)",
        "language": "java",
        "rating": "800",
        "tags": ["linear", "cf"],
        "expected_tc": "O(n)",
        "expected_sc": "O(1)",
        "expected_algorithms": [],
        "code": """import java.util.Scanner;
public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        if (!sc.hasNextInt()) return;
        int n = sc.nextInt();
        int ans = 0;
        for (int i = 0; i < n; i++) {
            int a = sc.nextInt();
            int b = sc.nextInt();
            int c = sc.nextInt();
            if (a + b + c >= 2) ans++;
        }
        System.out.println(ans);
    }
}"""
    },
    {
        "id": "cf_71a_way_too_long_words_cpp",
        "name": "CF 71A Way Too Long Words (C++)",
        "language": "cpp",
        "rating": "800",
        "tags": ["string", "linear", "cf"],
        "expected_tc": "O(n)",
        "expected_sc": "O(1)",
        "expected_algorithms": [],
        "code": """#include <iostream>
#include <string>
using namespace std;
int main() {
    int n;
    if (!(cin >> n)) return 0;
    for (int i = 0; i < n; i++) {
        string s;
        cin >> s;
        if (s.length() > 10) {
            cout << s[0] << s.length() - 2 << s.back() << "\\n";
        } else {
            cout << s << "\\n";
        }
    }
    return 0;
}"""
    },
    {
        "id": "cf_71a_way_too_long_words_java",
        "name": "CF 71A Way Too Long Words (Java)",
        "language": "java",
        "rating": "800",
        "tags": ["string", "linear", "cf"],
        "expected_tc": "O(n)",
        "expected_sc": "O(1)",
        "expected_algorithms": [],
        "code": """import java.util.Scanner;
public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        if (!sc.hasNextInt()) return;
        int n = sc.nextInt();
        for (int i = 0; i < n; i++) {
            String s = sc.next();
            if (s.length() > 10) {
                System.out.println("" + s.charAt(0) + (s.length() - 2) + s.charAt(s.length() - 1));
            } else {
                System.out.println(s);
            }
        }
    }
}"""
    },

    # -------------------------------------------------------------
    # Rating: 900 (Codeforces)
    # -------------------------------------------------------------
    {
        "id": "cf_160a_twins_cpp",
        "name": "CF 160A Twins (C++)",
        "language": "cpp",
        "rating": "900",
        "tags": ["sorting", "greedy", "cf"],
        "expected_tc": "O(n log n)",
        "expected_sc": "O(n)",
        "expected_algorithms": ["Sorting"],
        "code": """#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;
int main() {
    int n;
    if (!(cin >> n)) return 0;
    vector<int> a(n);
    int total = 0;
    for (int i = 0; i < n; i++) {
        cin >> a[i];
        total += a[i];
    }
    sort(a.rbegin(), a.rend());
    int take = 0, count = 0;
    for (int i = 0; i < n; i++) {
        take += a[i];
        count++;
        if (take > total - take) break;
    }
    cout << count << "\\n";
    return 0;
}"""
    },
    {
        "id": "cf_160a_twins_java",
        "name": "CF 160A Twins (Java)",
        "language": "java",
        "rating": "900",
        "tags": ["sorting", "greedy", "cf"],
        "expected_tc": "O(n log n)",
        "expected_sc": "O(n)",
        "expected_algorithms": ["Sorting"],
        "code": """import java.util.Scanner;
import java.util.Arrays;
public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        if (!sc.hasNextInt()) return;
        int n = sc.nextInt();
        int[] a = new int[n];
        int total = 0;
        for (int i = 0; i < n; i++) {
            a[i] = sc.nextInt();
            total += a[i];
        }
        Arrays.sort(a);
        int take = 0, count = 0;
        for (int i = n - 1; i >= 0; i--) {
            take += a[i];
            count++;
            if (take > total - take) break;
        }
        System.out.println(count);
    }
}"""
    },
    {
        "id": "cf_96a_football_cpp",
        "name": "CF 96A Football (C++)",
        "language": "cpp",
        "rating": "900",
        "tags": ["two_pointers", "string", "cf"],
        "expected_tc": "O(n)",
        "expected_sc": "O(1)",
        "expected_algorithms": ["Two Pointers"],
        "code": """#include <iostream>
#include <string>
using namespace std;
int main() {
    string s;
    if (!(cin >> s)) return 0;
    int left = 0, n = s.length();
    bool dangerous = false;
    for (int right = 0; right < n; right++) {
        if (s[right] != s[left]) {
            left = right;
        }
        if (right - left + 1 >= 7) {
            dangerous = true;
            break;
        }
    }
    if (dangerous) cout << "YES\\n";
    else cout << "NO\\n";
    return 0;
}"""
    },
    {
        "id": "cf_96a_football_java",
        "name": "CF 96A Football (Java)",
        "language": "java",
        "rating": "900",
        "tags": ["two_pointers", "string", "cf"],
        "expected_tc": "O(n)",
        "expected_sc": "O(1)",
        "expected_algorithms": ["Two Pointers"],
        "code": """import java.util.Scanner;
public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        if (!sc.hasNext()) return;
        String s = sc.next();
        int left = 0, n = s.length();
        boolean dangerous = false;
        for (int right = 0; right < n; right++) {
            if (s.charAt(right) != s.charAt(left)) {
                left = right;
            }
            if (right - left + 1 >= 7) {
                dangerous = true;
                break;
            }
        }
        if (dangerous) System.out.println("YES");
        else System.out.println("NO");
    }
}"""
    },

    # -------------------------------------------------------------
    # Rating: 1000 (Codeforces)
    # -------------------------------------------------------------
    {
        "id": "cf_69a_young_physicist_cpp",
        "name": "CF 69A Young Physicist (C++)",
        "language": "cpp",
        "rating": "1000",
        "tags": ["linear", "math", "cf"],
        "expected_tc": "O(n)",
        "expected_sc": "O(1)",
        "expected_algorithms": [],
        "code": """#include <iostream>
using namespace std;
int main() {
    int n;
    if (!(cin >> n)) return 0;
    int sx = 0, sy = 0, sz = 0;
    for (int i = 0; i < n; i++) {
        int x, y, z;
        cin >> x >> y >> z;
        sx += x; sy += y; sz += z;
    }
    if (sx == 0 && sy == 0 && sz == 0) cout << "YES\\n";
    else cout << "NO\\n";
    return 0;
}"""
    },
    {
        "id": "cf_69a_young_physicist_java",
        "name": "CF 69A Young Physicist (Java)",
        "language": "java",
        "rating": "1000",
        "tags": ["linear", "math", "cf"],
        "expected_tc": "O(n)",
        "expected_sc": "O(1)",
        "expected_algorithms": [],
        "code": """import java.util.Scanner;
public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        if (!sc.hasNextInt()) return;
        int n = sc.nextInt();
        int sx = 0, sy = 0, sz = 0;
        for (int i = 0; i < n; i++) {
            sx += sc.nextInt();
            sy += sc.nextInt();
            sz += sc.nextInt();
        }
        if (sx == 0 && sy == 0 && sz == 0) System.out.println("YES");
        else System.out.println("NO");
    }
}"""
    },

    # -------------------------------------------------------------
    # Rating: 1100 (Codeforces)
    # -------------------------------------------------------------
    {
        "id": "cf_313b_ilya_and_queries_cpp",
        "name": "CF 313B Ilya and Queries (C++)",
        "language": "cpp",
        "rating": "1100",
        "tags": ["prefix_sum", "cf"],
        "expected_tc": "O(n + q)",
        "expected_sc": "O(n)",
        "expected_algorithms": ["Prefix Sum"],
        "code": """#include <iostream>
#include <string>
#include <vector>
using namespace std;
int main() {
    ios::sync_with_stdio(false); cin.tie(nullptr);
    string s;
    if (!(cin >> s)) return 0;
    int n = s.length();
    vector<int> pref(n + 1, 0);
    for (int i = 1; i < n; i++) {
        pref[i] = pref[i - 1] + (s[i] == s[i - 1] ? 1 : 0);
    }
    int m;
    if (cin >> m) {
        while (m--) {
            int l, r;
            cin >> l >> r;
            cout << pref[r - 1] - pref[l - 1] << "\\n";
        }
    }
    return 0;
}"""
    },
    {
        "id": "cf_313b_ilya_and_queries_java",
        "name": "CF 313B Ilya and Queries (Java)",
        "language": "java",
        "rating": "1100",
        "tags": ["prefix_sum", "cf"],
        "expected_tc": "O(n + q)",
        "expected_sc": "O(n)",
        "expected_algorithms": ["Prefix Sum"],
        "code": """import java.util.Scanner;
public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        if (!sc.hasNext()) return;
        String s = sc.next();
        int n = s.length();
        int[] pref = new int[n + 1];
        for (int i = 1; i < n; i++) {
            pref[i] = pref[i - 1] + (s.charAt(i) == s.charAt(i - 1) ? 1 : 0);
        }
        if (sc.hasNextInt()) {
            int m = sc.nextInt();
            for (int k = 0; k < m; k++) {
                int l = sc.nextInt();
                int r = sc.nextInt();
                System.out.println(pref[r - 1] - pref[l - 1]);
            }
        }
    }
}"""
    },

    # -------------------------------------------------------------
    # Rating: 1200 (Codeforces)
    # -------------------------------------------------------------
    {
        "id": "cf_492b_vanya_and_lanterns_cpp",
        "name": "CF 492B Vanya and Lanterns (C++)",
        "language": "cpp",
        "rating": "1200",
        "tags": ["sorting", "math", "cf"],
        "expected_tc": "O(n log n)",
        "expected_sc": "O(n)",
        "expected_algorithms": ["Sorting"],
        "code": """#include <iostream>
#include <vector>
#include <algorithm>
#include <iomanip>
using namespace std;
int main() {
    int n;
    long long l;
    if (!(cin >> n >> l)) return 0;
    vector<long long> a(n);
    for (int i = 0; i < n; i++) cin >> a[i];
    sort(a.begin(), a.end());
    double max_d = max((double)a[0], (double)(l - a[n - 1]));
    for (int i = 1; i < n; i++) {
        max_d = max(max_d, (double)(a[i] - a[i - 1]) / 2.0);
    }
    cout << fixed << setprecision(10) << max_d << "\\n";
    return 0;
}"""
    },
    {
        "id": "cf_492b_vanya_and_lanterns_java",
        "name": "CF 492B Vanya and Lanterns (Java)",
        "language": "java",
        "rating": "1200",
        "tags": ["sorting", "math", "cf"],
        "expected_tc": "O(n log n)",
        "expected_sc": "O(n)",
        "expected_algorithms": ["Sorting"],
        "code": """import java.util.Scanner;
import java.util.Arrays;
public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        if (!sc.hasNextInt()) return;
        int n = sc.nextInt();
        long l = sc.nextLong();
        long[] a = new long[n];
        for (int i = 0; i < n; i++) a[i] = sc.nextLong();
        Arrays.sort(a);
        double maxD = Math.max((double)a[0], (double)(l - a[n - 1]));
        for (int i = 1; i < n; i++) {
            maxD = Math.max(maxD, (double)(a[i] - a[i - 1]) / 2.0);
        }
        System.out.printf(java.util.Locale.US, "%.10f\\n", maxD);
    }
}"""
    },

    # -------------------------------------------------------------
    # Rating: 1300 (Codeforces)
    # -------------------------------------------------------------
    {
        "id": "cf_451b_sort_the_array_cpp",
        "name": "CF 451B Sort the Array (C++)",
        "language": "cpp",
        "rating": "1300",
        "tags": ["two_pointers", "sorting", "cf"],
        "expected_tc": "O(n)",
        "expected_sc": "O(n)",
        "expected_algorithms": ["Two Pointers"],
        "code": """#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;
int main() {
    int n;
    if (!(cin >> n)) return 0;
    vector<int> a(n);
    for (int i = 0; i < n; i++) cin >> a[i];
    int l = 0, r = n - 1;
    while (l < n - 1 && a[l] <= a[l + 1]) l++;
    if (l == n - 1) {
        cout << "yes\\n1 1\\n";
        return 0;
    }
    while (r > 0 && a[r] >= a[r - 1]) r--;
    reverse(a.begin() + l, a.begin() + r + 1);
    bool ok = true;
    for (int i = 0; i < n - 1; i++) {
        if (a[i] > a[i + 1]) { ok = false; break; }
    }
    if (ok) cout << "yes\\n" << (l + 1) << " " << (r + 1) << "\\n";
    else cout << "no\\n";
    return 0;
}"""
    },
    {
        "id": "cf_451b_sort_the_array_java",
        "name": "CF 451B Sort the Array (Java)",
        "language": "java",
        "rating": "1300",
        "tags": ["two_pointers", "sorting", "cf"],
        "expected_tc": "O(n)",
        "expected_sc": "O(n)",
        "expected_algorithms": ["Two Pointers"],
        "code": """import java.util.Scanner;
public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        if (!sc.hasNextInt()) return;
        int n = sc.nextInt();
        int[] a = new int[n];
        for (int i = 0; i < n; i++) a[i] = sc.nextInt();
        int l = 0, r = n - 1;
        while (l < n - 1 && a[l] <= a[l + 1]) l++;
        if (l == n - 1) {
            System.out.println("yes\\n1 1");
            return;
        }
        while (r > 0 && a[r] >= a[r - 1]) r--;
        for (int i = l, j = r; i < j; i++, j--) {
            int tmp = a[i]; a[i] = a[j]; a[j] = tmp;
        }
        boolean ok = true;
        for (int i = 0; i < n - 1; i++) {
            if (a[i] > a[i + 1]) { ok = false; break; }
        }
        if (ok) System.out.println("yes\\n" + (l + 1) + " " + (r + 1));
        else System.out.println("no");
    }
}"""
    },

    # -------------------------------------------------------------
    # Rating: 1400 (Codeforces)
    # -------------------------------------------------------------
    {
        "id": "cf_520b_two_buttons_cpp",
        "name": "CF 520B Two Buttons (C++)",
        "language": "cpp",
        "rating": "1400",
        "tags": ["bfs", "graph", "cf"],
        "expected_tc": "O(V + E)",
        "expected_sc": "O(V + E)",
        "expected_algorithms": ["BFS"],
        "code": """#include <iostream>
#include <vector>
#include <queue>
using namespace std;
int main() {
    int n, m;
    if (!(cin >> n >> m)) return 0;
    const int MAX = 20000;
    vector<int> dist(MAX + 1, -1);
    queue<int> q;
    dist[n] = 0;
    q.push(n);
    while (!q.empty()) {
        int u = q.front();
        q.pop();
        if (u == m) break;
        if (u * 2 <= MAX && dist[u * 2] == -1) {
            dist[u * 2] = dist[u] + 1;
            q.push(u * 2);
        }
        if (u - 1 > 0 && dist[u - 1] == -1) {
            dist[u - 1] = dist[u] + 1;
            q.push(u - 1);
        }
    }
    cout << dist[m] << "\\n";
    return 0;
}"""
    },
    {
        "id": "cf_520b_two_buttons_java",
        "name": "CF 520B Two Buttons (Java)",
        "language": "java",
        "rating": "1400",
        "tags": ["bfs", "graph", "cf"],
        "expected_tc": "O(V + E)",
        "expected_sc": "O(V + E)",
        "expected_algorithms": ["BFS"],
        "code": """import java.util.Scanner;
import java.util.Queue;
import java.util.LinkedList;
import java.util.Arrays;
public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        if (!sc.hasNextInt()) return;
        int n = sc.nextInt();
        int m = sc.nextInt();
        int max = 20000;
        int[] dist = new int[max + 1];
        Arrays.fill(dist, -1);
        Queue<Integer> q = new LinkedList<>();
        dist[n] = 0;
        q.offer(n);
        while (!q.isEmpty()) {
            int u = q.poll();
            if (u == m) break;
            if (u * 2 <= max && dist[u * 2] == -1) {
                dist[u * 2] = dist[u] + 1;
                q.offer(u * 2);
            }
            if (u - 1 > 0 && dist[u - 1] == -1) {
                dist[u - 1] = dist[u] + 1;
                q.offer(u - 1);
            }
        }
        System.out.println(dist[m]);
    }
}"""
    },

    # -------------------------------------------------------------
    # Rating: 1500 (Codeforces)
    # -------------------------------------------------------------
    {
        "id": "cf_580c_kefa_and_park_cpp",
        "name": "CF 580C Kefa and Park (C++)",
        "language": "cpp",
        "rating": "1500",
        "tags": ["dfs", "tree", "cf"],
        "expected_tc": "O(V + E)",
        "expected_sc": "O(V + E)",
        "expected_algorithms": ["DFS"],
        "code": """#include <iostream>
#include <vector>
using namespace std;
int n, m;
vector<int> has_cat;
vector<vector<int>> adj;
int ans = 0;
void dfs(int u, int p, int consecutive) {
    if (has_cat[u]) consecutive++;
    else consecutive = 0;
    if (consecutive > m) return;
    bool is_leaf = true;
    for (int v : adj[u]) {
        if (v != p) {
            is_leaf = false;
            dfs(v, u, consecutive);
        }
    }
    if (is_leaf && u != 1) ans++;
}
int main() {
    if (!(cin >> n >> m)) return 0;
    has_cat.resize(n + 1);
    adj.resize(n + 1);
    for (int i = 1; i <= n; i++) cin >> has_cat[i];
    for (int i = 0; i < n - 1; i++) {
        int u, v; cin >> u >> v;
        adj[u].push_back(v);
        adj[v].push_back(u);
    }
    dfs(1, 0, 0);
    cout << ans << "\\n";
    return 0;
}"""
    },
    {
        "id": "cf_580c_kefa_and_park_java",
        "name": "CF 580C Kefa and Park (Java)",
        "language": "java",
        "rating": "1500",
        "tags": ["dfs", "tree", "cf"],
        "expected_tc": "O(V + E)",
        "expected_sc": "O(V + E)",
        "expected_algorithms": ["DFS"],
        "code": """import java.util.*;
public class Main {
    static int n, m, ans = 0;
    static int[] hasCat;
    static List<Integer>[] adj;
    static void dfs(int u, int p, int consecutive) {
        if (hasCat[u] == 1) consecutive++;
        else consecutive = 0;
        if (consecutive > m) return;
        boolean isLeaf = true;
        for (int v : adj[u]) {
            if (v != p) {
                isLeaf = false;
                dfs(v, u, consecutive);
            }
        }
        if (isLeaf && u != 1) ans++;
    }
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        if (!sc.hasNextInt()) return;
        n = sc.nextInt(); m = sc.nextInt();
        hasCat = new int[n + 1];
        adj = new ArrayList[n + 1];
        for (int i = 1; i <= n; i++) { hasCat[i] = sc.nextInt(); adj[i] = new ArrayList<>(); }
        for (int i = 0; i < n - 1; i++) {
            int u = sc.nextInt(), v = sc.nextInt();
            adj[u].add(v); adj[v].add(u);
        }
        dfs(1, 0, 0);
        System.out.println(ans);
    }
}"""
    },

    # -------------------------------------------------------------
    # Rating: 1600 (Codeforces)
    # -------------------------------------------------------------
    {
        "id": "cf_1899e_queue_sort_heldout_cpp",
        "name": "CF 1899E Queue Sort (C++)",
        "language": "cpp",
        "rating": "1600",
        "tags": ["linear", "cf"],
        "expected_tc": "O(n)",
        "expected_sc": "O(1)",
        "expected_algorithms": [],
        "code": """#include <iostream>
#include <vector>
using namespace std;
void solve() {
    int n;
    cin >> n;
    vector<int> a(n);
    int min_val = 2e9, min_idx = 0;
    for (int i = 0; i < n; i++) {
        cin >> a[i];
        if (a[i] < min_val) {
            min_val = a[i];
            min_idx = i;
        }
    }
    for (int i = min_idx; i < n - 1; i++) {
        if (a[i] > a[i + 1]) {
            cout << -1 << "\\n";
            return;
        }
    }
    cout << min_idx << "\\n";
}
int main() {
    int t;
    if (cin >> t) {
        while (t--) solve();
    }
    return 0;
}"""
    },
    {
        "id": "cf_1899e_queue_sort_heldout_java",
        "name": "CF 1899E Queue Sort (Java)",
        "language": "java",
        "rating": "1600",
        "tags": ["linear", "cf"],
        "expected_tc": "O(n)",
        "expected_sc": "O(1)",
        "expected_algorithms": [],
        "code": """import java.util.Scanner;
public class Main {
    static void solve(Scanner sc) {
        int n = sc.nextInt();
        int[] a = new int[n];
        int minVal = Integer.MAX_VALUE, minIdx = 0;
        for (int i = 0; i < n; i++) {
            a[i] = sc.nextInt();
            if (a[i] < minVal) {
                minVal = a[i];
                minIdx = i;
            }
        }
        for (int i = minIdx; i < n - 1; i++) {
            if (a[i] > a[i + 1]) {
                System.out.println(-1);
                return;
            }
        }
        System.out.println(minIdx);
    }
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        if (sc.hasNextInt()) {
            int t = sc.nextInt();
            while (t-- > 0) solve(sc);
        }
    }
}"""
    },

    # -------------------------------------------------------------
    # Rating: 1700 (Codeforces)
    # -------------------------------------------------------------
    {
        "id": "cf_1873e_building_aquarium_heldout_cpp",
        "name": "CF 1873E Building an Aquarium (C++)",
        "language": "cpp",
        "rating": "1700",
        "tags": ["binary_search_on_answer", "cf"],
        "expected_tc": "O(n log A)",
        "expected_sc": "O(1)",
        "expected_algorithms": ["Binary Search on Answer"],
        "code": """#include <iostream>
#include <vector>
using namespace std;
bool check(const vector<long long>& a, long long h, long long x) {
    long long water = 0;
    for (long long ai : a) {
        if (h > ai) water += (h - ai);
        if (water > x) return false;
    }
    return water <= x;
}
void solve() {
    int n;
    long long x;
    cin >> n >> x;
    vector<long long> a(n);
    for (int i = 0; i < n; i++) cin >> a[i];
    long long low = 1, high = 2e9 + 7, ans = 1;
    while (low <= high) {
        long long mid = low + (high - low) / 2;
        if (check(a, mid, x)) {
            ans = mid;
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    cout << ans << "\\n";
}
int main() {
    int t;
    if (cin >> t) {
        while (t--) solve();
    }
    return 0;
}"""
    },
    {
        "id": "cf_1873e_building_aquarium_heldout_java",
        "name": "CF 1873E Building an Aquarium (Java)",
        "language": "java",
        "rating": "1700",
        "tags": ["binary_search_on_answer", "cf"],
        "expected_tc": "O(n log A)",
        "expected_sc": "O(1)",
        "expected_algorithms": ["Binary Search on Answer"],
        "code": """import java.util.Scanner;
public class Main {
    static boolean check(long[] a, long h, long x) {
        long water = 0;
        for (long ai : a) {
            if (h > ai) water += (h - ai);
            if (water > x) return false;
        }
        return water <= x;
    }
    static void solve(Scanner sc) {
        int n = sc.nextInt();
        long x = sc.nextLong();
        long[] a = new long[n];
        for (int i = 0; i < n; i++) a[i] = sc.nextLong();
        long low = 1, high = 2000000007L, ans = 1;
        while (low <= high) {
            long mid = low + (high - low) / 2;
            if (check(a, mid, x)) {
                ans = mid;
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        System.out.println(ans);
    }
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        if (sc.hasNextInt()) {
            int t = sc.nextInt();
            while (t-- > 0) solve(sc);
        }
    }
}"""
    },

    # -------------------------------------------------------------
    # Rating: 1800 (Codeforces)
    # -------------------------------------------------------------
    {
        "id": "cf_1676g_white_black_subtrees_heldout_cpp",
        "name": "CF 1676G White-Black Subtrees (C++)",
        "language": "cpp",
        "rating": "1800",
        "tags": ["tree_dp", "dfs", "tree", "cf"],
        "expected_tc": "O(n)",
        "expected_sc": "O(n)",
        "expected_algorithms": ["Tree DP", "DFS"],
        "code": """#include <iostream>
#include <vector>
#include <string>
using namespace std;
vector<vector<int>> tree;
string colors;
int balanced_count;
int dfs(int u) {
    int balance = (colors[u - 1] == 'W' ? 1 : -1);
    for (int child : tree[u]) {
        balance += dfs(child);
    }
    if (balance == 0) balanced_count++;
    return balance;
}
void solve() {
    int n;
    cin >> n;
    tree.assign(n + 1, vector<int>());
    for (int i = 2; i <= n; i++) {
        int p; cin >> p;
        tree[p].push_back(i);
    }
    cin >> colors;
    balanced_count = 0;
    dfs(1);
    cout << balanced_count << "\\n";
}
int main() {
    int t;
    if (cin >> t) {
        while (t--) solve();
    }
    return 0;
}"""
    },
    {
        "id": "cf_1676g_white_black_subtrees_heldout_java",
        "name": "CF 1676G White-Black Subtrees (Java)",
        "language": "java",
        "rating": "1800",
        "tags": ["tree_dp", "dfs", "tree", "cf"],
        "expected_tc": "O(n)",
        "expected_sc": "O(n)",
        "expected_algorithms": ["Tree DP", "DFS"],
        "code": """import java.util.*;
public class Main {
    static List<Integer>[] tree;
    static String colors;
    static int balancedCount;
    static int dfs(int u) {
        int balance = (colors.charAt(u - 1) == 'W' ? 1 : -1);
        for (int child : tree[u]) {
            balance += dfs(child);
        }
        if (balance == 0) balancedCount++;
        return balance;
    }
    static void solve(Scanner sc) {
        int n = sc.nextInt();
        tree = new ArrayList[n + 1];
        for (int i = 1; i <= n; i++) tree[i] = new ArrayList<>();
        for (int i = 2; i <= n; i++) {
            int p = sc.nextInt();
            tree[p].add(i);
        }
        colors = sc.next();
        balancedCount = 0;
        dfs(1);
        System.out.println(balancedCount);
    }
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        if (sc.hasNextInt()) {
            int t = sc.nextInt();
            while (t-- > 0) solve(sc);
        }
    }
}"""
    },

    # -------------------------------------------------------------
    # Rating: Easy (LeetCode)
    # -------------------------------------------------------------
    {
        "id": "lc_1_two_sum_cpp",
        "name": "LeetCode 1 Two Sum (C++)",
        "language": "cpp",
        "rating": "Easy",
        "tags": ["two_pointers", "sorting", "leetcode"],
        "expected_tc": "O(n log n)",
        "expected_sc": "O(n)",
        "expected_algorithms": ["Sorting", "Two Pointers"],
        "code": """#include <vector>
#include <algorithm>
using namespace std;
class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        int n = nums.size();
        vector<pair<int, int>> p(n);
        for (int i = 0; i < n; i++) p[i] = {nums[i], i};
        sort(p.begin(), p.end());
        int l = 0, r = n - 1;
        while (l < r) {
            int sum = p[l].first + p[r].first;
            if (sum == target) return {p[l].second, p[r].second};
            if (sum < target) l++;
            else r--;
        }
        return {};
    }
};"""
    },
    {
        "id": "lc_1_two_sum_java",
        "name": "LeetCode 1 Two Sum (Java)",
        "language": "java",
        "rating": "Easy",
        "tags": ["two_pointers", "sorting", "leetcode"],
        "expected_tc": "O(n log n)",
        "expected_sc": "O(n)",
        "expected_algorithms": ["Sorting", "Two Pointers"],
        "code": """import java.util.Arrays;
import java.util.Comparator;
class Solution {
    public int[] twoSum(int[] nums, int target) {
        int n = nums.length;
        int[][] p = new int[n][2];
        for (int i = 0; i < n; i++) {
            p[i][0] = nums[i];
            p[i][1] = i;
        }
        Arrays.sort(p, Comparator.comparingInt(a -> a[0]));
        int l = 0, r = n - 1;
        while (l < r) {
            int sum = p[l][0] + p[r][0];
            if (sum == target) return new int[]{p[l][1], p[r][1]};
            if (sum < target) l++;
            else r--;
        }
        return new int[]{};
    }
}"""
    },
    {
        "id": "lc_20_valid_parentheses_cpp",
        "name": "LeetCode 20 Valid Parentheses (C++)",
        "language": "cpp",
        "rating": "Easy",
        "tags": ["monotonic_stack", "string", "leetcode"],
        "expected_tc": "O(n)",
        "expected_sc": "O(n)",
        "expected_algorithms": ["Monotonic Stack"],
        "code": """#include <string>
#include <stack>
using namespace std;
class Solution {
public:
    bool isValid(string s) {
        stack<char> st;
        for (char c : s) {
            if (c == '(' || c == '{' || c == '[') {
                st.push(c);
            } else {
                if (st.empty()) return false;
                char top = st.top();
                if ((c == ')' && top == '(') || (c == '}' && top == '{') || (c == ']' && top == '[')) {
                    st.pop();
                } else {
                    return false;
                }
            }
        }
        return st.empty();
    }
};"""
    },
    {
        "id": "lc_20_valid_parentheses_java",
        "name": "LeetCode 20 Valid Parentheses (Java)",
        "language": "java",
        "rating": "Easy",
        "tags": ["monotonic_stack", "string", "leetcode"],
        "expected_tc": "O(n)",
        "expected_sc": "O(n)",
        "expected_algorithms": ["Monotonic Stack"],
        "code": """import java.util.Stack;
class Solution {
    public boolean isValid(String s) {
        Stack<Character> st = new Stack<>();
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            if (c == '(' || c == '{' || c == '[') {
                st.push(c);
            } else {
                if (st.empty()) return false;
                char top = st.pop();
                if (c == ')' && top != '(') return false;
                if (c == '}' && top != '{') return false;
                if (c == ']' && top != '[') return false;
            }
        }
        return st.empty();
    }
}"""
    },
    {
        "id": "lc_53_maximum_subarray_cpp",
        "name": "LeetCode 53 Maximum Subarray (C++)",
        "language": "cpp",
        "rating": "Easy",
        "tags": ["dp", "linear", "leetcode"],
        "expected_tc": "O(n)",
        "expected_sc": "O(1)",
        "expected_algorithms": [],
        "code": """#include <vector>
#include <algorithm>
using namespace std;
class Solution {
public:
    int maxSubArray(vector<int>& nums) {
        int max_so_far = nums[0];
        int curr_max = nums[0];
        for (size_t i = 1; i < nums.size(); i++) {
            curr_max = max(nums[i], curr_max + nums[i]);
            max_so_far = max(max_so_far, curr_max);
        }
        return max_so_far;
    }
};"""
    },
    {
        "id": "lc_53_maximum_subarray_java",
        "name": "LeetCode 53 Maximum Subarray (Java)",
        "language": "java",
        "rating": "Easy",
        "tags": ["dp", "linear", "leetcode"],
        "expected_tc": "O(n)",
        "expected_sc": "O(1)",
        "expected_algorithms": [],
        "code": """class Solution {
    public int maxSubArray(int[] nums) {
        int maxSoFar = nums[0];
        int currMax = nums[0];
        for (int i = 1; i < nums.length; i++) {
            currMax = Math.max(nums[i], currMax + nums[i]);
            maxSoFar = Math.max(maxSoFar, currMax);
        }
        return maxSoFar;
    }
}"""
    },

    # -------------------------------------------------------------
    # Rating: Medium (LeetCode)
    # -------------------------------------------------------------
    {
        "id": "lc_15_3sum_cpp",
        "name": "LeetCode 15 3Sum (C++)",
        "language": "cpp",
        "rating": "Medium",
        "tags": ["two_pointers", "sorting", "leetcode"],
        "expected_tc": "O(n^2)",
        "expected_sc": "O(log n)",
        "expected_algorithms": ["Sorting", "Two Pointers"],
        "code": """#include <vector>
#include <algorithm>
using namespace std;
class Solution {
public:
    vector<vector<int>> threeSum(vector<int>& nums) {
        vector<vector<int>> res;
        sort(nums.begin(), nums.end());
        int n = nums.size();
        for (int i = 0; i < n - 2; i++) {
            if (i > 0 && nums[i] == nums[i - 1]) continue;
            int l = i + 1, r = n - 1;
            while (l < r) {
                int sum = nums[i] + nums[l] + nums[r];
                if (sum == 0) {
                    res.push_back({nums[i], nums[l], nums[r]});
                    while (l < r && nums[l] == nums[l + 1]) l++;
                    while (l < r && nums[r] == nums[r - 1]) r--;
                    l++; r--;
                } else if (sum < 0) {
                    l++;
                } else {
                    r--;
                }
            }
        }
        return res;
    }
};"""
    },
    {
        "id": "lc_15_3sum_java",
        "name": "LeetCode 15 3Sum (Java)",
        "language": "java",
        "rating": "Medium",
        "tags": ["two_pointers", "sorting", "leetcode"],
        "expected_tc": "O(n^2)",
        "expected_sc": "O(log n)",
        "expected_algorithms": ["Sorting", "Two Pointers"],
        "code": """import java.util.*;
class Solution {
    public List<List<Integer>> threeSum(int[] nums) {
        List<List<Integer>> res = new ArrayList<>();
        Arrays.sort(nums);
        int n = nums.length;
        for (int i = 0; i < n - 2; i++) {
            if (i > 0 && nums[i] == nums[i - 1]) continue;
            int l = i + 1, r = n - 1;
            while (l < r) {
                int sum = nums[i] + nums[l] + nums[r];
                if (sum == 0) {
                    res.add(Arrays.asList(nums[i], nums[l], nums[r]));
                    while (l < r && nums[l] == nums[l + 1]) l++;
                    while (l < r && nums[r] == nums[r - 1]) r--;
                    l++; r--;
                } else if (sum < 0) {
                    l++;
                } else {
                    r--;
                }
            }
        }
        return res;
    }
}"""
    },
    {
        "id": "lc_3_longest_substring_cpp",
        "name": "LeetCode 3 Longest Substring Without Repeating Characters (C++)",
        "language": "cpp",
        "rating": "Medium",
        "tags": ["sliding_window", "string", "leetcode"],
        "expected_tc": "O(n)",
        "expected_sc": "O(A)",
        "expected_algorithms": ["Sliding Window"],
        "code": """#include <string>
#include <vector>
#include <algorithm>
using namespace std;
class Solution {
public:
    int lengthOfLongestSubstring(string s) {
        vector<int> last(256, -1);
        int left = 0, max_len = 0;
        for (int right = 0; right < (int)s.size(); right++) {
            if (last[(unsigned char)s[right]] >= left) {
                left = last[(unsigned char)s[right]] + 1;
            }
            last[(unsigned char)s[right]] = right;
            max_len = max(max_len, right - left + 1);
        }
        return max_len;
    }
};"""
    },
    {
        "id": "lc_3_longest_substring_java",
        "name": "LeetCode 3 Longest Substring Without Repeating Characters (Java)",
        "language": "java",
        "rating": "Medium",
        "tags": ["sliding_window", "string", "leetcode"],
        "expected_tc": "O(n)",
        "expected_sc": "O(A)",
        "expected_algorithms": ["Sliding Window"],
        "code": """import java.util.Arrays;
class Solution {
    public int lengthOfLongestSubstring(String s) {
        int[] last = new int[256];
        Arrays.fill(last, -1);
        int left = 0, maxLen = 0;
        for (int right = 0; right < s.length(); right++) {
            char c = s.charAt(right);
            if (last[c] >= left) {
                left = last[c] + 1;
            }
            last[c] = right;
            maxLen = Math.max(maxLen, right - left + 1);
        }
        return maxLen;
    }
}"""
    },
    {
        "id": "lc_739_daily_temperatures_cpp",
        "name": "LeetCode 739 Daily Temperatures (C++)",
        "language": "cpp",
        "rating": "Medium",
        "tags": ["monotonic_stack", "leetcode"],
        "expected_tc": "O(n)",
        "expected_sc": "O(n)",
        "expected_algorithms": ["Monotonic Stack"],
        "code": """#include <vector>
#include <stack>
using namespace std;
class Solution {
public:
    vector<int> dailyTemperatures(vector<int>& temperatures) {
        int n = temperatures.size();
        vector<int> ans(n, 0);
        stack<int> st;
        for (int i = 0; i < n; i++) {
            while (!st.empty() && temperatures[i] > temperatures[st.top()]) {
                int prev = st.top();
                st.pop();
                ans[prev] = i - prev;
            }
            st.push(i);
        }
        return ans;
    }
};"""
    },
    {
        "id": "lc_739_daily_temperatures_java",
        "name": "LeetCode 739 Daily Temperatures (Java)",
        "language": "java",
        "rating": "Medium",
        "tags": ["monotonic_stack", "leetcode"],
        "expected_tc": "O(n)",
        "expected_sc": "O(n)",
        "expected_algorithms": ["Monotonic Stack"],
        "code": """import java.util.Stack;
class Solution {
    public int[] dailyTemperatures(int[] temperatures) {
        int n = temperatures.length;
        int[] ans = new int[n];
        Stack<Integer> st = new Stack<>();
        for (int i = 0; i < n; i++) {
            while (!st.empty() && temperatures[i] > temperatures[st.peek()]) {
                int prev = st.pop();
                ans[prev] = i - prev;
            }
            st.push(i);
        }
        return ans;
    }
}"""
    },
    {
        "id": "lc_200_number_of_islands_cpp",
        "name": "LeetCode 200 Number of Islands (C++)",
        "language": "cpp",
        "rating": "Medium",
        "tags": ["bfs", "dfs", "graph", "leetcode"],
        "expected_tc": "O(n * m)",
        "expected_sc": "O(n * m)",
        "expected_algorithms": ["BFS"],
        "code": """#include <vector>
#include <queue>
using namespace std;
class Solution {
public:
    int numIslands(vector<vector<char>>& grid) {
        int m = grid.size();
        if (m == 0) return 0;
        int n = grid[0].size();
        int count = 0;
        queue<pair<int, int>> q;
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (grid[i][j] == '1') {
                    count++;
                    grid[i][j] = '0';
                    q.push({i, j});
                    while (!q.empty()) {
                        auto [r, c] = q.front();
                        q.pop();
                        int dr[] = {-1, 1, 0, 0};
                        int dc[] = {0, 0, -1, 1};
                        for (int d = 0; d < 4; d++) {
                            int nr = r + dr[d], nc = c + dc[d];
                            if (nr >= 0 && nr < m && nc >= 0 && nc < n && grid[nr][nc] == '1') {
                                grid[nr][nc] = '0';
                                q.push({nr, nc});
                            }
                        }
                    }
                }
            }
        }
        return count;
    }
};"""
    },
    {
        "id": "lc_200_number_of_islands_java",
        "name": "LeetCode 200 Number of Islands (Java)",
        "language": "java",
        "rating": "Medium",
        "tags": ["bfs", "dfs", "graph", "leetcode"],
        "expected_tc": "O(n * m)",
        "expected_sc": "O(n * m)",
        "expected_algorithms": ["BFS"],
        "code": """import java.util.Queue;
import java.util.LinkedList;
class Solution {
    public int numIslands(char[][] grid) {
        int m = grid.length;
        if (m == 0) return 0;
        int n = grid[0].length;
        int count = 0;
        Queue<int[]> q = new LinkedList<>();
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (grid[i][j] == '1') {
                    count++;
                    grid[i][j] = '0';
                    q.offer(new int[]{i, j});
                    while (!q.isEmpty()) {
                        int[] cur = q.poll();
                        int r = cur[0], c = cur[1];
                        int[] dr = {-1, 1, 0, 0};
                        int[] dc = {0, 0, -1, 1};
                        for (int d = 0; d < 4; d++) {
                            int nr = r + dr[d], nc = c + dc[d];
                            if (nr >= 0 && nr < m && nc >= 0 && nc < n && grid[nr][nc] == '1') {
                                grid[nr][nc] = '0';
                                q.offer(new int[]{nr, nc});
                            }
                        }
                    }
                }
            }
        }
        return count;
    }
}"""
    },

    # -------------------------------------------------------------
    # Rating: Hard (LeetCode)
    # -------------------------------------------------------------
    {
        "id": "lc_42_trapping_rain_water_cpp",
        "name": "LeetCode 42 Trapping Rain Water (C++)",
        "language": "cpp",
        "rating": "Hard",
        "tags": ["two_pointers", "monotonic_stack", "leetcode"],
        "expected_tc": "O(n)",
        "expected_sc": "O(1)",
        "expected_algorithms": ["Two Pointers"],
        "code": """#include <vector>
#include <algorithm>
using namespace std;
class Solution {
public:
    int trap(vector<int>& height) {
        int left = 0, right = height.size() - 1;
        int left_max = 0, right_max = 0;
        int water = 0;
        while (left < right) {
            if (height[left] < height[right]) {
                if (height[left] >= left_max) left_max = height[left];
                else water += left_max - height[left];
                left++;
            } else {
                if (height[right] >= right_max) right_max = height[right];
                else water += right_max - height[right];
                right--;
            }
        }
        return water;
    }
};"""
    },
    {
        "id": "lc_42_trapping_rain_water_java",
        "name": "LeetCode 42 Trapping Rain Water (Java)",
        "language": "java",
        "rating": "Hard",
        "tags": ["two_pointers", "monotonic_stack", "leetcode"],
        "expected_tc": "O(n)",
        "expected_sc": "O(1)",
        "expected_algorithms": ["Two Pointers"],
        "code": """class Solution {
    public int trap(int[] height) {
        int left = 0, right = height.length - 1;
        int leftMax = 0, rightMax = 0;
        int water = 0;
        while (left < right) {
            if (height[left] < height[right]) {
                if (height[left] >= leftMax) leftMax = height[left];
                else water += leftMax - height[left];
                left++;
            } else {
                if (height[right] >= rightMax) rightMax = height[right];
                else water += rightMax - height[right];
                right--;
            }
        }
        return water;
    }
}"""
    },
    {
        "id": "lc_239_sliding_window_max_cpp",
        "name": "LeetCode 239 Sliding Window Maximum (C++)",
        "language": "cpp",
        "rating": "Hard",
        "tags": ["monotonic_queue", "sliding_window", "leetcode"],
        "expected_tc": "O(n)",
        "expected_sc": "O(k)",
        "expected_algorithms": ["Monotonic Queue", "Sliding Window"],
        "code": """#include <vector>
#include <deque>
using namespace std;
class Solution {
public:
    vector<int> maxSlidingWindow(vector<int>& nums, int k) {
        deque<int> dq;
        vector<int> ans;
        for (int i = 0; i < (int)nums.size(); i++) {
            if (!dq.empty() && dq.front() == i - k) dq.pop_front();
            while (!dq.empty() && nums[dq.back()] <= nums[i]) dq.pop_back();
            dq.push_back(i);
            if (i >= k - 1) ans.push_back(nums[dq.front()]);
        }
        return ans;
    }
};"""
    },
    {
        "id": "lc_239_sliding_window_max_java",
        "name": "LeetCode 239 Sliding Window Maximum (Java)",
        "language": "java",
        "rating": "Hard",
        "tags": ["monotonic_queue", "sliding_window", "leetcode"],
        "expected_tc": "O(n)",
        "expected_sc": "O(k)",
        "expected_algorithms": ["Monotonic Queue", "Sliding Window"],
        "code": """import java.util.ArrayDeque;
import java.util.Deque;
class Solution {
    public int[] maxSlidingWindow(int[] nums, int k) {
        Deque<Integer> dq = new ArrayDeque<>();
        int n = nums.length;
        int[] ans = new int[n - k + 1];
        int idx = 0;
        for (int i = 0; i < n; i++) {
            if (!dq.isEmpty() && dq.peekFirst() == i - k) dq.pollFirst();
            while (!dq.isEmpty() && nums[dq.peekLast()] <= nums[i]) dq.pollLast();
            dq.offerLast(i);
            if (i >= k - 1) ans[idx++] = nums[dq.peekFirst()];
        }
        return ans;
    }
}"""
    },

    # -------------------------------------------------------------
    # Rating: CSES Problems
    # -------------------------------------------------------------
    {
        "id": "cses_distinct_numbers_cpp",
        "name": "CSES Distinct Numbers (C++)",
        "language": "cpp",
        "rating": "1000",
        "tags": ["sorting", "cses"],
        "expected_tc": "O(n log n)",
        "expected_sc": "O(n)",
        "expected_algorithms": ["Sorting"],
        "code": """#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;
int main() {
    int n;
    if (!(cin >> n)) return 0;
    vector<int> a(n);
    for (int i = 0; i < n; i++) cin >> a[i];
    sort(a.begin(), a.end());
    int count = 1;
    for (int i = 1; i < n; i++) {
        if (a[i] != a[i - 1]) count++;
    }
    cout << count << "\\n";
    return 0;
}"""
    },
    {
        "id": "cses_distinct_numbers_java",
        "name": "CSES Distinct Numbers (Java)",
        "language": "java",
        "rating": "1000",
        "tags": ["sorting", "cses"],
        "expected_tc": "O(n log n)",
        "expected_sc": "O(n)",
        "expected_algorithms": ["Sorting"],
        "code": """import java.util.Scanner;
import java.util.Arrays;
public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        if (!sc.hasNextInt()) return;
        int n = sc.nextInt();
        int[] a = new int[n];
        for (int i = 0; i < n; i++) a[i] = sc.nextInt();
        Arrays.sort(a);
        int count = 1;
        for (int i = 1; i < n; i++) {
            if (a[i] != a[i - 1]) count++;
        }
        System.out.println(count);
    }
}"""
    },
    {
        "id": "cses_sum_of_two_values_cpp",
        "name": "CSES Sum of Two Values (C++)",
        "language": "cpp",
        "rating": "1100",
        "tags": ["two_pointers", "sorting", "cses"],
        "expected_tc": "O(n log n)",
        "expected_sc": "O(n)",
        "expected_algorithms": ["Sorting", "Two Pointers"],
        "code": """#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;
int main() {
    int n;
    long long x;
    if (!(cin >> n >> x)) return 0;
    vector<pair<long long, int>> a(n);
    for (int i = 0; i < n; i++) {
        cin >> a[i].first;
        a[i].second = i + 1;
    }
    sort(a.begin(), a.end());
    int l = 0, r = n - 1;
    while (l < r) {
        long long s = a[l].first + a[r].first;
        if (s == x) {
            cout << a[l].second << " " << a[r].second << "\\n";
            return 0;
        }
        if (s < x) l++;
        else r--;
    }
    cout << "IMPOSSIBLE\\n";
    return 0;
}"""
    },
    {
        "id": "cses_sum_of_two_values_java",
        "name": "CSES Sum of Two Values (Java)",
        "language": "java",
        "rating": "1100",
        "tags": ["two_pointers", "sorting", "cses"],
        "expected_tc": "O(n log n)",
        "expected_sc": "O(n)",
        "expected_algorithms": ["Sorting", "Two Pointers"],
        "code": """import java.util.Scanner;
import java.util.Arrays;
import java.util.Comparator;
public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        if (!sc.hasNextInt()) return;
        int n = sc.nextInt();
        long x = sc.nextLong();
        long[][] a = new long[n][2];
        for (int i = 0; i < n; i++) {
            a[i][0] = sc.nextLong();
            a[i][1] = i + 1;
        }
        Arrays.sort(a, Comparator.comparingLong(p -> p[0]));
        int l = 0, r = n - 1;
        while (l < r) {
            long s = a[l][0] + a[r][0];
            if (s == x) {
                System.out.println(a[l][1] + " " + a[r][1]);
                return;
            }
            if (s < x) l++;
            else r--;
        }
        System.out.println("IMPOSSIBLE");
    }
}"""
    },
    {
        "id": "cses_subarray_sums_i_cpp",
        "name": "CSES Subarray Sums I (C++)",
        "language": "cpp",
        "rating": "1200",
        "tags": ["sliding_window", "two_pointers", "cses"],
        "expected_tc": "O(n)",
        "expected_sc": "O(n)",
        "expected_algorithms": ["Two Pointers"],
        "code": """#include <iostream>
#include <vector>
using namespace std;
int main() {
    int n;
    long long x;
    if (!(cin >> n >> x)) return 0;
    vector<long long> a(n);
    for (int i = 0; i < n; i++) cin >> a[i];
    int l = 0, count = 0;
    long long cur = 0;
    for (int r = 0; r < n; r++) {
        cur += a[r];
        while (cur > x && l <= r) {
            cur -= a[l];
            l++;
        }
        if (cur == x) count++;
    }
    cout << count << "\\n";
    return 0;
}"""
    },
    {
        "id": "cses_subarray_sums_i_java",
        "name": "CSES Subarray Sums I (Java)",
        "language": "java",
        "rating": "1200",
        "tags": ["sliding_window", "two_pointers", "cses"],
        "expected_tc": "O(n)",
        "expected_sc": "O(n)",
        "expected_algorithms": ["Two Pointers"],
        "code": """import java.util.Scanner;
public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        if (!sc.hasNextInt()) return;
        int n = sc.nextInt();
        long x = sc.nextLong();
        long[] a = new long[n];
        for (int i = 0; i < n; i++) a[i] = sc.nextLong();
        int l = 0, count = 0;
        long cur = 0;
        for (int r = 0; r < n; r++) {
            cur += a[r];
            while (cur > x && l <= r) {
                cur -= a[l];
                l++;
            }
            if (cur == x) count++;
        }
        System.out.println(count);
    }
}"""
    },

    # -------------------------------------------------------------
    # Rating: AtCoder DP Contest
    # -------------------------------------------------------------
    {
        "id": "atcoder_dp_a_frog1_cpp",
        "name": "AtCoder Educational DP A Frog 1 (C++)",
        "language": "cpp",
        "rating": "1100",
        "tags": ["dp", "atcoder"],
        "expected_tc": "O(n)",
        "expected_sc": "O(n)",
        "expected_algorithms": [],
        "code": """#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>
using namespace std;
int main() {
    int n;
    if (!(cin >> n)) return 0;
    vector<int> h(n);
    for (int i = 0; i < n; i++) cin >> h[i];
    vector<int> dp(n, 1e9);
    dp[0] = 0;
    for (int i = 0; i < n; i++) {
        if (i + 1 < n) dp[i + 1] = min(dp[i + 1], dp[i] + abs(h[i] - h[i + 1]));
        if (i + 2 < n) dp[i + 2] = min(dp[i + 2], dp[i] + abs(h[i] - h[i + 2]));
    }
    cout << dp[n - 1] << "\\n";
    return 0;
}"""
    },
    {
        "id": "atcoder_dp_a_frog1_java",
        "name": "AtCoder Educational DP A Frog 1 (Java)",
        "language": "java",
        "rating": "1100",
        "tags": ["dp", "atcoder"],
        "expected_tc": "O(n)",
        "expected_sc": "O(n)",
        "expected_algorithms": [],
        "code": """import java.util.Scanner;
import java.util.Arrays;
public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        if (!sc.hasNextInt()) return;
        int n = sc.nextInt();
        int[] h = new int[n];
        for (int i = 0; i < n; i++) h[i] = sc.nextInt();
        int[] dp = new int[n];
        Arrays.fill(dp, 1000000000);
        dp[0] = 0;
        for (int i = 0; i < n; i++) {
            if (i + 1 < n) dp[i + 1] = Math.min(dp[i + 1], dp[i] + Math.abs(h[i] - h[i + 1]));
            if (i + 2 < n) dp[i + 2] = Math.min(dp[i + 2], dp[i] + Math.abs(h[i] - h[i + 2]));
        }
        System.out.println(dp[n - 1]);
    }
}"""
    },
    {
        "id": "atcoder_dp_b_frog2_cpp",
        "name": "AtCoder Educational DP B Frog 2 (C++)",
        "language": "cpp",
        "rating": "1200",
        "tags": ["dp", "nested", "atcoder"],
        "expected_tc": "O(n * k)",
        "expected_sc": "O(n)",
        "expected_algorithms": [],
        "code": """#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>
using namespace std;
int main() {
    int n, k;
    if (!(cin >> n >> k)) return 0;
    vector<int> h(n);
    for (int i = 0; i < n; i++) cin >> h[i];
    vector<int> dp(n, 1e9);
    dp[0] = 0;
    for (int i = 0; i < n; i++) {
        for (int j = 1; j <= k && i + j < n; j++) {
            dp[i + j] = min(dp[i + j], dp[i] + abs(h[i] - h[i + j]));
        }
    }
    cout << dp[n - 1] << "\\n";
    return 0;
}"""
    },
    {
        "id": "atcoder_dp_b_frog2_java",
        "name": "AtCoder Educational DP B Frog 2 (Java)",
        "language": "java",
        "rating": "1200",
        "tags": ["dp", "nested", "atcoder"],
        "expected_tc": "O(n * k)",
        "expected_sc": "O(n)",
        "expected_algorithms": [],
        "code": """import java.util.Scanner;
import java.util.Arrays;
public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        if (!sc.hasNextInt()) return;
        int n = sc.nextInt();
        int k = sc.nextInt();
        int[] h = new int[n];
        for (int i = 0; i < n; i++) h[i] = sc.nextInt();
        int[] dp = new int[n];
        Arrays.fill(dp, 1000000000);
        dp[0] = 0;
        for (int i = 0; i < n; i++) {
            for (int j = 1; j <= k && i + j < n; j++) {
                dp[i + j] = Math.min(dp[i + j], dp[i] + Math.abs(h[i] - h[i + j]));
            }
        }
        System.out.println(dp[n - 1]);
    }
}"""
    },
    {
        "id": "atcoder_abc085_c_otoshidama_cpp",
        "name": "AtCoder ABC 085 C Otoshidama (C++)",
        "language": "cpp",
        "rating": "1000",
        "tags": ["nested", "math", "atcoder"],
        "expected_tc": "O(n^2)",
        "expected_sc": "O(1)",
        "expected_algorithms": [],
        "code": """#include <iostream>
using namespace std;
int main() {
    int n;
    long long y;
    if (!(cin >> n >> y)) return 0;
    for (int i = 0; i <= n; i++) {
        for (int j = 0; i + j <= n; j++) {
            int k = n - i - j;
            if (10000LL * i + 5000LL * j + 1000LL * k == y) {
                cout << i << " " << j << " " << k << "\\n";
                return 0;
            }
        }
    }
    cout << "-1 -1 -1\\n";
    return 0;
}"""
    },
    {
        "id": "atcoder_abc085_c_otoshidama_java",
        "name": "AtCoder ABC 085 C Otoshidama (Java)",
        "language": "java",
        "rating": "1000",
        "tags": ["nested", "math", "atcoder"],
        "expected_tc": "O(n^2)",
        "expected_sc": "O(1)",
        "expected_algorithms": [],
        "code": """import java.util.Scanner;
public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        if (!sc.hasNextInt()) return;
        int n = sc.nextInt();
        long y = sc.nextLong();
        for (int i = 0; i <= n; i++) {
            for (int j = 0; i + j <= n; j++) {
                int k = n - i - j;
                if (10000L * i + 5000L * j + 1000L * k == y) {
                    System.out.println(i + " " + j + " " + k);
                    return;
                }
            }
        }
        System.out.println("-1 -1 -1");
    }
}"""
    },

    # -------------------------------------------------------------
    # Additional Real CP Problems for Richness & Scale
    # -------------------------------------------------------------
    {
        "id": "cf_1352c_kth_not_divisible_heldout_cpp",
        "name": "CF 1352C K-th Not Divisible by n (C++)",
        "language": "cpp",
        "rating": "1200",
        "tags": ["math", "cf"],
        "expected_tc": "O(1)",
        "expected_sc": "O(1)",
        "expected_algorithms": [],
        "code": """#include <iostream>
using namespace std;
void solve() {
    long long n, k;
    cin >> n >> k;
    cout << k + (k - 1) / (n - 1) << "\\n";
}
int main() {
    int t;
    if (cin >> t) {
        while (t--) solve();
    }
    return 0;
}"""
    },
    {
        "id": "cf_1352c_kth_not_divisible_heldout_java",
        "name": "CF 1352C K-th Not Divisible by n (Java)",
        "language": "java",
        "rating": "1200",
        "tags": ["math", "cf"],
        "expected_tc": "O(1)",
        "expected_sc": "O(1)",
        "expected_algorithms": [],
        "code": """import java.util.Scanner;
public class Main {
    static void solve(Scanner sc) {
        long n = sc.nextLong();
        long k = sc.nextLong();
        System.out.println(k + (k - 1) / (n - 1));
    }
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        if (sc.hasNextInt()) {
            int t = sc.nextInt();
            while (t-- > 0) solve(sc);
        }
    }
}"""
    },
    {
        "id": "cf_1840c_ski_resort_heldout_cpp",
        "name": "CF 1840C Ski Resort (C++)",
        "language": "cpp",
        "rating": "1000",
        "tags": ["two_pointers", "math", "cf"],
        "expected_tc": "O(n)",
        "expected_sc": "O(1)",
        "expected_algorithms": ["Two Pointers"],
        "code": """#include <iostream>
#include <vector>
using namespace std;
void solve() {
    int n, k;
    long long q;
    cin >> n >> k >> q;
    long long ans = 0, len = 0;
    for (int i = 0; i < n; i++) {
        long long a;
        cin >> a;
        if (a <= q) {
            len++;
        } else {
            if (len >= k) {
                long long m = len - k + 1;
                ans += m * (m + 1) / 2;
            }
            len = 0;
        }
    }
    if (len >= k) {
        long long m = len - k + 1;
        ans += m * (m + 1) / 2;
    }
    cout << ans << "\\n";
}
int main() {
    int t;
    if (cin >> t) {
        while (t--) solve();
    }
    return 0;
}"""
    },
    {
        "id": "cf_1840c_ski_resort_heldout_java",
        "name": "CF 1840C Ski Resort (Java)",
        "language": "java",
        "rating": "1000",
        "tags": ["two_pointers", "math", "cf"],
        "expected_tc": "O(n)",
        "expected_sc": "O(1)",
        "expected_algorithms": ["Two Pointers"],
        "code": """import java.util.Scanner;
public class Main {
    static void solve(Scanner sc) {
        int n = sc.nextInt();
        int k = sc.nextInt();
        long q = sc.nextLong();
        long ans = 0, len = 0;
        for (int i = 0; i < n; i++) {
            long a = sc.nextLong();
            if (a <= q) {
                len++;
            } else {
                if (len >= k) {
                    long m = len - k + 1;
                    ans += m * (m + 1) / 2;
                }
                len = 0;
            }
        }
        if (len >= k) {
            long m = len - k + 1;
            ans += m * (m + 1) / 2;
        }
        System.out.println(ans);
    }
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        if (sc.hasNextInt()) {
            int t = sc.nextInt();
            while (t-- > 0) solve(sc);
        }
    }
}"""
    },
    {
        "id": "cf_1850d_balanced_round_heldout_cpp",
        "name": "CF 1850D Balanced Round (C++)",
        "language": "cpp",
        "rating": "900",
        "tags": ["sorting", "two_pointers", "cf"],
        "expected_tc": "O(n log n)",
        "expected_sc": "O(log n)",
        "expected_algorithms": ["Sorting", "Two Pointers"],
        "code": """#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;
void solve() {
    int n;
    long long k;
    cin >> n >> k;
    vector<long long> a(n);
    for (int i = 0; i < n; i++) cin >> a[i];
    sort(a.begin(), a.end());
    int max_chain = 1, cur_chain = 1;
    for (int i = 1; i < n; i++) {
        if (a[i] - a[i - 1] <= k) {
            cur_chain++;
        } else {
            max_chain = max(max_chain, cur_chain);
            cur_chain = 1;
        }
    }
    max_chain = max(max_chain, cur_chain);
    cout << (n - max_chain) << "\\n";
}
int main() {
    int t;
    if (cin >> t) {
        while (t--) solve();
    }
    return 0;
}"""
    },
    {
        "id": "cf_1850d_balanced_round_heldout_java",
        "name": "CF 1850D Balanced Round (Java)",
        "language": "java",
        "rating": "900",
        "tags": ["sorting", "two_pointers", "cf"],
        "expected_tc": "O(n log n)",
        "expected_sc": "O(log n)",
        "expected_algorithms": ["Sorting", "Two Pointers"],
        "code": """import java.util.Scanner;
import java.util.Arrays;
public class Main {
    static void solve(Scanner sc) {
        int n = sc.nextInt();
        long k = sc.nextLong();
        long[] a = new long[n];
        for (int i = 0; i < n; i++) a[i] = sc.nextLong();
        Arrays.sort(a);
        int maxChain = 1, curChain = 1;
        for (int i = 1; i < n; i++) {
            if (a[i] - a[i - 1] <= k) {
                curChain++;
            } else {
                maxChain = Math.max(maxChain, curChain);
                curChain = 1;
            }
        }
        maxChain = Math.max(maxChain, curChain);
        System.out.println(n - maxChain);
    }
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        if (sc.hasNextInt()) {
            int t = sc.nextInt();
            while (t-- > 0) solve(sc);
        }
    }
}"""
    },
    {
        "id": "cf_1872d_plus_minus_permutation_heldout_cpp",
        "name": "CF 1872D Plus Minus Permutation (C++)",
        "language": "cpp",
        "rating": "1100",
        "tags": ["math", "cf"],
        "expected_tc": "O(log n)",
        "expected_sc": "O(1)",
        "expected_algorithms": [],
        "code": """#include <iostream>
#include <numeric>
using namespace std;
long long gcd(long long a, long long b) {
    while (b) { a %= b; swap(a, b); }
    return a;
}
long long lcm(long long a, long long b) {
    return (a / gcd(a, b)) * b;
}
void solve() {
    long long n, x, y;
    cin >> n >> x >> y;
    long long common = n / lcm(x, y);
    long long plus_count = n / x - common;
    long long minus_count = n / y - common;
    long long sum_plus = (n + (n - plus_count + 1)) * plus_count / 2;
    long long sum_minus = (1 + minus_count) * minus_count / 2;
    cout << sum_plus - sum_minus << "\\n";
}
int main() {
    int t;
    if (cin >> t) {
        while (t--) solve();
    }
    return 0;
}"""
    },
    {
        "id": "cf_1872d_plus_minus_permutation_heldout_java",
        "name": "CF 1872D Plus Minus Permutation (Java)",
        "language": "java",
        "rating": "1100",
        "tags": ["math", "cf"],
        "expected_tc": "O(log n)",
        "expected_sc": "O(1)",
        "expected_algorithms": [],
        "code": """import java.util.Scanner;
public class Main {
    static long gcd(long a, long b) {
        while (b != 0) { long t = a % b; a = b; b = t; }
        return a;
    }
    static long lcm(long a, long b) {
        return (a / gcd(a, b)) * b;
    }
    static void solve(Scanner sc) {
        long n = sc.nextLong();
        long x = sc.nextLong();
        long y = sc.nextLong();
        long common = n / lcm(x, y);
        long plusCount = n / x - common;
        long minusCount = n / y - common;
        long sumPlus = (n + (n - plusCount + 1)) * plusCount / 2;
        long sumMinus = (1 + minusCount) * minusCount / 2;
        System.out.println(sumPlus - sumMinus);
    }
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        if (sc.hasNextInt()) {
            int t = sc.nextInt();
            while (t-- > 0) solve(sc);
        }
    }
}"""
    },
    {
        "id": "cf_1985f_final_boss_heldout_cpp",
        "name": "CF 1985F Final Boss (C++)",
        "language": "cpp",
        "rating": "1500",
        "tags": ["binary_search_on_answer", "cf"],
        "expected_tc": "O(n log A)",
        "expected_sc": "O(1)",
        "expected_algorithms": ["Binary Search on Answer"],
        "code": """#include <iostream>
#include <vector>
using namespace std;
bool can_defeat(long long turns, long long h, int n, const vector<long long>& a, const vector<long long>& c) {
    long long total = 0;
    for (int i = 0; i < n; i++) {
        long long attacks = (turns - 1) / c[i] + 1;
        total += attacks * a[i];
        if (total >= h) return true;
    }
    return total >= h;
}
void solve() {
    long long h;
    int n;
    cin >> h >> n;
    vector<long long> a(n), c(n);
    for (int i = 0; i < n; i++) cin >> a[i];
    for (int i = 0; i < n; i++) cin >> c[i];
    long long low = 1, high = 4e11, ans = high;
    while (low <= high) {
        long long mid = low + (high - low) / 2;
        if (can_defeat(mid, h, n, a, c)) {
            ans = mid;
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    cout << ans << "\\n";
}
int main() {
    int t;
    if (cin >> t) {
        while (t--) solve();
    }
    return 0;
}"""
    },
    {
        "id": "cf_1985f_final_boss_heldout_java",
        "name": "CF 1985F Final Boss (Java)",
        "language": "java",
        "rating": "1500",
        "tags": ["binary_search_on_answer", "cf"],
        "expected_tc": "O(n log A)",
        "expected_sc": "O(1)",
        "expected_algorithms": ["Binary Search on Answer"],
        "code": """import java.util.Scanner;
public class Main {
    static boolean canDefeat(long turns, long h, int n, long[] a, long[] c) {
        long total = 0;
        for (int i = 0; i < n; i++) {
            long attacks = (turns - 1) / c[i] + 1;
            total += attacks * a[i];
            if (total >= h) return true;
        }
        return total >= h;
    }
    static void solve(Scanner sc) {
        long h = sc.nextLong();
        int n = sc.nextInt();
        long[] a = new long[n];
        long[] c = new long[n];
        for (int i = 0; i < n; i++) a[i] = sc.nextLong();
        for (int i = 0; i < n; i++) c[i] = sc.nextLong();
        long low = 1, high = 400000000000L, ans = high;
        while (low <= high) {
            long mid = low + (high - low) / 2;
            if (canDefeat(mid, h, n, a, c)) {
                ans = mid;
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        System.out.println(ans);
    }
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        if (sc.hasNextInt()) {
            int t = sc.nextInt();
            while (t-- > 0) solve(sc);
        }
    }
}"""
    },
    {
        "id": "cf_1843d_apple_tree_heldout_cpp",
        "name": "CF 1843D Apple Tree (C++)",
        "language": "cpp",
        "rating": "1200",
        "tags": ["dfs", "tree", "cf"],
        "expected_tc": "O(n)",
        "expected_sc": "O(n)",
        "expected_algorithms": ["DFS"],
        "code": """#include <iostream>
#include <vector>
using namespace std;
vector<vector<int>> adj;
vector<long long> leaves;
void dfs(int u, int p) {
    bool is_leaf = true;
    for (int v : adj[u]) {
        if (v != p) {
            is_leaf = false;
            dfs(v, u);
            leaves[u] += leaves[v];
        }
    }
    if (is_leaf) leaves[u] = 1;
}
void solve() {
    int n;
    cin >> n;
    adj.assign(n + 1, vector<int>());
    leaves.assign(n + 1, 0);
    for (int i = 0; i < n - 1; i++) {
        int u, v; cin >> u >> v;
        adj[u].push_back(v);
        adj[v].push_back(u);
    }
    dfs(1, 0);
    int q;
    cin >> q;
    while (q--) {
        int x, y;
        cin >> x >> y;
        cout << leaves[x] * leaves[y] << "\\n";
    }
}
int main() {
    int t;
    if (cin >> t) {
        while (t--) solve();
    }
    return 0;
}"""
    },
    {
        "id": "cf_1843d_apple_tree_heldout_java",
        "name": "CF 1843D Apple Tree (Java)",
        "language": "java",
        "rating": "1200",
        "tags": ["dfs", "tree", "cf"],
        "expected_tc": "O(n)",
        "expected_sc": "O(n)",
        "expected_algorithms": ["DFS"],
        "code": """import java.util.*;
public class Main {
    static List<Integer>[] adj;
    static long[] leaves;
    static void dfs(int u, int p) {
        boolean isLeaf = true;
        for (int v : adj[u]) {
            if (v != p) {
                isLeaf = false;
                dfs(v, u);
                leaves[u] += leaves[v];
            }
        }
        if (isLeaf) leaves[u] = 1;
    }
    static void solve(Scanner sc) {
        int n = sc.nextInt();
        adj = new ArrayList[n + 1];
        leaves = new long[n + 1];
        for (int i = 1; i <= n; i++) adj[i] = new ArrayList<>();
        for (int i = 0; i < n - 1; i++) {
            int u = sc.nextInt(), v = sc.nextInt();
            adj[u].add(v); adj[v].add(u);
        }
        dfs(1, 0);
        int q = sc.nextInt();
        while (q-- > 0) {
            int x = sc.nextInt(), y = sc.nextInt();
            System.out.println(leaves[x] * leaves[y]);
        }
    }
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        if (sc.hasNextInt()) {
            int t = sc.nextInt();
            while (t-- > 0) solve(sc);
        }
    }
}"""
    }
]

out_path = os.path.join(os.path.dirname(__file__), "held_out.json")
with open(out_path, "w", encoding="utf-8") as f:
    json.dump(HELD_OUT_CASES, f, indent=2)

print(f"Generated {len(HELD_OUT_CASES)} held-out cases into {out_path}")

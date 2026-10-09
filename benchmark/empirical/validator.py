import math
import os
import sys
import time

def run_constant(n):
    return (n % 2 == 0) and (n > 2)

def run_logarithmic(n):
    cnt = 0
    i = n
    while i > 0:
        cnt += 1
        i //= 2
    return cnt

def run_linear(n):
    s = 0
    for i in range(n):
        s += i
    return s

def run_linearithmic(n):
    a = list(range(n, 0, -1))
    a.sort()
    return a[0]

def run_quadratic(n):
    s = 0
    for i in range(n):
        for j in range(n):
            s += i ^ j
    return s

def run_cubic(n):
    s = 0
    for i in range(n):
        for j in range(n):
            for k in range(n):
                s += (i + j + k) & 1
    return s

BENCHMARKS = [
    {
        "name": "Constant O(1)",
        "predicted": "O(1)",
        "expected_exponent": 0.0,
        "func": run_constant,
        "sizes": [100000, 200000, 400000, 800000],
        "tolerance": 0.25,
    },
    {
        "name": "Logarithmic O(log n)",
        "predicted": "O(log n)",
        "expected_exponent": 0.0,
        "func": run_logarithmic,
        "sizes": [1000000, 2000000, 4000000, 8000000],
        "tolerance": 0.25,
    },
    {
        "name": "Linear O(n)",
        "predicted": "O(n)",
        "expected_exponent": 1.0,
        "func": run_linear,
        "sizes": [200000, 400000, 800000, 1600000],
        "tolerance": 0.35,
    },
    {
        "name": "Linearithmic O(n log n)",
        "predicted": "O(n log n)",
        "expected_exponent": 1.1,
        "func": run_linearithmic,
        "sizes": [50000, 100000, 200000, 400000],
        "tolerance": 0.35,
    },
    {
        "name": "Quadratic O(n^2)",
        "predicted": "O(n^2)",
        "expected_exponent": 2.0,
        "func": run_quadratic,
        "sizes": [300, 600, 1200, 2400],
        "tolerance": 0.40,
    },
    {
        "name": "Cubic O(n^3)",
        "predicted": "O(n^3)",
        "expected_exponent": 3.0,
        "func": run_cubic,
        "sizes": [30, 60, 120, 240],
        "tolerance": 0.45,
    },
]

def measure_time(func, n, repeats=2):
    func(min(n, 50))
    best = float("inf")
    for _ in range(repeats):
        start = time.perf_counter()
        func(n)
        duration = time.perf_counter() - start
        if duration < best:
            best = duration
    return best

def compute_scaling_exponent(sizes, times):
    log_n = [math.log(s) for s in sizes]
    log_t = [math.log(max(t, 1e-9)) for t in times]
    mean_n = sum(log_n) / len(log_n)
    mean_t = sum(log_t) / len(log_t)

    numerator = sum((log_n[i] - mean_n) * (log_t[i] - mean_t) for i in range(len(sizes)))
    denominator = sum((log_n[i] - mean_n) ** 2 for i in range(len(sizes)))
    if denominator == 0:
        return 0.0
    return numerator / denominator

def main():
    print("=" * 80)
    print("                  KITEO EMPIRICAL COMPLEXITY VALIDATOR                 ")
    print("=" * 80)

    results = []
    all_passed = True

    for bench in BENCHMARKS:
        times = []
        for s in bench["sizes"]:
            t = measure_time(bench["func"], s)
            times.append(t)

        alpha = compute_scaling_exponent(bench["sizes"], times)
        expected = bench["expected_exponent"]
        tol = bench["tolerance"]

        if expected == 0.0:
            passed = alpha <= (expected + tol + 0.15)
        else:
            passed = abs(alpha - expected) <= tol

        status = "PASS" if passed else "FAIL"
        if not passed:
            all_passed = False

        results.append({
            "name": bench["name"],
            "predicted": bench["predicted"],
            "expected_exponent": expected,
            "measured_exponent": alpha,
            "status": status,
            "sizes": bench["sizes"],
            "times_ms": [t * 1000 for t in times],
        })

        print(f"[{status}] {bench['name']:<25} | Predicted: {bench['predicted']:<12} | Exp Alpha: {expected:.2f} | Meas Alpha: {alpha:.2f}")

    print("-" * 80)
    print(f"Empirical validation status: {'ALL PASSED' if all_passed else 'SOME FAILED'}")
    print("=" * 80)

    os.makedirs("benchmark/reports", exist_ok=True)
    report_path = "benchmark/reports/empirical_validation_report.md"
    with open(report_path, "w", encoding="utf-8") as f:
        f.write("# Kite0 Empirical Complexity Scaling Report\n\n")
        f.write(r"Empirical power-law scaling analysis comparing predicted asymptotic bounds with measured wall-clock scaling ($\Delta \ln T / \Delta \ln N$)." + "\n\n")
        f.write("| Algorithm Class | Predicted Big-O | Theoretical Exponent | Measured Exponent | Status |\n")
        f.write("| :--- | :--- | :--- | :--- | :--- |\n")
        for r in results:
            f.write(f"| {r['name']} | `{r['predicted']}` | {r['expected_exponent']:.2f} | {r['measured_exponent']:.2f} | **{r['status']}** |\n")

        f.write("\n\n## Per-Benchmark Timing Curves (ms)\n\n")
        for r in results:
            f.write(f"### {r['name']}\n")
            f.write(f"- Sizes: `{r['sizes']}`\n")
            f.write(f"- Times (ms): `{[round(t, 4) for t in r['times_ms']]}`\n")
            f.write(f"- Empirical Exponent: `{r['measured_exponent']:.3f}`\n\n")

    print(f"Wrote report to {report_path}")
    if not all_passed:
        sys.exit(1)

if __name__ == "__main__":
    main()

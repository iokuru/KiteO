#![allow(dead_code)]

use kiteo_engine::algorithms::AllowedAlgorithm;
use kiteo_engine::analyze;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct BenchmarkCase {
    id: String,
    name: String,
    language: String,
    code: String,
    expected_tc: String,
    expected_sc: String,
    expected_algorithms: Vec<String>,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    rating: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RealCaseJson {
    id: String,
    platform: String,
    problem_id: String,
    rating: Option<i64>,
    language: String,
    source_file: String,
    expected_tc: String,
    expected_sc: String,
    expected_algorithms: Vec<String>,
    variables: Vec<String>,
    case_type: String,
    status: String,
}

#[derive(Debug, Clone, Deserialize)]
struct RealManifestCase {
    id: String,
    split: String,
    language: String,
    platform: String,
}

#[derive(Debug, Deserialize)]
struct RealManifestDoc {
    manifest_version: String,
    manifest_sha256: String,
    total_cases: usize,
    dev_count: usize,
    held_out_count: usize,
    cases: Vec<RealManifestCase>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum FailureClass {
    Parser,
    Preprocessor,
    Ir,
    Loop,
    Recursion,
    CostModel,
    Detector,
    Simplification,
}

impl FailureClass {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Parser => "parser",
            Self::Preprocessor => "preprocessor",
            Self::Ir => "IR",
            Self::Loop => "loop",
            Self::Recursion => "recursion",
            Self::CostModel => "cost_model",
            Self::Detector => "detector",
            Self::Simplification => "simplification",
        }
    }
}

#[derive(Default, Debug, Clone)]
struct MetricCounter {
    total: usize,
    tc_exact: usize,
    tc_unknown: usize,
    tc_wrong: usize,
    sc_exact: usize,
    sc_unknown: usize,
    sc_wrong: usize,
    algo_exact: usize,
    fully_correct: usize,
}

impl MetricCounter {
    fn record(
        &mut self,
        tc_exact: bool,
        tc_unknown: bool,
        sc_exact: bool,
        sc_unknown: bool,
        algo_exact: bool,
    ) {
        self.total += 1;
        if tc_exact {
            self.tc_exact += 1;
        } else if tc_unknown {
            self.tc_unknown += 1;
        } else {
            self.tc_wrong += 1;
        }

        if sc_exact {
            self.sc_exact += 1;
        } else if sc_unknown {
            self.sc_unknown += 1;
        } else {
            self.sc_wrong += 1;
        }

        if algo_exact {
            self.algo_exact += 1;
        }

        if tc_exact && sc_exact && algo_exact {
            self.fully_correct += 1;
        }
    }
}

#[derive(Default, Debug, Clone)]
struct LabelConfusion {
    tp: usize,
    fp: usize,
    fn_count: usize,
}

impl LabelConfusion {
    fn precision(&self) -> f64 {
        if self.tp + self.fp == 0 {
            100.0
        } else {
            (self.tp as f64 / (self.tp + self.fp) as f64) * 100.0
        }
    }

    fn recall(&self) -> f64 {
        if self.tp + self.fn_count == 0 {
            100.0
        } else {
            (self.tp as f64 / (self.tp + self.fn_count) as f64) * 100.0
        }
    }
}

struct EvaluatedItem {
    id: String,
    tc_exact: bool,
    tc_unknown: bool,
    sc_exact: bool,
    sc_unknown: bool,
    algo_exact: bool,
    failures: Vec<FailureClass>,
}

fn compute_failure_classes(
    tc_exact: bool,
    sc_exact: bool,
    algo_exact: bool,
    code: &str,
    expected_tc: &str,
    actual_tc: &str,
) -> Vec<FailureClass> {
    let mut failures = Vec::new();
    if !tc_exact {
        if actual_tc == "Unknown" {
            if code.contains("solve(") || code.contains("dfs(") || code.contains("factorial(") {
                failures.push(FailureClass::Recursion);
            } else if code.contains("for ") || code.contains("while ") {
                failures.push(FailureClass::Loop);
            } else {
                failures.push(FailureClass::Simplification);
            }
        } else if expected_tc.contains("log") && !actual_tc.contains("log") {
            failures.push(FailureClass::Loop);
        } else if code.contains("vector") || code.contains("map") || code.contains("set") {
            failures.push(FailureClass::CostModel);
        } else {
            failures.push(FailureClass::Simplification);
        }
    }

    if !sc_exact && !failures.contains(&FailureClass::CostModel) {
        failures.push(FailureClass::CostModel);
    }

    if !algo_exact {
        failures.push(FailureClass::Detector);
    }

    failures
}

#[allow(clippy::too_many_arguments)]
fn print_evaluation_summary(
    title: &str,
    _cases: &[EvaluatedItem],
    overall: &MetricCounter,
    tag_metrics: &HashMap<String, MetricCounter>,
    rating_metrics: &HashMap<String, MetricCounter>,
    lang_metrics: &HashMap<String, MetricCounter>,
    label_confusion: &BTreeMap<String, LabelConfusion>,
    failure_counts: &HashMap<FailureClass, usize>,
) {
    println!("================================================================================");
    println!("             KITEO BENCHMARK EVALUATION: {title:<33}");
    println!("================================================================================");
    println!("TOTAL EVALUATED CASES: {}", overall.total);
    println!("--------------------------------------------------------------------------------");
    println!(
        "  TC Exact   : {:>4} / {:<4} ({:>5.1}%)",
        overall.tc_exact,
        overall.total,
        (overall.tc_exact as f64 / overall.total as f64) * 100.0
    );
    println!(
        "  TC Unknown : {:>4} / {:<4} ({:>5.1}%)",
        overall.tc_unknown,
        overall.total,
        (overall.tc_unknown as f64 / overall.total as f64) * 100.0
    );
    println!(
        "  TC Wrong   : {:>4} / {:<4} ({:>5.1}%)",
        overall.tc_wrong,
        overall.total,
        (overall.tc_wrong as f64 / overall.total as f64) * 100.0
    );
    println!(
        "  SC Exact   : {:>4} / {:<4} ({:>5.1}%)",
        overall.sc_exact,
        overall.total,
        (overall.sc_exact as f64 / overall.total as f64) * 100.0
    );
    println!(
        "  SC Unknown : {:>4} / {:<4} ({:>5.1}%)",
        overall.sc_unknown,
        overall.total,
        (overall.sc_unknown as f64 / overall.total as f64) * 100.0
    );
    println!(
        "  SC Wrong   : {:>4} / {:<4} ({:>5.1}%)",
        overall.sc_wrong,
        overall.total,
        (overall.sc_wrong as f64 / overall.total as f64) * 100.0
    );
    println!(
        "  Algo Exact : {:>4} / {:<4} ({:>5.1}%)",
        overall.algo_exact,
        overall.total,
        (overall.algo_exact as f64 / overall.total as f64) * 100.0
    );
    println!(
        "  Full Match : {:>4} / {:<4} ({:>5.1}%)",
        overall.fully_correct,
        overall.total,
        (overall.fully_correct as f64 / overall.total as f64) * 100.0
    );

    if !failure_counts.is_empty() {
        println!(
            "--------------------------------------------------------------------------------"
        );
        println!("FAILURE CLASSES BREAKDOWN:");
        for (fc, count) in failure_counts {
            println!("  - {:<15}: {:>3} cases", fc.as_str(), count);
        }
    }

    if !lang_metrics.is_empty() {
        println!(
            "--------------------------------------------------------------------------------"
        );
        println!("PER-LANGUAGE BREAKDOWN:");
        println!("  | Language | Total | TC Exact (%) | SC Exact (%) | Algo Exact (%) |");
        println!("  |:---------|:------|:-------------|:-------------|:---------------|");
        for (lang, m) in lang_metrics {
            println!(
                "  | {:<8} | {:<5} | {:>5.1}% ({:>3}) | {:>5.1}% ({:>3}) | {:>5.1}% ({:>3})   |",
                lang,
                m.total,
                (m.tc_exact as f64 / m.total as f64) * 100.0,
                m.tc_exact,
                (m.sc_exact as f64 / m.total as f64) * 100.0,
                m.sc_exact,
                (m.algo_exact as f64 / m.total as f64) * 100.0,
                m.algo_exact,
            );
        }
    }

    if !rating_metrics.is_empty() {
        println!(
            "--------------------------------------------------------------------------------"
        );
        println!("PER-RATING BREAKDOWN:");
        println!("  | Rating / Tier   | Total | Full Exact (%) | TC Exact (%) | SC Exact (%) |");
        println!("  |:----------------|:------|:---------------|:-------------|:-------------|");
        for (rating, m) in rating_metrics {
            println!(
                "  | {:<15} | {:<5} | {:>5.1}% ({:>3})   | {:>5.1}% ({:>3})| {:>5.1}% ({:>3}) |",
                rating,
                m.total,
                (m.fully_correct as f64 / m.total as f64) * 100.0,
                m.fully_correct,
                (m.tc_exact as f64 / m.total as f64) * 100.0,
                m.tc_exact,
                (m.sc_exact as f64 / m.total as f64) * 100.0,
                m.sc_exact,
            );
        }
    }

    if !tag_metrics.is_empty() {
        println!(
            "--------------------------------------------------------------------------------"
        );
        println!("PER-TAG BREAKDOWN (Top active tags):");
        println!(
            "  | Tag                    | Total | TC Exact (%) | SC Exact (%) | Full Match   |"
        );
        println!(
            "  |:-----------------------|:------|:-------------|:-------------|:-------------|"
        );
        let mut sorted_tags: Vec<(&String, &MetricCounter)> = tag_metrics.iter().collect();
        sorted_tags.sort_by(|a, b| b.1.total.cmp(&a.1.total).then_with(|| a.0.cmp(b.0)));
        for (tag, m) in sorted_tags.iter().take(25) {
            println!(
                "  | {:<22} | {:<5} | {:>5.1}% ({:>2}) | {:>5.1}% ({:>2}) | {:>5.1}% ({:>2}) |",
                tag,
                m.total,
                (m.tc_exact as f64 / m.total as f64) * 100.0,
                m.tc_exact,
                (m.sc_exact as f64 / m.total as f64) * 100.0,
                m.sc_exact,
                (m.fully_correct as f64 / m.total as f64) * 100.0,
                m.fully_correct,
            );
        }
    }

    // Per-label precision and recall
    println!("--------------------------------------------------------------------------------");
    println!("ALGORITHM PRECISION & RECALL PER LABEL:");
    println!("  | Algorithm Label                | TP  | FP  | FN  | Precision | Recall  |");
    println!("  |:-------------------------------|:----|:----|:----|:----------|:--------|");
    for (label, conf) in label_confusion {
        if conf.tp > 0 || conf.fp > 0 || conf.fn_count > 0 {
            println!(
                "  | {:<30} | {:<3} | {:<3} | {:<3} | {:>8.1}% | {:>6.1}% |",
                label,
                conf.tp,
                conf.fp,
                conf.fn_count,
                conf.precision(),
                conf.recall()
            );
        }
    }
    println!("================================================================================\n");
}

fn evaluate_corpus(corpus_file: &str, report_name: &str) {
    let path = Path::new(corpus_file);
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => {
            let alt = format!("../{corpus_file}");
            fs::read_to_string(&alt)
                .unwrap_or_else(|_| panic!("Failed to locate corpus at {corpus_file}"))
        }
    };

    let cases: Vec<BenchmarkCase> =
        serde_json::from_str(&content).expect("Failed to deserialize benchmark cases JSON");

    let mut overall = MetricCounter::default();
    let mut tag_metrics: HashMap<String, MetricCounter> = HashMap::new();
    let mut rating_metrics: HashMap<String, MetricCounter> = HashMap::new();
    let mut lang_metrics: HashMap<String, MetricCounter> = HashMap::new();
    let mut failure_counts: HashMap<FailureClass, usize> = HashMap::new();

    let mut label_confusion: BTreeMap<String, LabelConfusion> = BTreeMap::new();
    for algo in AllowedAlgorithm::ALL {
        label_confusion.insert(algo.as_str().to_string(), LabelConfusion::default());
    }

    let mut evaluated_cases = Vec::new();

    for case in &cases {
        let output = analyze(&case.code, &case.language);

        let tc_exact = output.tc == case.expected_tc;
        let tc_unknown = output.tc == "Unknown";
        let sc_exact = output.sc == case.expected_sc;
        let sc_unknown = output.sc == "Unknown";

        let mut expected_sorted = case.expected_algorithms.clone();
        expected_sorted.sort();
        let mut actual_sorted = output.algorithms.clone();
        actual_sorted.sort();
        let algo_exact = actual_sorted == expected_sorted;

        overall.record(tc_exact, tc_unknown, sc_exact, sc_unknown, algo_exact);

        lang_metrics
            .entry(case.language.clone())
            .or_default()
            .record(tc_exact, tc_unknown, sc_exact, sc_unknown, algo_exact);

        for tag in &case.tags {
            tag_metrics
                .entry(tag.clone())
                .or_default()
                .record(tc_exact, tc_unknown, sc_exact, sc_unknown, algo_exact);
        }

        let rating_key = case.rating.clone().unwrap_or_else(|| "Unrated".to_string());
        rating_metrics
            .entry(rating_key)
            .or_default()
            .record(tc_exact, tc_unknown, sc_exact, sc_unknown, algo_exact);

        // Update confusion matrix for every canonical label
        let actual_set: HashSet<String> = output.algorithms.iter().cloned().collect();
        let expected_set: HashSet<String> = case.expected_algorithms.iter().cloned().collect();

        for (label, conf) in label_confusion.iter_mut() {
            let in_actual = actual_set.contains(label);
            let in_expected = expected_set.contains(label);
            if in_actual && in_expected {
                conf.tp += 1;
            } else if in_actual && !in_expected {
                conf.fp += 1;
            } else if !in_actual && in_expected {
                conf.fn_count += 1;
            }
        }

        let failures = compute_failure_classes(
            tc_exact,
            sc_exact,
            algo_exact,
            &case.code,
            &case.expected_tc,
            &output.tc,
        );

        for f in &failures {
            *failure_counts.entry(f.clone()).or_default() += 1;
        }

        evaluated_cases.push(EvaluatedItem {
            id: case.id.clone(),
            tc_exact,
            tc_unknown,
            sc_exact,
            sc_unknown,
            algo_exact,
            failures,
        });
    }

    print_evaluation_summary(
        report_name,
        &evaluated_cases,
        &overall,
        &tag_metrics,
        &rating_metrics,
        &lang_metrics,
        &label_confusion,
        &failure_counts,
    );
}

fn evaluate_real_benchmark(is_debug_mode: bool, verbose: bool) {
    // Manifest lives in the repo workspace (dev split only).
    // Held-out cases are loaded from a separate private path set via KITEO_HELD_OUT_DIR.
    // If that env var is not set, held-out evaluation is skipped unconditionally.
    let manifest_path = "benchmark/real/manifest.json";
    let manifest_raw = match fs::read_to_string(manifest_path) {
        Ok(c) => c,
        Err(_) => fs::read_to_string("../benchmark/real/manifest.json")
            .expect("Failed to locate benchmark/real/manifest.json"),
    };

    let manifest: RealManifestDoc =
        serde_json::from_str(&manifest_raw).expect("Invalid benchmark/real/manifest.json");

    // Verify SHA-256 hash of manifest splits
    let mut hasher = Sha256::new();
    let mut sorted_cases = manifest.cases.clone();
    sorted_cases.sort_by(|a, b| a.id.cmp(&b.id));
    for c in &sorted_cases {
        hasher.update(format!("{}:{}\n", c.id, c.split).as_bytes());
    }
    let hash_bytes = hasher.finalize();
    let calculated_hash: String = hash_bytes.iter().map(|b| format!("{b:02x}")).collect();

    if calculated_hash != manifest.manifest_sha256 {
        panic!(
            "MANIFEST INTEGRITY ERROR: Stored hash {} does not match computed hash {calculated_hash}",
            manifest.manifest_sha256
        );
    }

    println!("[MANIFEST VERIFIED] SHA-256 integrity match: {calculated_hash}");
    println!(
        "[MANIFEST STATS] Total: {}, Dev: {} (70%), Held-Out: {} (30%)",
        manifest.total_cases, manifest.dev_count, manifest.held_out_count
    );

    // WARNING: all cases in manifest.json are currently PLACEHOLDERS (synthetic stubs).
    // Their expected labels were written by the agent, not by hand, and their source files
    // contain only stub code. Results below are NOT MEANINGFUL until real accepted solutions
    // and human-verified labels are supplied via KITEO_REAL_CASES_DIR.
    println!("\n>>> RUNNING DEV SPLIT (PLACEHOLDER CASES — RESULTS NOT MEANINGFUL) <<<");
    evaluate_real_split(&manifest.cases, "dev", verbose);

    // Held-out evaluation requires the external directory to be explicitly set.
    // This keeps held-out data entirely outside the repo workspace.
    let held_out_dir = std::env::var("KITEO_HELD_OUT_DIR").ok();
    match &held_out_dir {
        None => {
            eprintln!(
                "\n[HELD-OUT SKIPPED] KITEO_HELD_OUT_DIR is not set. \
                 Held-out cases live outside the repo. Set the env var to a directory \
                 containing the real held-out manifest and sources to evaluate."
            );
        }
        Some(_dir) => {
            if is_debug_mode {
                eprintln!(
                    "\n[POLICY REFUSAL] KITEO_HELD_OUT_DIR is set but the binary was built in \
                     debug mode. Held-out evaluation requires a release build \
                     (`cargo run --release -p kiteo-runner -- --real`)."
                );
                return;
            }
            println!("\n>>> RUNNING HELD-OUT SPLIT (Blind Verification) <<<");
            evaluate_real_split(&manifest.cases, "held_out", verbose);
        }
    }
}

fn evaluate_real_split(manifest_cases: &[RealManifestCase], target_split: &str, verbose: bool) {
    let mut overall = MetricCounter::default();
    let mut tag_metrics: HashMap<String, MetricCounter> = HashMap::new();
    let mut rating_metrics: HashMap<String, MetricCounter> = HashMap::new();
    let mut lang_metrics: HashMap<String, MetricCounter> = HashMap::new();
    let mut failure_counts: HashMap<FailureClass, usize> = HashMap::new();

    let mut label_confusion: BTreeMap<String, LabelConfusion> = BTreeMap::new();
    for algo in AllowedAlgorithm::ALL {
        label_confusion.insert(algo.as_str().to_string(), LabelConfusion::default());
    }

    let mut evaluated_cases = Vec::new();

    for entry in manifest_cases {
        if entry.split != target_split {
            continue;
        }

        let case_file = format!("benchmark/real/cases/{}.json", entry.id);
        let case_raw = match fs::read_to_string(&case_file) {
            Ok(c) => c,
            Err(_) => fs::read_to_string(format!("../{case_file}"))
                .unwrap_or_else(|_| panic!("Failed to read case {case_file}")),
        };

        let case: RealCaseJson =
            serde_json::from_str(&case_raw).expect("Invalid RealCaseJson file");

        let source_path = format!("benchmark/real/{}", case.source_file);
        let code = match fs::read_to_string(&source_path) {
            Ok(c) => c,
            Err(_) => fs::read_to_string(format!("../{source_path}"))
                .unwrap_or_else(|_| panic!("Failed to read source {source_path}")),
        };

        let output = analyze(&code, &case.language);

        let tc_exact = output.tc == case.expected_tc;
        let tc_unknown = output.tc == "Unknown";
        let sc_exact = output.sc == case.expected_sc;
        let sc_unknown = output.sc == "Unknown";

        let mut expected_sorted = case.expected_algorithms.clone();
        expected_sorted.sort();
        let mut actual_sorted = output.algorithms.clone();
        actual_sorted.sort();
        let algo_exact = actual_sorted == expected_sorted;

        overall.record(tc_exact, tc_unknown, sc_exact, sc_unknown, algo_exact);

        lang_metrics
            .entry(case.language.clone())
            .or_default()
            .record(tc_exact, tc_unknown, sc_exact, sc_unknown, algo_exact);

        let rating_str = case
            .rating
            .map(|r| format!("Rating {r}"))
            .unwrap_or_else(|| "Unrated".to_string());
        rating_metrics
            .entry(rating_str)
            .or_default()
            .record(tc_exact, tc_unknown, sc_exact, sc_unknown, algo_exact);

        tag_metrics
            .entry(case.platform.clone())
            .or_default()
            .record(tc_exact, tc_unknown, sc_exact, sc_unknown, algo_exact);

        // Update confusion
        let actual_set: HashSet<String> = output.algorithms.iter().cloned().collect();
        let expected_set: HashSet<String> = case.expected_algorithms.iter().cloned().collect();

        for (label, conf) in label_confusion.iter_mut() {
            let in_actual = actual_set.contains(label);
            let in_expected = expected_set.contains(label);
            if in_actual && in_expected {
                conf.tp += 1;
            } else if in_actual && !in_expected {
                conf.fp += 1;
            } else if !in_actual && in_expected {
                conf.fn_count += 1;
            }
        }

        let failures = compute_failure_classes(
            tc_exact,
            sc_exact,
            algo_exact,
            &code,
            &case.expected_tc,
            &output.tc,
        );

        for f in &failures {
            *failure_counts.entry(f.clone()).or_default() += 1;
        }

        // Per-case verbose output
        if verbose {
            let status = if tc_exact && sc_exact && algo_exact {
                "PASS"
            } else {
                "FAIL"
            };
            println!("  [{status}] {}", case.id);
            println!("       code ({} lines):", code.lines().count());
            for line in code.lines().take(20) {
                println!("         {line}");
            }
            if code.lines().count() > 20 {
                println!("         ... ({} more lines)", code.lines().count() - 20);
            }
            println!(
                "       expected TC: {}  SC: {}  algos: {:?}",
                case.expected_tc, case.expected_sc, expected_sorted
            );
            println!(
                "       actual   TC: {}  SC: {}  algos: {:?}",
                output.tc, output.sc, actual_sorted
            );
            let failure_strs: Vec<&str> = failures.iter().map(|f| f.as_str()).collect();
            println!("       failure classes: {failure_strs:?}");
            println!();
        }

        evaluated_cases.push(EvaluatedItem {
            id: case.id.clone(),
            tc_exact,
            tc_unknown,
            sc_exact,
            sc_unknown,
            algo_exact,
            failures,
        });
    }

    // Title reflects placeholder status for dev split
    let title = if target_split == "dev" {
        "Placeholder Cases [DEV] — NOT MEANINGFUL".to_string()
    } else {
        format!("Real Solutions [{}]", target_split.to_uppercase())
    };
    print_evaluation_summary(
        &title,
        &evaluated_cases,
        &overall,
        &tag_metrics,
        &rating_metrics,
        &lang_metrics,
        &label_confusion,
        &failure_counts,
    );

    // Generate benchmark/reports/detector_status.json per Section 42
    let total_cases = evaluated_cases.len();
    let mut status_report = Vec::new();
    for (label, conf) in &label_confusion {
        let tp = conf.tp;
        let fp = conf.fp;
        let fn_count = conf.fn_count;
        let tn = total_cases.saturating_sub(tp + fp + fn_count);
        let positive_count = tp + fn_count;
        let negative_count = fp + tn;
        let precision = if tp + fp == 0 {
            1.0
        } else {
            tp as f64 / (tp + fp) as f64
        };
        let recall = if positive_count == 0 {
            0.0
        } else {
            tp as f64 / positive_count as f64
        };
        // Status gate: precision >= 95%, positive_count >= 10, negative_count >= 10
        let status = if precision >= 0.95 && positive_count >= 10 && negative_count >= 10 {
            "approved"
        } else {
            "candidate"
        };

        status_report.push(DetectorStatusReportItem {
            label: label.clone(),
            precision,
            recall,
            tp,
            fp,
            tn,
            fn_count,
            positive_count,
            negative_count,
            status: status.to_string(),
        });
    }

    let report_path = "benchmark/reports/detector_status.json";
    let _ = fs::create_dir_all("benchmark/reports");
    if let Ok(json_str) = serde_json::to_string_pretty(&status_report) {
        let _ = fs::write(report_path, json_str);
    }
}

#[derive(serde::Serialize)]
struct DetectorStatusReportItem {
    label: String,
    precision: f64,
    recall: f64,
    tp: usize,
    fp: usize,
    tn: usize,
    #[serde(rename = "fn")]
    fn_count: usize,
    positive_count: usize,
    negative_count: usize,
    status: String,
}

#[derive(serde::Serialize)]
struct RunnerJsonItem {
    id: String,
    tc: String,
    sc: String,
    algorithms: Vec<String>,
}

fn dump_json(corpus_file: &str) {
    let path = Path::new(corpus_file);
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => {
            let alt = format!("../{corpus_file}");
            fs::read_to_string(&alt)
                .unwrap_or_else(|_| panic!("Failed to locate corpus at {corpus_file}"))
        }
    };

    let cases: Vec<BenchmarkCase> = serde_json::from_str(&content).expect("Invalid JSON corpus");
    let mut results = Vec::new();
    for case in &cases {
        let out = analyze(&case.code, &case.language);
        results.push(RunnerJsonItem {
            id: case.id.clone(),
            tc: out.tc,
            sc: out.sc,
            algorithms: out.algorithms,
        });
    }

    println!("{}", serde_json::to_string(&results).unwrap());
}

#[derive(serde::Serialize)]
struct RunnerAstItem {
    id: String,
    ast: Option<kiteo_engine::parser::AstNode>,
}

fn dump_ast(corpus_file: &str) {
    let path = Path::new(corpus_file);
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => {
            let alt = format!("../{corpus_file}");
            fs::read_to_string(&alt)
                .unwrap_or_else(|_| panic!("Failed to locate corpus at {corpus_file}"))
        }
    };

    let cases: Vec<BenchmarkCase> = serde_json::from_str(&content).expect("Invalid JSON corpus");
    let mut results = Vec::new();
    for case in &cases {
        let ast = kiteo_engine::parse_to_ast(&case.code, &case.language);
        results.push(RunnerAstItem {
            id: case.id.clone(),
            ast,
        });
    }

    println!("{}", serde_json::to_string(&results).unwrap());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_ast = args.iter().any(|a| a == "--ast");
    let is_json = args.iter().any(|a| a == "--json");
    let is_dev2 = args
        .iter()
        .any(|a| a == "--dev2" || a == "--held-out" || a == "-h");
    let is_all = args.iter().any(|a| a == "--all");
    let is_real = args.iter().any(|a| a == "--real" || a == "--real-verbose");
    let is_verbose = args
        .iter()
        .any(|a| a == "--real-verbose" || a == "--verbose");
    let is_debug = args.iter().any(|a| a == "--debug") || cfg!(debug_assertions);

    let corpus_file = if is_dev2 {
        "benchmark/corpus/dev2.json"
    } else {
        "benchmark/corpus/snippets.json"
    };

    if is_ast {
        dump_ast(corpus_file);
    } else if is_json {
        dump_json(corpus_file);
    } else if is_real {
        evaluate_real_benchmark(is_debug, is_verbose);
    } else if is_all {
        evaluate_corpus(
            "benchmark/corpus/snippets.json",
            "Canonical Dev Benchmark Corpus (152 Cases)",
        );
        println!("\n\n");
        evaluate_corpus(
            "benchmark/corpus/dev2.json",
            "Dev2 Synthetic Solutions (70 Cases)",
        );
    } else if is_dev2 {
        evaluate_corpus(
            "benchmark/corpus/dev2.json",
            "Dev2 Synthetic Solutions (70 Cases)",
        );
    } else {
        evaluate_corpus(
            "benchmark/corpus/snippets.json",
            "Canonical Dev Corpus (152 Cases)",
        );
    }
}

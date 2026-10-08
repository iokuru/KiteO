use kiteo_engine::analyze;
use serde::Deserialize;
use std::collections::HashMap;
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

#[derive(Debug, PartialEq, Eq)]
enum FailureClass {
    UnknownResult,
    ComplexityMismatch,
    AlgorithmMismatch,
}

#[derive(Debug)]
#[allow(dead_code)]
struct CaseEvaluation {
    id: String,
    name: String,
    tc_match: bool,
    sc_match: bool,
    algo_match: bool,
    is_match: bool,
    is_unknown: bool,
    is_wrong: bool,
    failures: Vec<FailureClass>,
}

#[derive(Default, Debug, Clone)]
struct MetricCounter {
    total: usize,
    matches: usize,
    unknowns: usize,
    wrongs: usize,
}

impl MetricCounter {
    fn record(&mut self, is_match: bool, is_unknown: bool, is_wrong: bool) {
        self.total += 1;
        if is_match {
            self.matches += 1;
        } else if is_unknown {
            self.unknowns += 1;
        } else if is_wrong {
            self.wrongs += 1;
        }
    }

    fn accuracy_pct(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.matches as f64 / self.total as f64) * 100.0
        }
    }

    fn unknown_pct(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.unknowns as f64 / self.total as f64) * 100.0
        }
    }

    fn wrong_pct(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.wrongs as f64 / self.total as f64) * 100.0
        }
    }
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

    let total = cases.len();
    let mut overall = MetricCounter::default();
    let mut tag_metrics: HashMap<String, MetricCounter> = HashMap::new();
    let mut rating_metrics: HashMap<String, MetricCounter> = HashMap::new();
    let mut evaluations = Vec::new();

    println!("================================================================================");
    println!("             KITEO BENCHMARK EVALUATION: {report_name:<33}");
    println!("================================================================================");

    for case in &cases {
        let output = analyze(&case.code, &case.language);

        let tc_match = output.tc == case.expected_tc;
        let sc_match = output.sc == case.expected_sc;

        let mut expected_sorted = case.expected_algorithms.clone();
        expected_sorted.sort();
        let mut actual_sorted = output.algorithms.clone();
        actual_sorted.sort();
        let algo_match = actual_sorted == expected_sorted;

        let is_match = tc_match && sc_match && algo_match;
        let is_unknown = !is_match
            && (output.tc == "Unknown"
                || output.sc == "Unknown"
                || (output.algorithms.is_empty() && !case.expected_algorithms.is_empty()));
        let is_wrong = !is_match && !is_unknown;

        overall.record(is_match, is_unknown, is_wrong);

        for tag in &case.tags {
            tag_metrics
                .entry(tag.clone())
                .or_default()
                .record(is_match, is_unknown, is_wrong);
        }

        let rating_key = case.rating.clone().unwrap_or_else(|| "Unrated".to_string());
        rating_metrics
            .entry(rating_key)
            .or_default()
            .record(is_match, is_unknown, is_wrong);

        let mut failures = Vec::new();
        if !tc_match {
            if output.tc == "Unknown" {
                failures.push(FailureClass::UnknownResult);
            } else {
                failures.push(FailureClass::ComplexityMismatch);
            }
        }
        if !sc_match && output.sc == "Unknown" && !failures.contains(&FailureClass::UnknownResult) {
            failures.push(FailureClass::UnknownResult);
        }
        if !algo_match {
            failures.push(FailureClass::AlgorithmMismatch);
        }

        evaluations.push(CaseEvaluation {
            id: case.id.clone(),
            name: case.name.clone(),
            tc_match,
            sc_match,
            algo_match,
            is_match,
            is_unknown,
            is_wrong,
            failures,
        });

        let status = if is_match {
            "PASS"
        } else if is_unknown {
            "UNKN"
        } else {
            "FAIL"
        };

        let fail_summary: Vec<_> = evaluations
            .last()
            .unwrap()
            .failures
            .iter()
            .map(|f| format!("{f:?}"))
            .collect();

        println!(
            "[{status}] {:<36} | TC: {:<12} (exp {:<12}) | SC: {:<6} | Fails: [{}]",
            case.id,
            output.tc,
            case.expected_tc,
            output.sc,
            fail_summary.join(", ")
        );
    }

    println!("--------------------------------------------------------------------------------");
    println!("OVERALL METRICS SUMMARY (Total Cases: {total})");
    println!(
        "  Accuracy Rate : {:>3}/{} ({:.1}%)",
        overall.matches,
        total,
        overall.accuracy_pct()
    );
    println!(
        "  Unknown Rate  : {:>3}/{} ({:.1}%)",
        overall.unknowns,
        total,
        overall.unknown_pct()
    );
    println!(
        "  Wrong Rate    : {:>3}/{} ({:.1}%)",
        overall.wrongs,
        total,
        overall.wrong_pct()
    );
    println!("--------------------------------------------------------------------------------");

    println!("PER-TAG BREAKDOWN:");
    println!(
        "  | {:<22} | {:<5} | {:<12} | {:<12} | {:<10} |",
        "Tag", "Total", "Accuracy", "Unknown Rate", "Wrong Rate"
    );
    println!("  |:-----------------------|:------|:-------------|:-------------|:-----------|");
    let mut sorted_tags: Vec<_> = tag_metrics.into_iter().collect();
    sorted_tags.sort_by(|a, b| a.0.cmp(&b.0));
    for (tag, m) in &sorted_tags {
        println!(
            "  | {:<22} | {:<5} | {:>5.1}% ({:>2}) | {:>5.1}% ({:>2}) | {:>5.1}% ({:>2}) |",
            tag,
            m.total,
            m.accuracy_pct(),
            m.matches,
            m.unknown_pct(),
            m.unknowns,
            m.wrong_pct(),
            m.wrongs
        );
    }
    println!("--------------------------------------------------------------------------------");

    println!("PER-RATING BREAKDOWN:");
    println!(
        "  | {:<15} | {:<5} | {:<12} | {:<12} | {:<10} |",
        "Rating / Tier", "Total", "Accuracy", "Unknown Rate", "Wrong Rate"
    );
    println!("  |:----------------|:------|:-------------|:-------------|:-----------|");
    let mut sorted_ratings: Vec<_> = rating_metrics.into_iter().collect();
    sorted_ratings.sort_by(|a, b| {
        let a_num = a.0.parse::<i32>().unwrap_or(9999);
        let b_num = b.0.parse::<i32>().unwrap_or(9999);
        if a_num != b_num {
            a_num.cmp(&b_num)
        } else {
            a.0.cmp(&b.0)
        }
    });
    for (rating, m) in &sorted_ratings {
        println!(
            "  | {:<15} | {:<5} | {:>5.1}% ({:>2}) | {:>5.1}% ({:>2}) | {:>5.1}% ({:>2}) |",
            rating,
            m.total,
            m.accuracy_pct(),
            m.matches,
            m.unknown_pct(),
            m.unknowns,
            m.wrong_pct(),
            m.wrongs
        );
    }
    println!("================================================================================");

    // Write markdown report
    let mut md = String::new();
    md.push_str(&format!("# KiteO Evaluation Report: {report_name}\n\n"));
    md.push_str(&format!("**Dataset File**: `{corpus_file}`  \n"));
    md.push_str(&format!("**Total Problems**: `{total}`  \n\n"));

    md.push_str("## 1. Overall Summary\n\n");
    md.push_str("| Metric | Count | Percentage |\n");
    md.push_str("| :--- | :--- | :--- |\n");
    md.push_str(&format!(
        "| **Accuracy Rate** | {}/{} | **{:.1}%** |\n",
        overall.matches,
        total,
        overall.accuracy_pct()
    ));
    md.push_str(&format!(
        "| **Unknown Rate** | {}/{} | {:.1}% |\n",
        overall.unknowns,
        total,
        overall.unknown_pct()
    ));
    md.push_str(&format!(
        "| **Wrong Rate** | {}/{} | {:.1}% |\n\n",
        overall.wrongs,
        total,
        overall.wrong_pct()
    ));

    md.push_str("## 2. Breakdown Per Tag\n\n");
    md.push_str("| Tag | Total | Accuracy | Unknown Rate | Wrong Rate |\n");
    md.push_str("| :--- | :--- | :--- | :--- | :--- |\n");
    for (tag, m) in &sorted_tags {
        md.push_str(&format!(
            "| `{}` | {} | {:.1}% ({}) | {:.1}% ({}) | {:.1}% ({}) |\n",
            tag,
            m.total,
            m.accuracy_pct(),
            m.matches,
            m.unknown_pct(),
            m.unknowns,
            m.wrong_pct(),
            m.wrongs
        ));
    }

    md.push_str("\n## 3. Breakdown Per Rating / Tier\n\n");
    md.push_str("| Rating / Tier | Total | Accuracy | Unknown Rate | Wrong Rate |\n");
    md.push_str("| :--- | :--- | :--- | :--- | :--- |\n");
    for (rating, m) in &sorted_ratings {
        md.push_str(&format!(
            "| `{}` | {} | {:.1}% ({}) | {:.1}% ({}) | {:.1}% ({}) |\n",
            rating,
            m.total,
            m.accuracy_pct(),
            m.matches,
            m.unknown_pct(),
            m.unknowns,
            m.wrong_pct(),
            m.wrongs
        ));
    }

    let report_filename = if corpus_file.contains("dev2") || corpus_file.contains("held_out") {
        "benchmark/reports/dev2_evaluation_report.md"
    } else {
        "benchmark/reports/benchmark_evaluation_report.md"
    };

    let _ = fs::create_dir_all("benchmark/reports");
    let _ = fs::write(report_filename, md);
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

    let corpus_file = if is_dev2 {
        "benchmark/corpus/dev2.json"
    } else {
        "benchmark/corpus/snippets.json"
    };

    if is_ast {
        dump_ast(corpus_file);
    } else if is_json {
        dump_json(corpus_file);
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

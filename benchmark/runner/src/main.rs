use kiteo_engine::analyze;
use serde::Deserialize;
use std::collections::HashMap;
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
    failures: Vec<FailureClass>,
}

fn main() {
    let corpus_path = Path::new("benchmark/corpus/snippets.json");
    let content = match fs::read_to_string(corpus_path) {
        Ok(c) => c,
        Err(_) => {
            // Check fallback relative path if running from benchmark/runner
            fs::read_to_string("../corpus/snippets.json")
                .expect("Failed to locate snippets.json benchmark corpus")
        }
    };

    let cases: Vec<BenchmarkCase> =
        serde_json::from_str(&content).expect("Failed to deserialize benchmark cases JSON");

    let total = cases.len();
    let mut tc_matches = 0;
    let mut sc_matches = 0;
    let mut algo_matches = 0;
    let mut tag_stats: HashMap<String, (usize, usize)> = HashMap::new();

    println!("================================================================================");
    println!("                           KITEO BENCHMARK EVALUATION                           ");
    println!("================================================================================");

    let mut evaluations = Vec::new();

    for case in &cases {
        let output = analyze(&case.code, &case.language);

        let tc_match = output.tc == case.expected_tc;
        let sc_match = output.sc == case.expected_sc;

        let mut expected_sorted = case.expected_algorithms.clone();
        expected_sorted.sort();
        let mut actual_sorted = output.algorithms.clone();
        actual_sorted.sort();
        let algo_match = actual_sorted == expected_sorted;

        if tc_match {
            tc_matches += 1;
        }
        if sc_match {
            sc_matches += 1;
        }
        if algo_match {
            algo_matches += 1;
        }

        for tag in &case.tags {
            let entry = tag_stats.entry(tag.clone()).or_insert((0, 0));
            entry.0 += 1;
            if tc_match && sc_match && algo_match {
                entry.1 += 1;
            }
        }

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
            failures,
        });

        let status = if tc_match && sc_match && algo_match {
            "PASS"
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
            "[{status}] {:<32} | TC: {:<12} (exp {:<12}) | SC: {:<8} | Fails: [{}]",
            case.id,
            output.tc,
            case.expected_tc,
            output.sc,
            fail_summary.join(", ")
        );
    }

    let tc_pct = (tc_matches as f64 / total as f64) * 100.0;
    let sc_pct = (sc_matches as f64 / total as f64) * 100.0;
    let algo_pct = (algo_matches as f64 / total as f64) * 100.0;

    println!("--------------------------------------------------------------------------------");
    println!("SUMMARY: Total Cases: {total}");
    println!("  Time Complexity Accuracy : {tc_matches}/{total} ({tc_pct:.1}%)");
    println!("  Space Complexity Accuracy: {sc_matches}/{total} ({sc_pct:.1}%)");
    println!("  Algorithm Match Accuracy : {algo_matches}/{total} ({algo_pct:.1}%)");
    println!("--------------------------------------------------------------------------------");
    println!("PER-TAG BREAKDOWN:");
    let mut sorted_tags: Vec<_> = tag_stats.into_iter().collect();
    sorted_tags.sort_by(|a, b| a.0.cmp(&b.0));
    for (tag, (count, passed)) in sorted_tags {
        let pct = (passed as f64 / count as f64) * 100.0;
        println!("  Tag '{tag:<20}': {passed}/{count} fully passed ({pct:.1}%)");
    }
    println!("================================================================================");
}

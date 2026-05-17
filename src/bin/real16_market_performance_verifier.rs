use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use turingosv4::runtime::market_e2_candidate_verifier::E2CandidateVerifierReport;
use turingosv4::runtime::market_performance_e4::{
    derive_arm_input_from_evidence, evaluate_market_performance_e4, E4ArmEvidenceConfig,
    E4ArmInput, E4Verdict,
};

#[derive(Debug, Serialize, Deserialize)]
struct InputFile {
    arms: Vec<E4ArmInput>,
}

fn usage() -> ! {
    eprintln!(
        "usage:\n  real16_market_performance_verifier --input-json <arms.json> --json-out <report.json> --md-out <report.md>\n  real16_market_performance_verifier --derive-arm-json --arm-id <A|B|C|D> --evidence-dir <dir> --e2-json <report.json> --problem-set-hash <sha> --model-assignment-hash <sha> --budget-hash <sha> --prompt-template-hash <sha> --runtime-config-hash <sha> --market-pressure-enabled <true|false> --json-out <arm.json>"
    );
    std::process::exit(2);
}

fn main() {
    let mut args = std::env::args().skip(1);
    let mut input_json: Option<PathBuf> = None;
    let mut json_out: Option<PathBuf> = None;
    let mut md_out: Option<PathBuf> = None;
    let mut derive_arm_json = false;
    let mut arm_id: Option<String> = None;
    let mut evidence_dir: Option<String> = None;
    let mut e2_json: Option<PathBuf> = None;
    let mut problem_set_hash: Option<String> = None;
    let mut model_assignment_hash: Option<String> = None;
    let mut budget_hash: Option<String> = None;
    let mut prompt_template_hash: Option<String> = None;
    let mut runtime_config_hash: Option<String> = None;
    let mut market_pressure_enabled: Option<bool> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--derive-arm-json" => derive_arm_json = true,
            "--arm-id" => arm_id = args.next(),
            "--evidence-dir" => evidence_dir = args.next(),
            "--e2-json" => e2_json = args.next().map(PathBuf::from),
            "--problem-set-hash" => problem_set_hash = args.next(),
            "--model-assignment-hash" => model_assignment_hash = args.next(),
            "--budget-hash" => budget_hash = args.next(),
            "--prompt-template-hash" => prompt_template_hash = args.next(),
            "--runtime-config-hash" => runtime_config_hash = args.next(),
            "--market-pressure-enabled" => {
                let raw = args.next().unwrap_or_else(|| usage());
                market_pressure_enabled = Some(parse_bool_arg(&raw))
            }
            "--input-json" => input_json = args.next().map(PathBuf::from),
            "--json-out" => json_out = args.next().map(PathBuf::from),
            "--md-out" => md_out = args.next().map(PathBuf::from),
            "-h" | "--help" => usage(),
            other => {
                eprintln!("unknown arg: {other}");
                usage();
            }
        }
    }

    let json_out = json_out.unwrap_or_else(|| usage());
    if derive_arm_json {
        let e2_json = e2_json.unwrap_or_else(|| usage());
        let e2_body = fs::read_to_string(&e2_json).unwrap_or_else(|e| {
            eprintln!("read {}: {e}", e2_json.display());
            std::process::exit(2);
        });
        let e2_report: E2CandidateVerifierReport =
            serde_json::from_str(&e2_body).unwrap_or_else(|e| {
                eprintln!("parse {}: {e}", e2_json.display());
                std::process::exit(2);
            });
        let arm = derive_arm_input_from_evidence(
            E4ArmEvidenceConfig {
                arm_id: arm_id.unwrap_or_else(|| usage()),
                evidence_dir: evidence_dir.unwrap_or_else(|| usage()),
                problem_set_hash: problem_set_hash.unwrap_or_else(|| usage()),
                model_assignment_hash: model_assignment_hash.unwrap_or_else(|| usage()),
                budget_hash: budget_hash.unwrap_or_else(|| usage()),
                prompt_template_hash: prompt_template_hash.unwrap_or_else(|| usage()),
                runtime_config_hash: runtime_config_hash.unwrap_or_else(|| usage()),
                market_pressure_enabled: market_pressure_enabled.unwrap_or_else(|| usage()),
            },
            e2_report,
        )
        .unwrap_or_else(|e| {
            eprintln!("derive arm input: {e}");
            std::process::exit(2);
        });
        let json = serde_json::to_string_pretty(&arm).unwrap_or_else(|e| {
            eprintln!("serialize arm input: {e}");
            std::process::exit(2);
        });
        fs::write(&json_out, format!("{json}\n")).unwrap_or_else(|e| {
            eprintln!("write {}: {e}", json_out.display());
            std::process::exit(2);
        });
        return;
    }

    let input_json = input_json.unwrap_or_else(|| usage());
    let md_out = md_out.unwrap_or_else(|| usage());

    let input_body = fs::read_to_string(&input_json).unwrap_or_else(|e| {
        eprintln!("read {}: {e}", input_json.display());
        std::process::exit(2);
    });
    let input: InputFile = serde_json::from_str(&input_body).unwrap_or_else(|e| {
        eprintln!("parse {}: {e}", input_json.display());
        std::process::exit(2);
    });
    let report = evaluate_market_performance_e4(&input.arms);
    let json = serde_json::to_string_pretty(&report).unwrap_or_else(|e| {
        eprintln!("serialize report: {e}");
        std::process::exit(2);
    });
    fs::write(&json_out, format!("{json}\n")).unwrap_or_else(|e| {
        eprintln!("write {}: {e}", json_out.display());
        std::process::exit(2);
    });
    fs::write(&md_out, report.render_markdown()).unwrap_or_else(|e| {
        eprintln!("write {}: {e}", md_out.display());
        std::process::exit(2);
    });

    if report.verdict == E4Verdict::Veto {
        std::process::exit(1);
    }
}

fn parse_bool_arg(raw: &str) -> bool {
    match raw {
        "true" | "1" | "yes" => true,
        "false" | "0" | "no" => false,
        other => {
            eprintln!("invalid bool argument: {other}");
            std::process::exit(2);
        }
    }
}

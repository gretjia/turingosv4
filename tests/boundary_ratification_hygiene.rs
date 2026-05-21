use std::fs;
use std::path::Path;

const BOUNDARY_DOC: &str = "docs/architecture/FC_REAL_WORLD_BOUNDARY.md";
const DIRECTIVE: &str = "handover/directives/2026-05-21_FC_BOUNDARY_RATIFICATION_DIRECTIVE.md";
const LATEST: &str = "handover/ai-direct/LATEST.md";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_has_all(haystack: &str, needles: &[&str]) {
    for needle in needles {
        assert!(
            haystack.contains(needle),
            "expected document to contain `{needle}`"
        );
    }
}

#[test]
fn boundary_doc_exists_and_keeps_v2_terms_only_in_forbidden_scope() {
    assert!(Path::new(BOUNDARY_DOC).exists(), "{BOUNDARY_DOC} missing");
    let doc = read(BOUNDARY_DOC);

    assert_has_all(
        &doc,
        &[
            "Class 0 fact record",
            "FC boundary facts",
            "ratification debt",
            "P7.z note",
            "Art. 0.4",
            "Hermetic",
            "Predicate locality",
            "LLM topology",
            "Out of scope / forbidden",
        ],
    );

    let marker = "## Out of scope / forbidden";
    let marker_index = doc
        .find(marker)
        .unwrap_or_else(|| panic!("{BOUNDARY_DOC} must contain `{marker}`"));
    let before_forbidden = &doc[..marker_index];
    let forbidden_terms = [
        "ProblemCapsule",
        "CandidatePatchBundle",
        "OracleSignature",
        "CooldownLock",
        "tos predicate",
        "atom_id",
        "schema",
        "CLI",
        "roadmap",
    ];

    for term in forbidden_terms {
        assert!(
            !before_forbidden.contains(term),
            "`{term}` appears before the explicit forbidden section"
        );
    }
}

#[test]
fn directive_answers_boundary_questions_without_implementation_authorization() {
    let directive = read(DIRECTIVE);

    assert_has_all(
        &directive,
        &[
            "Art. 0.4",
            "C-hybrid",
            "B-pragmatic",
            "Phase E full-B",
            "process hygiene only",
            "no OS-level no-network claim",
            "default subprocess",
            "not sequencer admission",
            "δ remains inside FC1",
            "proposal/evidence",
            "never accept predicate",
            "does NOT authorize",
            "sequencer",
            "typed_tx",
            "trust root",
            "canonical signing payload",
            "constitution changes",
        ],
    );
}

#[test]
fn boundary_and_latest_have_no_conflict_markers() {
    for path in [BOUNDARY_DOC, LATEST] {
        let text = read(path);
        for marker in ["<<<<<<<", "=======", ">>>>>>>"] {
            assert!(
                !text.contains(marker),
                "{path} contains unresolved conflict marker `{marker}`"
            );
        }
    }
}

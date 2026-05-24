use std::collections::BTreeMap;

use serde_json::Value;
use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::state::typed_tx::{
    BoolWithProof, PredicateId, PredicateResultsBundle, SafetyOrCreation,
};

#[test]
fn bool_with_proof_wire_shape_remains_value_and_proof_cid_only() {
    let value = BoolWithProof {
        value: true,
        proof_cid: Some(Cid::from_content(b"proof")),
    };
    let json = serde_json::to_value(value).expect("json");
    let obj = json.as_object().expect("object");
    let keys: Vec<_> = obj.keys().map(String::as_str).collect();
    assert_eq!(keys, vec!["proof_cid", "value"]);
}

#[test]
fn predicate_results_bundle_keeps_btreemap_and_safety_class_shape() {
    let mut acceptance = BTreeMap::new();
    acceptance.insert(
        PredicateId("p.accept".to_string()),
        BoolWithProof {
            value: true,
            proof_cid: None,
        },
    );
    let bundle = PredicateResultsBundle {
        acceptance,
        settlement: BTreeMap::new(),
        safety_class: SafetyOrCreation::Safety,
    };
    let json = serde_json::to_value(bundle).expect("json");
    let obj = json.as_object().expect("object");
    let keys: Vec<_> = obj.keys().map(String::as_str).collect();
    assert_eq!(keys, vec!["acceptance", "safety_class", "settlement"]);
    assert!(matches!(obj.get("acceptance"), Some(Value::Object(_))));
    assert!(matches!(obj.get("settlement"), Some(Value::Object(_))));
}

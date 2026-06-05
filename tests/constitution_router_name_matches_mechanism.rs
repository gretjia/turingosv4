fn mechanism_name_matches_body(name: &str, body: &str) -> Result<(), String> {
    let lower_name = name.to_ascii_lowercase();
    let lower_body = body.to_ascii_lowercase();
    let claims_softmax = lower_name.contains("softmax");
    let implements_softmax = lower_body.contains(".exp()") || lower_body.contains("exp(");
    let implements_argmax = lower_body.contains("argmax") || lower_body.contains("max_by");

    if claims_softmax && implements_argmax && !implements_softmax {
        return Err(format!("{name} claims softmax but body is argmax-like"));
    }
    Ok(())
}

fn function_window<'a>(source: &'a str, name: &str, lines: usize) -> String {
    source
        .lines()
        .skip_while(|line| !line.contains(name))
        .take(lines)
        .collect::<Vec<&'a str>>()
        .join("\n")
}

#[test]
fn positive_control_rejects_softmax_label_over_argmax_body() {
    let err = mechanism_name_matches_body(
        "softmax_parent_router",
        "fn route() { /* argmax */ candidates.max_by(score) }",
    )
    .expect_err("softmax label over argmax body must fail");
    assert!(err.contains("claims softmax"));
}

#[test]
fn boltzmann_v2_name_discloses_argmax_epsilon_mechanism() {
    let src = std::fs::read_to_string("src/sdk/actor.rs").expect("actor.rs readable");
    let window = function_window(&src, "pub fn boltzmann_select_parent_v2", 90);

    assert!(
        window.to_ascii_lowercase().contains("argmax")
            && window.to_ascii_lowercase().contains("epsilon"),
        "boltzmann_select_parent_v2 must disclose argmax + epsilon mechanism"
    );
    mechanism_name_matches_body("boltzmann_select_parent_v2", &window)
        .expect("v2 name must not claim softmax");
}

#[test]
fn softmax_router_name_has_softmax_body() {
    let src = std::fs::read_to_string("src/sdk/actor.rs").expect("actor.rs readable");
    let window = function_window(&src, "pub fn boltzmann_softmax_select_parent", 80);

    assert!(
        window.contains(".exp()"),
        "softmax-named selector must actually use exponential weights"
    );
    mechanism_name_matches_body("boltzmann_softmax_select_parent", &window)
        .expect("softmax selector name must match body");
}

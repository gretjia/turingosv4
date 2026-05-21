use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=frontend/dist/main.js");

    if std::env::var("CARGO_FEATURE_WEB").is_ok() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR env var not set");
        let main_js = Path::new(&manifest_dir)
            .join("frontend")
            .join("dist")
            .join("main.js");

        if !main_js.exists() {
            eprintln!("run: cd frontend && npm ci && npm run build");
            std::process::exit(1);
        }
    }
}

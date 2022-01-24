use prost_build_config::{BuildConfig, Builder};
use std::process::Command;
use std::{fs, option_env};

fn main() {
    let build_enabled = option_env!("BUILD_PROTO")
        .map(|v| v == "1")
        .unwrap_or(false);

    if !build_enabled {
        println!("=== Skipped compiling protos ===");
        return;
    }

    let config: BuildConfig = serde_yaml::from_str(include_str!("build_opts.yml")).unwrap();
    Builder::from(config).build_protos();
    fs::rename("src/pb/abi.rs", "src/pb/gen.rs")
        .unwrap_or_else(|e| panic!("Failed to move proto files. Error: {:?}", e));
    Command::new("cargo")
        .args(&["fmt", "--", "src/pb/gen.rs"])
        .status()
        .expect("cargo fmt failed");
}

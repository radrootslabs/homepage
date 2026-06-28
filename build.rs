use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("cargo manifest dir should exist"));
    let config_path = manifest_dir.join("locales").join("mf2_i18n.toml");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("out dir should exist"));

    let build_output = mf2_i18n::build_leptos_target(
        &mf2_i18n::LeptosTargetBuildOptions::new(&config_path, "app").with_out_dir(&out_dir),
    )
    .unwrap_or_else(|error| {
        panic!(
            "failed to build radroots homepage leptos i18n from {}: {error}",
            config_path.display()
        )
    });

    for path in build_output.native_output().rerun_if_changed_paths() {
        println!("cargo:rerun-if-changed={}", path.display());
    }
}

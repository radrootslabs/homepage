use std::env;
use std::path::PathBuf;

const HOMEPAGE_ENV_VARS: &[&str] = &[
    "RADROOTS_GIT_URL",
    "RADROOTS_IOS_URL",
    "RADROOTS_ANDROID_URL",
    "RADROOTS_DESKTOP_URL",
    "RADROOTS_CLI_URL",
    "RADROOTS_SUPPORT_CONTACT_URL",
];

fn main() {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("cargo manifest dir should exist"));
    let env_path = manifest_dir.join(".env");
    let config_path = manifest_dir.join("locales").join("mf2_i18n.toml");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("out dir should exist"));

    println!("cargo:rerun-if-changed={}", env_path.display());
    for key in HOMEPAGE_ENV_VARS {
        println!("cargo:rerun-if-env-changed={key}");
    }

    if let Err(error) = dotenvy::from_path(&env_path)
        && !error.not_found()
    {
        panic!("failed to load {}: {error}", env_path.display());
    }

    for key in HOMEPAGE_ENV_VARS {
        let value = env::var(key).unwrap_or_else(|error| panic!("{key} must be set: {error}"));
        if value.trim().is_empty() {
            panic!("{key} must not be empty");
        }
        println!("cargo:rustc-env={key}={value}");
    }

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

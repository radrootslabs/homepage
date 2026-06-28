use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const RADROOTS_GIT_URL_ENV: &str = "RADROOTS_GIT_URL";

fn main() {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("cargo manifest dir should exist"));
    let env_path = manifest_dir.join(".env");
    let config_path = manifest_dir.join("locales").join("mf2_i18n.toml");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("out dir should exist"));
    let radroots_git_url = required_dotenv_value(&env_path, RADROOTS_GIT_URL_ENV);

    println!("cargo:rerun-if-changed={}", env_path.display());
    println!("cargo:rustc-env={RADROOTS_GIT_URL_ENV}={radroots_git_url}");

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

fn required_dotenv_value(path: &Path, key: &str) -> String {
    let contents = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));

    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let Some((raw_key, raw_value)) = line.split_once('=') else {
            panic!("invalid .env entry in {}: {raw_line}", path.display());
        };

        if raw_key.trim() == key {
            let value = unquote_dotenv_value(raw_value.trim());
            if value.is_empty() {
                panic!("{key} in {} must not be empty", path.display());
            }
            return value.to_owned();
        }
    }

    panic!("missing {key} in {}", path.display());
}

fn unquote_dotenv_value(value: &str) -> &str {
    if let Some(value) = value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
    {
        return value;
    }

    if let Some(value) = value
        .strip_prefix('\'')
        .and_then(|value| value.strip_suffix('\''))
    {
        return value;
    }

    value
}

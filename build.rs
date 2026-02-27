use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
struct GitmojisRoot {
    gitmojis: Vec<Gitmoji>,
}

#[derive(Deserialize)]
struct Gitmoji {
    emoji: String,
    entity: String,
    code: String,
    description: String,
    name: String,
    semver: Option<String>,
}

#[derive(Deserialize)]
struct ConventionalCommitTypesRoot {
    types: std::collections::BTreeMap<String, CommitType>,
}

#[derive(Deserialize)]
struct CommitType {
    description: String,
    title: String,
}

fn escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn fetch(url: &str) -> String {
    ureq::get(url)
        .call()
        .unwrap_or_else(|e| panic!("failed to fetch {url}: {e}"))
        .body_mut()
        .read_to_string()
        .unwrap_or_else(|e| panic!("failed to read body from {url}: {e}"))
}

fn main() {
    println!("cargo:rerun-if-changed=.env.build");
    println!("cargo:rerun-if-changed=build.rs");

    // Load versioned URLs from .env.build.
    dotenvy::from_filename(".env.build").expect(".env.build not found");
    let gitmojis_url = std::env::var("GITMOJIS_URL").expect("GITMOJIS_URL not set in .env.build");
    let cct_url = std::env::var("CONVENTIONAL_COMMIT_TYPES_URL")
        .expect("CONVENTIONAL_COMMIT_TYPES_URL not set in .env.build");

    let gitmojis: GitmojisRoot =
        serde_json::from_str(&fetch(&gitmojis_url)).expect("failed to parse gitmojis.json");
    let cct: ConventionalCommitTypesRoot = serde_json::from_str(&fetch(&cct_url))
        .expect("failed to parse conventional-commit-types index.json");

    let mut out = String::from(concat!(
        "// @generated — do not edit by hand. Source URLs are in .env.build.\n",
        "\n",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n",
        "pub enum Semver { Major, Minor, Patch }\n",
        "\n",
        "#[derive(Debug, Clone, Copy)]\n",
        "pub struct Gitmoji {\n",
        "    pub emoji: &'static str,\n",
        "    pub entity: &'static str,\n",
        "    pub code: &'static str,\n",
        "    pub description: &'static str,\n",
        "    pub name: &'static str,\n",
        "    pub semver: Option<Semver>,\n",
        "}\n",
        "\n",
        "pub static GITMOJIS: &[Gitmoji] = &[\n",
    ));

    for g in &gitmojis.gitmojis {
        let semver = match g.semver.as_deref() {
            Some("major") => "Some(Semver::Major)",
            Some("minor") => "Some(Semver::Minor)",
            Some("patch") => "Some(Semver::Patch)",
            _ => "None",
        };
        out.push_str(&format!(
            "    Gitmoji {{ emoji: \"{emoji}\", entity: \"{entity}\", code: \"{code}\", description: \"{description}\", name: \"{name}\", semver: {semver} }},\n",
            emoji = escape(&g.emoji),
            entity = escape(&g.entity),
            code = escape(&g.code),
            description = escape(&g.description),
            name = escape(&g.name),
        ));
    }

    out.push_str(concat!(
        "];\n",
        "\n",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n",
        "pub struct CommitType {\n",
        "    pub key: &'static str,\n",
        "    pub title: &'static str,\n",
        "    pub description: &'static str,\n",
        "}\n",
        "\n",
        "pub static COMMIT_TYPES: &[CommitType] = &[\n",
    ));

    for (key, ct) in &cct.types {
        out.push_str(&format!(
            "    CommitType {{ key: \"{key}\", title: \"{title}\", description: \"{description}\" }},\n",
            title = escape(&ct.title),
            description = escape(&ct.description),
        ));
    }

    out.push_str("];\n");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("generated.rs");
    std::fs::write(&out_path, out).expect("failed to write generated.rs");
}

#[cfg(not(feature = "cc_build"))]
use std::process::Command;
use std::{collections::HashSet, env, path::{Path, PathBuf}};



#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

#[derive(Debug)]
struct DeriveMacros(Vec<String>);

impl bindgen::callbacks::ParseCallbacks for DeriveMacros {
    fn add_derives(&self, _info: &bindgen::callbacks::DeriveInfo<'_>) -> Vec<String> {
        match _info.kind {
            bindgen::callbacks::TypeKind::Struct => Vec::new(),
            bindgen::callbacks::TypeKind::Enum => self.0.clone(),
            bindgen::callbacks::TypeKind::Union => Vec::new(),
        }
    }
}

fn main() {
    #[cfg(not(feature = "cc_build"))]
    build_with_make();

    #[cfg(feature = "cc_build")]
    build_with_cc();

    println!("cargo:rustc-link-search={}/", env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-lib=static=cubiomes");

    let ignored_macros = IgnoreMacros(
        vec![
            "FP_INFINITE".into(),
            "FP_NAN".into(),
            "FP_NORMAL".into(),
            "FP_SUBNORMAL".into(),
            "FP_ZERO".into(),
        ]
        .into_iter()
        .collect(),
    );

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(ignored_macros))
        .newtype_enum(".*")
        .layout_tests(true)
        .generate()
        .expect("Unable to generate binding for cubiomes");

    // Generates rustified enums for use in a wrapper library
    let biome_enum_bindings = bindgen::Builder::default()
        .header("enum_wrapper.h")
        .parse_callbacks(Box::new(DeriveMacros(vec![
            "FromPrimitive".into(),
            "ToPrimitive".into(),
            "PartialOrd".into(),
            "Ord".into(),
        ])))
        .blocklist_function(".*") //Blocks all functions, as we are only intrested in the enums
        .allowlist_type(["BiomeID", "Dimension", "MCVersion", "StructureType"].join("|"))
        .rustified_non_exhaustive_enum(".*")
        .generate()
        .expect("Unable to generate rustified enums for cubiomes");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");

    biome_enum_bindings
        .write_to_file(out_path.join("biome_enums.rs"))
        .expect("Couldn't write biome enums");
}

#[cfg(not(feature = "cc_build"))]
fn build_with_make() {
    if !Command::new("make")
        .arg("-C")
        .arg("cubiomes/")
        .status()
        .expect("Failed to build cubiomes")
        .success()
    {
        panic!("Make did not return 0")
    }

    if !Command::new("mv")
        .arg("cubiomes/libcubiomes.a")
        .arg(env::var("OUT_DIR").unwrap())
        .status()
        .expect("Failed to move libcubiomes")
        .success()
    {
        panic!("mv did not return 0")
    }
}

#[cfg(feature = "cc_build")]
fn build_with_cc() {
    let mut build = cc::Build::new();
    for f in collect_sources() {
        build.file(f);
    }
    build.include("cubiomes");
    // Variable-length arrays and variably-modified pointer types are standard
    // C99 that MSVC has never implemented; clang accepts them, and the CI
    // workflow points CC at clang-cl on Windows for exactly this reason.
    build.flag_if_supported("-Wno-vla-extension");
    build.compile("cubiomes");
}

/// Every `.c` file in the vendored tree except the test harness.
///
/// The sources are no longer a flat list: the fork keeps structure-piece code
/// under `features/` and loot tables under `loot/`, roughly seventy files in
/// all. Compiling only the root-level ones still links on macOS and Linux —
/// their linkers dead-strip the unreferenced callers — but fails on Windows
/// with unresolved externals, because MSVC resolves every symbol in an object
/// it pulls in. Walking the tree keeps this correct as upstream adds files.
fn collect_sources() -> Vec<PathBuf> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|e| e == "c")
                && path.file_name().is_some_and(|n| n != "tests.c")
            {
                out.push(path);
            }
        }
    }

    let mut out = Vec::new();
    walk(Path::new("cubiomes"), &mut out);
    // Deterministic order, so the build is reproducible.
    out.sort();
    assert!(
        out.len() > 8,
        "found only {} C sources; the vendored tree looks wrong",
        out.len()
    );
    out
}

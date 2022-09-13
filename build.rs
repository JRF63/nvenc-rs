use bindgen::callbacks::ParseCallbacks;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

#[derive(Debug)]
struct CustomParseCallback {}

impl ParseCallbacks for CustomParseCallback {
    fn add_derives(&self, name: &str) -> Vec<String> {
        if name == "GUID" {
            vec!["PartialEq".to_string(), "Eq".to_string()]
        } else {
            vec![]
        }
    }
}

fn generate_bindings(version: &str, filename: &str, out_dir: &PathBuf) {
    let bindings = bindgen::Builder::default()
        .header(filename)
        .parse_callbacks(Box::new(CustomParseCallback {}))
        .layout_tests(false)
        .derive_debug(true)
        .generate_comments(false)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
        .must_use_type("_NVENCSTATUS")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join(&format!("nvenc_{}.rs", version)))
        .expect("Could not write bindings");
}

/// Manually generates the struct version macros that are otherwise skipped by bindgen.
fn generate_struct_versions(
    version: &str,
    filename: &str,
    out_dir: &PathBuf,
) -> std::io::Result<()> {
    lazy_static! {
        static ref RE: Regex = Regex::new("#define (NV_.+VER) (.+)").unwrap();
    }

    let mut struct_versions =
        File::create(out_dir.join(&format!("nvenc_{}_struct_versions.rs", version)))?;
    let header = File::open(&filename)?;
    let reader = BufReader::new(header);
    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(caps) = RE.captures(&line) {
                writeln!(
                    &mut struct_versions,
                    "pub const {}: u32 = {};",
                    caps.get(1).unwrap().as_str(),
                    caps.get(2).unwrap().as_str()
                )?;
            }
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=headers");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let versions = ["v9_0", "v9_1", "v10_0", "v11_1"];
    for version in versions {
        if let Ok(_) = env::var(&format!("CARGO_FEATURE_{}", version.to_uppercase())) {
            let path = PathBuf::from(format!("headers/{}/nvEncodeAPI.h", version));
            if let Ok(canonical_path) = path.canonicalize() {
                if let Ok(filename) = canonical_path.into_os_string().into_string() {
                    println!("cargo:nvenc_{}={}", version, filename);
                    generate_bindings(version, &filename, &out_dir);
                    generate_struct_versions(version, &filename, &out_dir)?;
                }
            }
        }
    }

    Ok(())
}

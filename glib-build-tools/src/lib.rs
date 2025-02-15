// Take a look at the license at the top of the repository in the LICENSE file.

#![doc = include_str!("../README.md")]

use std::{env, path::Path, process::Command};

// rustdoc-stripper-ignore-next
/// Call to run `glib-compile-resources` to generate compiled gresources to embed
/// in binary with [`gio::resources_register_include`]. `target` is relative to `OUT_DIR`.
///
/// ```no_run
/// glib_build_tools::compile_resources(
///     "resources",
///     "resources/resources.gresource.xml",
///     "compiled.gresource",
/// );
/// ```
pub fn compile_resources<P: AsRef<Path>>(source_dir: P, gresource: &str, target: &str) {
    let out_dir = env::var("OUT_DIR").unwrap();

    let status = Command::new("glib-compile-resources")
        .arg("--sourcedir")
        .arg(source_dir.as_ref())
        .arg("--target")
        .arg(&format!("{out_dir}/{target}"))
        .arg(gresource)
        .status()
        .unwrap();

    assert!(
        status.success(),
        "glib-compile-resources failed with exit status {status}",
    );

    println!("cargo:rerun-if-changed={gresource}");
    let output = Command::new("glib-compile-resources")
        .arg("--sourcedir")
        .arg(source_dir.as_ref())
        .arg("--generate-dependencies")
        .arg(gresource)
        .output()
        .unwrap()
        .stdout;
    let output = String::from_utf8(output).unwrap();
    for dep in output.split_whitespace() {
        println!("cargo:rerun-if-changed={dep}");
    }
}

fn main() {
    pkg_config::Config::new()
        .atleast_version("2.0.2")
        .probe("ldacBT-enc")
        .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}

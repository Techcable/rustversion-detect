pub fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let version = rustversion_detect::RUST_VERSION;

    if version.is_since(rustversion_detect::spec!(1.80)) {
        println!("cargo:rustc-cfg=since_180")
    }

    if version.is_nightly() {
        println!("cargo:rustc-cfg=is_nightly")
    }

    println!("cargo:rustc-check-cfg=cfg(since_180)");
    println!("cargo:rustc-check-cfg=cfg(is_nightly)");
}
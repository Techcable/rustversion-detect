pub fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    if rustversion::cfg!(since(1.80)) {
        println!("cargo:rustc-cfg=since_180")
    }

    if rustversion::cfg!(nightly) {
        println!("cargo:rustc-cfg=is_nightly")
    }

    println!("cargo:rustc-check-cfg=cfg(since_180)");
    println!("cargo:rustc-check-cfg=cfg(is_nightly)");
}
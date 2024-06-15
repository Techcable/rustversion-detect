fn main() {
    if cfg!(since_180) {
        println!("Rust since 1.80");
    }
    if cfg!(is_nightly) {
        println!("Rust nightly");
    }
}

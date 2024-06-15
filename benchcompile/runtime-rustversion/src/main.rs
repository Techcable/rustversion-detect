fn main() {
    if rustversion::cfg!(since(1.80)) {
        println!("Rust since 1.80");
    }
    if rustversion::cfg!(nightly) {
        println!("Rust nightly");
    }
}

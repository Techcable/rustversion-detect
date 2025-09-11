pub fn main() {
    let version = rustversion_detect::detect_version().unwrap();
    println!("version: {}", version);
}

pub fn hello() -> String {
    "Hello Rust".to_string()
}

pub fn greet(name: &str) -> String {
    format!("Hello {}", name)
}

pub fn append(mut s: String) -> String {
    // format!("{}!", s) this gives warning because a mutable s is not mutated ever
    s += "!";
    s
}

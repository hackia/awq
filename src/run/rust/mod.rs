pub const RUST_TEST: &str = "cargo test -- --show-output";
pub const RUST_AUDIT: &str = "cargo audit";
pub const RUST_FORMAT: &str = "cargo fmt -- --check";
pub const RUST_BUILD: &str = "cargo build";
pub const RUST_FULL_LINT: &str = "cargo clippy -- -W clippy::all -W clippy::cargo";

pub const RUN_RUST: [(&str, &str, &str); 5] = [
    ("build", "compiling source code", RUST_BUILD),
    ("fmt", "checking source code format", RUST_FORMAT),
    ("lint", "checking source code", RUST_FULL_LINT),
    ("audit", "auditing source code ", RUST_AUDIT),
    ("test", "testing source code ", RUST_TEST),
];

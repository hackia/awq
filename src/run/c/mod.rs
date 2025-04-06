pub const C_BUILD: &str = "make";
pub const C_FORMAT: &str = "make fmt";
pub const C_LINT: &str = "make lint";
pub const C_TEST: &str = "make test";

pub const RUN_C: [(&str, &str, &str); 4] = [
    ("build", "compiling C source", C_BUILD),
    ("fmt", "checking code format", C_FORMAT),
    ("lint", "static analysis", C_LINT),
    ("test", "running tests", C_TEST),
];

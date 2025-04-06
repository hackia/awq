pub const CMAKE_BUILD: &str = "cmake . && make";
pub const CMAKE_FORMAT: &str = "cmake fmt";
pub const CMAKE_LINT: &str = "cmake lint";
pub const CMAKE_AUDIT: &str = "cmake audit";
pub const CMAKE_TEST: &str = "cmake test";

pub const RUN_CMAKE: [(&str, &str, &str); 4] = [
    ("build", "building project with CMake", CMAKE_BUILD),
    ("fmt", "checking code format", CMAKE_FORMAT),
    ("lint", "static analysis", CMAKE_LINT),
    ("test", "running tests with CTest", CMAKE_TEST),
];

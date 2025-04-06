pub const CMAKE_CONFIGURE: &str = "cmake -S . -B build";
pub const CMAKE_BUILD: &str = "cmake --build build";
pub const CMAKE_FORMAT: &str = "clang-format --dry-run --Werror $(find . -name '*.c' -o -name '*.cpp' -o -name '*.h' -o -name '*.hpp')";
pub const CMAKE_LINT: &str = "cppcheck --enable=all --inconclusive --error-exitcode=1 .";
pub const CMAKE_AUDIT: &str = "echo 'Security audit to be performed manually'";
pub const CMAKE_TEST: &str = "ctest --test-dir build";
pub const RUN_CMAKE: [(&str, &str, &str); 5] = [
    (
        "configure",
        "configuring project with CMake",
        CMAKE_CONFIGURE,
    ),
    ("build", "building project with CMake", CMAKE_BUILD),
    ("fmt", "checking code format", CMAKE_FORMAT),
    ("lint", "static analysis", CMAKE_LINT),
    ("test", "running tests with CTest", CMAKE_TEST),
];

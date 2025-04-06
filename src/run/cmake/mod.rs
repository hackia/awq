/// Configure le projet avec CMake
pub const CMAKE_CONFIGURE: &str = "cmake -S . -B build";

/// Compile le projet avec CMake
pub const CMAKE_BUILD: &str = "cmake --build build";

/// Vérifie la mise en forme avec clang-format (sans modifier)
pub const CMAKE_FORMAT: &str = "clang-format --dry-run --Werror $(find . -name '*.c' -o -name '*.cpp' -o -name '*.h' -o -name '*.hpp')";

/// Analyse statique du code avec cppcheck
pub const CMAKE_LINT: &str = "cppcheck --enable=all --inconclusive --error-exitcode=1 .";

/// Audit (manuel ou via script)
pub const CMAKE_AUDIT: &str = "echo 'Security audit to be performed manually'";

/// Lance les tests avec CTest (intégré à CMake)
pub const CMAKE_TEST: &str = "ctest --test-dir build";

/// Commandes standards pour projet C/C++ avec CMake
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

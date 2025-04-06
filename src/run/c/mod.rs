/// Compile le projet C avec gcc
pub const C_BUILD: &str = "gcc -o main main.c";

/// Vérifie la mise en forme avec clang-format (sans modifier)
pub const C_FORMAT: &str = "clang-format --dry-run --Werror *.c *.h";

/// Analyse statique du code avec cppcheck (niveau strict)
pub const C_LINT: &str = "cppcheck --enable=all --inconclusive --error-exitcode=1 .";

/// Audit de sécurité (simulé, souvent manuellement pour C)
pub const C_AUDIT: &str = "echo 'Audit manuel recommandé pour le C'";

/// Lance les tests avec CTest ou un script custom
pub const C_TEST: &str = "make test"; // ou ./run_tests.sh

/// Commandes standards pour un projet C
pub const RUN_C: [(&str, &str, &str); 5] = [
    ("build", "compiling C source", C_BUILD),
    ("fmt", "checking code format", C_FORMAT),
    ("lint", "static analysis", C_LINT),
    ("audit", "security audit", C_AUDIT),
    ("test", "running tests", C_TEST),
];

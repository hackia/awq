/// Exécute tous les tests avec affichage des sorties
pub const RUST_TEST: &str = "test -- --show-output";

/// Audit de sécurité des dépendances (avec cargo-audit)
pub const RUST_AUDIT: &str = "audit";

/// Vérifie la mise en forme du code (sans modifier)
pub const RUST_FORMAT: &str = "fmt -- --check";
pub const RUST_BUILD: &str = "build";

/// Vérifie les dépendances obsolètes
pub const RUST_DEPS_OUTDATED: &str = "outdated";

/// Combine plusieurs lints en une passe stricte
pub const RUST_FULL_LINT: &str = "clippy -- --warn clippy::all \
                                    --warn clippy::nursery \
                                    --warn clippy::pedantic \
                                    --warn clippy::suspicious \
                                    --deny warnings \
                                    --deny clippy::complexity";

pub const RUN_RUST: [(&str, &str, &str); 5] = [
    ("build", "compiling source code", RUST_BUILD),
    ("fmt", "checking source code format", RUST_FORMAT),
    ("lint", "checking source code", RUST_FULL_LINT),
    ("audit", "auditing source code ", RUST_AUDIT),
    ("test", "testing source code ", RUST_TEST),
];

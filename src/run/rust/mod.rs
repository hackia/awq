/// Exécute tous les tests avec affichage des sorties
pub const RUST_TEST: &str = "test -- --show-output";

/// Audit de sécurité des dépendances (avec cargo-audit)
pub const RUST_AUDIT: &str = "audit";

/// Vérifie la mise en forme du code (sans modifier)
pub const RUST_FORMAT: &str = "fmt -- --check";

/// Vérifie les dépendances obsolètes
pub const RUST_DEPS_OUTDATED: &str = "outdated";

/// Vérifie les dépendances inutilisées
pub const RUST_DEPS_UDEPS: &str = "udeps";

/// Combine plusieurs lints en une passe stricte
pub const RUST_FULL_LINT: &str = "clippy -- --warn clippy::all \
                                            --warn clippy::nursery \
                                            --deny warnings \
                                            --deny clippy::complexity";
pub const RUN_RUST: [&str; 4] = [RUST_FORMAT, RUST_FULL_LINT, RUST_AUDIT, RUST_TEST];

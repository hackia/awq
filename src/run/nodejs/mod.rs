/// Compile le projet TypeScript ou JavaScript
pub const NPM_BUILD: &str = "run build";

/// Vérifie la mise en forme du code (sans modifier)
pub const NPM_FORMAT: &str = "run fmt";

/// Lint strict du code avec ESLint
pub const NPM_LINT: &str = "run lint";

/// Audit de sécurité des dépendances
pub const NPM_AUDIT: &str = "audit";

/// Exécute tous les tests
pub const NPM_TEST: &str = "test";

pub const RUN_NPM: [(&str, &str, &str); 5] = [
    ("build", "compiling project", NPM_BUILD),
    ("fmt", "checking code format", NPM_FORMAT),
    ("lint", "linting code", NPM_LINT),
    ("audit", "auditing dependencies", NPM_AUDIT),
    ("test", "running tests", NPM_TEST),
];

pub const NPM_BUILD: &str = "npm run build";
pub const NPM_FORMAT: &str = "npm run fmt";
pub const NPM_LINT: &str = "npm run lint";

pub const NPM_AUDIT: &str = "npm audit";
pub const NPM_TEST: &str = "npm test";
pub const RUN_NPM: [(&str, &str, &str); 5] = [
    ("build", "compiling project", NPM_BUILD),
    ("fmt", "checking code format", NPM_FORMAT),
    ("lint", "linting code", NPM_LINT),
    ("audit", "auditing dependencies", NPM_AUDIT),
    ("test", "running tests", NPM_TEST),
];

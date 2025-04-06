pub const GO_BUILD: &str = "go build";

pub const GO_FORMAT: &str = "gofmt -s -w .";

pub const GO_LINT: &str = "golangci-lint run .";

pub const GO_AUDIT: &str = "go list -m all | go mod verify";

pub const GO_TEST: &str = "go test -v";

/// Commandes standardisées pour un projet Go
pub const RUN_GO: [(&str, &str, &str); 5] = [
    ("build", "compile the Go code", GO_BUILD),
    ("fmt", "check and format the code", GO_FORMAT),
    ("lint", "perform static code analysis", GO_LINT),
    ("audit", "audit and verify dependencies", GO_AUDIT),
    ("test", "execute the test suite", GO_TEST),
];

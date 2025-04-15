# awq

This is an example of a `README.md` file that clearly explains how to use the `awq` system for various programming
languages (`Rust`, `JavaScript`, `C`, `CMake`). It outlines the prerequisites such as the `Makefile`, scripts in
`package.json`, or custom commands for `CMake`, as well as the need to create an `awq.yml` file:

This project provides a unified abstraction for running build, test, lint, formatting, and audit commands across
different programming languages using a common configuration system.

## Concept



Each language is associated with a set of standardized commands grouped into a `RUN_XXX` table. These commands can then
be executed using the `awq` tool.

## Minimum Configuration Requirements

### Rust

No special configuration is required. The commands are directly compatible with `cargo`.

### JavaScript / TypeScript

- Requires a `package.json` file.
- The following scripts must be defined in `package.json`:

```json
"scripts": {
"build": "tsc",
"fmt": "prettier --check .",
"lint": "eslint .",
"audit": "npm audit",
"test": "npm test"
}
```

### C

- Requires a `Makefile` containing the following targets:

```bash
make         # for building
make fmt     # for formatting
make lint    # for linting
make test    # for testing
```

### CMake

- Requires a custom configuration based on shell commands:
    - `cmake -S . -B build` for configuration
    - `cmake --build build` for compilation
    - `ctest --test-dir build` for testing
    - `clang-format` and `cppcheck` for formatting and linting

It is recommended to have a well-structured `CMakeLists.txt` file.

## Configuration File `awq.yml`

You must create an `awq.yml` file at the root of your project with the following syntax:

```yaml
language: [ "rust" ] # or "js", "c", "cmake"
```

This file specifies which set of commands `awq` should load.

## Example Usage

```bash
awq 
```

## Supported Languages

- Rust
- JavaScript / TypeScript (via npm)
- C (via Makefile)
- CMake

## Upcoming Features

- Support for Python, Go, and Bash
- Automatic generation of the `awq.yml` file
- Automatic language detection using heuristics

**awq** – *Unify your workflows. Simplify your development process.*

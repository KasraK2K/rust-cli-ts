# TechSuite CLI

A small Rust-powered scaffolding tool that generates the TypeScript plumbing (routes, controllers, services, logic, repositories, interfaces, and validation schemas) used across TechSuite projects. It applies consistent naming conventions and writes the files for you, so you can concentrate on the business logic instead of boilerplate.

## Highlights
- Generates a complete feature stack (`all`) or individual building blocks on demand
- Supports Express-style routing, service/logic layers, repositories, and validation
- Automatically injects `PascalCase`, `camelCase`, `snake_case`, and lower-case variants of the provided feature name
- Writes files into the requested directory, creating nested folders (e.g., `logic/`, `validation/`) as needed

## Prerequisites
- Rust toolchain (edition 2024). Install via [rustup](https://rustup.rs/) if you do not already have cargo available.

## Building / Installing
```bash
# Run in debug mode
cargo run -- <kind> <name> [path]

# Build a release binary (placed in target/release/)
cargo build --release

# Or install locally so it is available on your PATH
cargo install --path .
```

## Usage
```bash
techsuite-cli <kind> <name> [path]
```
- `kind`: what you want to scaffold. Supported values: `all`, `routes`, `controller`, `service`, `logic`, `repository`, `interface`, `schema`.
- `name`: feature name used to fill file names and template placeholders. The CLI derives several case styles (Pascal, camel, snake, lower) automatically.
- `path` *(optional)*: base directory for the generated files. Defaults to the current working directory.

### Example
```bash
# Generate an entire feature stack for "User" inside ./features/users
techsuite-cli all User ./src/modules/users
```
The command above creates files similar to:
```
features/users/
├── User.routes.ts
├── User.controller.ts
├── User.service.ts
├── logic/
│   └── User.logic.ts
├── User.repository.ts
├── User.interface.ts
└── validation/
    └── schema.ts
```

## Templates
The scaffolding logic lives in `src/statics/*.txt`. Each template file contains placeholders such as `{{PascalName}}`, `{{camelName}}`, `{{snakeCase}}`, and `{{lowerName}}`. Feel free to edit these templates to match your project's conventions—the CLI simply interpolates the placeholders and writes the result.

## Development
- Update or add templates in `src/statics/` as requirements evolve.
- Extend `Flag` and `get_template_files` in `src/lib.rs` if you introduce new generators.
- Run `cargo fmt && cargo clippy` before contributing to keep the codebase tidy.

# Repository Guidelines

## Project Structure & Module Organization

CCProxy is a Tauri v2 app with a Vue 3/Vite renderer and Rust backend.

- `src/`: renderer code. Pages live in `views/`, reusable UI in `components/`, Pinia stores in `stores/`, and Tauri IPC in `api/tauriBridge.js`.
- `src-tauri/src/`: Rust core. `proxy/` routes requests and upstream calls, `convert/` handles protocol/stream conversion, and `clients/` manages client configuration.
- `src-tauri/tests/`: Rust integration tests. Unit tests sit beside source in `#[cfg(test)]` modules.
- `resources/`, `src-tauri/icons/`: icons; `docs/`: designs and specifications; `scripts/`: macOS build helpers.
- Do not commit `node_modules/`, `dist/`, or `src-tauri/target/`.

## Build, Test, and Development Commands

Use Node.js >=21 and Rust.

- `npm install` installs dependencies.
- `npm run tauri:dev` starts Vite and the Tauri shell with hot reload.
- `npm test` runs all Rust unit and integration tests.
- `cargo test --manifest-path src-tauri/Cargo.toml <test_name>` runs one test.
- `npm run build` builds renderer assets into `dist/`.
- `npm run tauri:build` packages the app; CI builds macOS arm64 and Windows x64.
- `npm run build:win` builds the Windows NSIS installer.
- `./scripts/rebuild.sh --only-build` builds only the frontend on macOS.

## Coding Style & Naming Conventions

- Use two-space indentation, single quotes, no semicolons, `<script setup>`, and the `@/` import alias in Vue/JavaScript. Use `PascalCase.vue` for components, `useX.js` for stores/composables, and `camelCase.js` for utilities.
- Rust follows rustfmt: `snake_case` modules/functions and `PascalCase` types. Run `cargo fmt --manifest-path src-tauri/Cargo.toml` before committing.
- Keep protocol behavior in Rust `convert/` and `proxy/`; expose it through `api/tauriBridge.js` rather than duplicating logic in Vue.
- No frontend linter is configured; match adjacent files.

## Testing Guidelines

- Tests use Rust's built-in harness; async proxy tests use `tokio::test`.
- Name tests by observable behavior, e.g. `first_matching_mapping_wins`.
- Add integration coverage in `src-tauri/tests/<area>.rs`; keep narrow unit tests inline.
- Run focused tests, then `npm test` before submitting. Coverage is not configured.

## Commit & Pull Request Guidelines

- Follow the existing Conventional Commit style: `feat:`, `fix:`, `docs:`, `refactor:`, `ci:`, or `chore:`, followed by a concise Chinese summary, such as `fix: 托盘左键直接打开界面`.
- Keep commits focused. PRs should explain behavior and impact, list verification commands, link related issues, and include screenshots for UI changes.

## Configuration & Security

Runtime data is under `~/.ccproxy/`; logs use the platform temp directory. Never commit provider tokens, API keys, or machine-specific client configuration. Update the relevant design document when changing protocol mapping, conversion semantics, or persisted configuration.

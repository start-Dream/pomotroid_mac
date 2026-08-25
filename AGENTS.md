# OpenCode Agent Instructions

This repository is a Pomodoro timer desktop app built with **Tauri 2 + Rust** (backend) and **SvelteKit + Svelte 5** (frontend). 

## 🛠️ Commands & Workflows
- **Development:** `npm run tauri dev` (runs Vite dev server + Tauri).
- **Frontend only:** `npm run dev` (no Tauri IPC available).
- **Type-check:** `npm run check` (Svelte + TypeScript).
- **Formatting:** `npm run format`.
- **Localization:** `npm run paraglide:compile` MUST be run after modifying message files in `project.inlang/`.
- **Tests:** There are **no automated tests** in this codebase. Do not look for or attempt to run test suites.

## 🏗️ Architecture & Boundaries
- **IPC Layer:** All frontend↔backend communication must go through `src/lib/ipc/index.ts`. **NEVER** call Tauri's `invoke()` directly in Svelte components.
- **Rust Commands:** Live in `src-tauri/src/commands.rs` and all return `Result<T, String>`.
- **Types Contract:** `src/lib/types.ts` mirrors Rust structs (snake_case field names). Update this file if `Settings` or `TimerSnapshot` structs change in Rust.
- **Tauri Permissions:** Any new IPC command or API plugin used from the frontend must be allowlisted in `src-tauri/capabilities/default.json`.

## ⚙️ Settings & Database
- SQLite is used for a key/value settings table. DB column names may differ from Rust struct field names (mappings are in `src-tauri/src/settings/mod.rs`).
- **Conversions to note:** 
  - Time is stored in DB as **minutes**, but the `Settings` struct holds **seconds**.
  - Volume is stored in DB as **0–100**, but the `Settings` struct holds **0.0–1.0**.
- **Adding a new setting requires:**
  1. A DB key mapping in `src-tauri/src/settings/mod.rs`.
  2. A default value in `src-tauri/src/settings/defaults.rs`.
  3. A corresponding `settings_set` key handler.
- **Migrations:** Add new schema migrations in `src-tauri/src/db/migrations.rs` (add a new `MIGRATION_N` constant and increment schema version).

## 🎨 UI Conventions
- **Window:** The app uses a decoration-free window with a custom titlebar (`Titlebar.svelte`).
- **Compact mode:** Triggered when window is resized small (`w < 300 || h < 300`).
- **Scale:** `uiScale` is applied via CSS `zoom` on the `.timer` div (not `transform: scale`).
- **Themes:** Theme JSON files apply custom CSS properties prefixed with `--`. The Rust backend watches the theme directory and emits `themes:changed`.
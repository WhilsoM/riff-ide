# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Common commands

All commands below assume the current working directory is the repository root.

### Build and run the app
- Debug build and run the GUI IDE:
  - `cargo run`
- Release build and run (optimized, slower to build):
  - `cargo run --release`
- Build without running:
  - `cargo build`
  - `cargo build --release`

### Tests
- Run all tests in the project (including library tests such as `core/lib/rsx`):
  - `cargo test`
- Run a specific test or group of tests by name substring:
  - `cargo test <name_substring>`

## High-level architecture

### Crates
- **`riff` (root crate)**
  - Binary crate that builds the native IDE application using `eframe`/`egui`.
  - Depends on the internal proc-macro crate `riff-rsx-macro` and several GUI/runtime libraries.
- **`riff-rsx-macro` (`rsx-macro/`)**
  - Provides the `#[component]` attribute macro used to declare RSX-style function components.
  - The macro currently forwards the function body and adds `#[allow(non_snake_case)]`, integrating with the runtime in `core/lib/rsx`.

### Entry point and top-level app
- **`src/main.rs`**
  - Entrypoint that configures `eframe::NativeOptions` and calls `eframe::run_native` with `MyApp` as the `eframe::App` implementation.
  - Initializes the global `AppNameStore` and the `IconsInteractionsStore`, then calls `modules::plugins::run_test_plugin()` before starting the UI event loop.
- **`core::app::MyApp` (`src/core/app.rs`)**
  - Central application struct implementing `eframe::App`.
  - Responsibilities in `update`:
    - Handles global input such as font zooming: reads `egui` scroll delta and Command modifier, updates the reactive `font_size` in `GlobalStore`, and applies `egui` visuals accordingly.
    - Applies theming by reading from the editor theme store (`modules::editor::stores::theme_store`) and mapping it into `egui::Visuals` (panel fills, background colors, selection colors, etc.).
    - Observes high-level UI actions from the file interaction store (e.g., `UiAction::OpenFile`) and delegates to the editor interactions store to open tabs.
    - Assembles and registers `AppStores` via `modules::editor::stores::context::set_all_stores`, wiring together:
      - File actions/interactions stores
      - Editor interactions store
      - Theme store
      - Icons store
      - Global store (`GlobalStore`)
      - The current directory file list
    - Creates the editor root component `modules::editor::components::App` and renders it via `core::ui::ui_kit::render_app`.
  - On construction, `MyApp::new`:
    - Determines the current working directory and reads its entries via `core::utils::utils::read_current_folder`.
    - Instantiates per-domain stores (`FileActionsStore`, `FileInteractionsStore`, `EditorInteractionsStore`, `ThemeInteractionsStore`, `HotkeysInteractionsStore`, `GlobalStore`) wrapped in `Rc<RefCell<...>>` as appropriate.

### Core layer (`src/core`)
- **`core/mod.rs`**
  - Re-exports major core submodules: `app`, `context`, `enums`, `lib`, `models`, `stores`, `types`, `ui`, and `utils`.
- **Application context (`core/context.rs`)**
  - Defines `AppContext`, a thread-local, globally accessible context holding references to key stores and the `egui::Context`.
  - Provides helpers:
    - `init_context` to initialize the thread-local context once.
    - `get_context`, `try_get_context`, and `with_context` to access the context from anywhere without explicit parameter threading.
- **Global state and reactivity (`core/stores`)**
  - `GlobalStore` (`core/stores/global_store.rs`):
    - Declared via a `store!` macro, with reactive fields such as `is_open_settings`, `font_size`, and `is_show_settings` using the `ReField` type from `core::lib::reaxive::reactive`.
    - Exposes methods to toggle settings flags and update font size, automatically propagating changes via the reactive system.
    - `global_store()` returns a shared singleton reference for use across the app.
  - Other core stores (e.g., `icons`, `app_name_store`) provide cross-cutting concerns used by multiple modules.
- **Reactive + RSX runtime (`core/lib`)**
  - `core/lib/reaxive`: home-grown reactive state primitives (`ReField`, store macros) used by `GlobalStore` and editor/theme stores.
  - `core/lib/rsx`:
    - Provides the component runtime used with the `#[component]` macro, including:
      - Component abstraction (`ComponentWrapper` and friends).
      - Error boundary support (`ErrorBoundary`).
      - Component lifecycle handling (`ComponentLifecycle`, `LifecycleWrapper`).
      - Memoization and hashing helpers (`ComponentCache`, `MemoizedComponent`, `compute_hash`).
    - This layer lets higher-level modules express UI as composable RSX-style components rather than imperative `egui` code everywhere.
- **UI toolkit (`core/ui`)**
  - `core/ui/ui_kit`:
    - Contains reusable UI building blocks and layout primitives (e.g., app shell, panels, buttons, text, images, scroll areas, styles, and style applicators).
    - `render_app` is the main entry used by `MyApp` to render the top-level component tree into `egui`.
  - `core/ui/widgets`:
    - Higher-level widgets such as file lists and side panels that are shared across modules.
  - `core/ui/draw_entry` and related helpers handle drawing the file system entries and other low-level UI concerns.
- **Utilities (`core/utils`)**
  - `core/utils/utils.rs` includes helpers like `read_current_folder` for reading directory entries into the internal `EntryRc` representation used by file explorer components.

### Feature modules (`src/modules`)

#### Editor module (`src/modules/editor`)
- **Components (`modules/editor/components`)**
  - `App` is the top-level editor component, annotated with `#[component]` from `riff-rsx-macro`.
    - It constructs high-level subcomponents such as `Navbar`, `FileExplorerPanel`, and `CodeEditorPanel`, wiring them together conceptually as the main IDE layout.
  - Additional components (e.g., `bottom_panel`, `tabs_bar`, `left_panel`, `right_panel`) represent distinct UI regions, typically taking an `egui::Context` and using stores to drive their behavior.
- **Stores (`modules/editor/stores`)**
  - Organized by concern:
    - `editor`: editor interactions (tabs, cursor, etc.).
    - `file`: file actions, interactions, and services for file system operations and state.
    - `hotkeys`: action and interaction stores for keyboard shortcuts.
    - `theme`: theme state and derived colors used by both the core visuals configuration and editor components.
  - `modules/editor/stores/context.rs` exposes `AppStores` and `set_all_stores` to register the set of active stores; this is called from `MyApp::update` so editor components can fetch the current stores without passing them explicitly through every function.
- **Shared editor utilities (`modules/editor/shared`)**
  - Shared types and helpers—especially theming primitives (`shared/theme.rs`)—used across editor components and stores to keep styling consistent.

#### Plugins module (`src/modules/plugins`)
- Integrates a Lua runtime via `mlua` to support scripting and plugins.
- `run_test_plugin`:
  - Creates a new Lua state, reads `test_plugin.lua` from the repository root, and executes it.
  - Exposes host functions to Lua such as `disable_mouse`, which toggles the global `MOUSE_ENABLED` flag.
  - Intended as a proof-of-concept for a plugin system; commented code shows the plan to scan an `extensions/` directory for `.lua` plugins.
- Global plugin state:
  - `MOUSE_ENABLED` is a `static mut` boolean used to gate mouse interactions (currently toggled via Lua).

#### Settings module (`src/modules/settings`)
- Provides the settings UI and its backing state:
  - `components` and `pages` define the concrete settings screens.
  - `stores` contains settings-related state.
  - `shared` holds shared types/utilities.
- Visibility of settings is controlled via fields in `GlobalStore` (e.g., `is_open_settings`, `is_show_settings`), with helper methods to toggle these flags.

### RSX macro crate (`rsx-macro`)
- `rsx-macro/src/lib.rs` defines the `#[component]` proc-macro attribute:
  - Parses a function (`ItemFn`), preserves its attributes and signature, wraps it with `#[allow(non_snake_case)]`, and returns it unchanged otherwise.
  - Used across the editor module (and potentially elsewhere) to mark RSX-style components that integrate with the `core/lib/rsx` runtime.

## How pieces fit together
- The **binary crate** ties together core services, feature modules, and the RSX system into a native IDE:
  - `src/main.rs` initializes core stores, icons, and plugins, then delegates control to `MyApp` via `eframe::run_native`.
  - `MyApp` orchestrates:
    - Global input handling and visual configuration.
    - Registration of all domain-specific stores via `AppStores`.
    - Rendering of the root editor component through the UI kit and RSX runtime.
- The **core layer** provides shared infrastructure: global state management via `GlobalStore`, the reactive system, the RSX runtime, UI toolkit primitives, and utilities.
- The **editor module** defines the IDE’s main experience—layout, file explorer, code editor, hotkeys, theme, and related state—all built on the core RSX/reactive infrastructure.
- The **plugin system** (currently centered around `run_test_plugin` and `test_plugin.lua`) demonstrates how external Lua scripts can interact with internal state (e.g., toggling mouse behavior) and is the natural extension point for future plugin-related work.

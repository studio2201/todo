# RustDo

A high-performance, single-purpose todo list application written in **100% Rust**. Powered by **Axum** on the backend and **Yew (WebAssembly)** on the frontend.

No heavy databases, no bloated JavaScript runtime, no tracking—just todos, compiled to native code.

---

## Features

- ✨ **Clean, Minimal Interface**: A premium responsive layout optimized for mobile and desktop.
- 🌓 **Automatic Dark/Light Mode**: Synced to system preference with localStorage override.
- 💾 **Atomic File-Based Storage**: Todos are persisted safely using atomic file renames to prevent corruption.
- 🚀 **Blazing Fast WASM**: Client UI is powered by Rust compiled to WebAssembly (via Yew & Trunk).
- 🔒 **PIN Lockout Protection**: Secure client-IP rate limiting and timing-safe constant-time comparisons.
- 🌐 **PWA & Offline Support**: Fully installable as a web app with service-worker caching.
- 🌍 **Global Localization (i18n)**: Lightweight type-safe client-side translations supporting 8 languages (English, Chinese, Spanish, German, Japanese, French, Portuguese, Russian).

---

## Environment Variables

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `PORT` | The port number the Axum server will listen on | `3000` | No |
| `RUSTDO_PIN` | Secure PIN code for accessing todos (4-10 digits) | - | No |
| `RUSTDO_SITE_TITLE` | Override the UI and HTML title | `RustDo` | No |
| `SINGLE_LIST` | Show a single list of todos (without selector controls) | `false` | No |
| `ALLOWED_ORIGINS` | Restrict CORS origins (e.g. `https://sub.domain.com`) | `*` | No |

---

## Build and Run Locally

Ensure you have Rust and **Trunk** installed:

```bash
# 1. Install target for WASM compilation
rustup target add wasm32-unknown-unknown

# 2. Install Trunk for building the Yew frontend
cargo install trunk
```

### 1. Build and Start the Application

1. **Clone the repository** and navigate to it:
   ```bash
   git clone https://github.com/UberMetroid/RustDo.git
   cd RustDo
   ```

2. **Build the Yew frontend assets**:
   ```bash
   cd frontend
   trunk build --release
   cd ..
   ```

3. **Start the Axum backend server**:
   ```bash
   # Defaults to port 3000
   cargo run --bin backend --release
   
   # Or run on a custom port (e.g. 3002)
   PORT=3002 cargo run --bin backend --release
   ```

4. Open `http://localhost:3000` (or `http://localhost:3002`) in your web browser.

---

## Using Docker Compose

Build and spin up the entire application inside a lightweight Alpine container:

```bash
docker-compose up --build -d
```

Your `docker-compose.yml` service configuration:

```yaml
services:
  rustdo:
    build: .
    image: ubermetroid/rustdo:latest
    container_name: rustdo
    restart: unless-stopped
    ports:
      - ${RUSTDO_PORT:-3000}:3000
    volumes:
      - ${RUSTDO_DATA_PATH:-./data}:/app/data
    environment:
      - RUSTDO_PIN=${RUSTDO_PIN-}
      - RUSTDO_SITE_TITLE=RustDo
      - SINGLE_LIST=${SINGLE_LIST:-false}
```

---

## Project Structure

All source files are strictly bounded between 25 and 250 lines of code for high maintainability.

```
RustDo/
├── Cargo.toml                # Workspace manifest (LTO, strip, optimization flags)
├── Dockerfile                # Multi-stage optimized Rust container builder
├── docker-compose.yml        # Docker Compose configuration
├── data/                     # Persistent todo JSON storage folder
│   └── todos.json
├── shared/                   # Shared Rust structures (DTO data shapes)
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs            # Structs for items, config, verification
├── backend/                  # Axum HTTP server
│   ├── Cargo.toml            # De-bloated backend manifest (zero rand/sha2/url dependencies)
│   └── src/
│       ├── auth.rs           # Zero-allocation timing-safe comparisons and native PRNG IDs
│       ├── handlers.rs       # Stream serialization I/O and verify handlers
│       ├── main.rs           # Server routing config & CORS mirroring middleware
│       ├── middleware.rs     # Authorization & zero-allocation origin parsing
│       ├── state.rs          # Shared configuration and locking structures
│       └── static_files.rs   # Static resource pre-caching and asset loading
└── frontend/                 # Yew WebAssembly client
    ├── Cargo.toml            # Client manifest (pruned dependencies)
    ├── index.html            # Trunk entry layout
    ├── styles.css            # Responsive layout design system
    ├── service-worker.js     # PWA caching lifecycle
    └── src/
        ├── api.rs            # Async API fetch interface
        ├── i18n/             # Dedicated language dictionaries (en, zh, es, de, ja, fr, pt, ru)
        ├── i18n.rs           # Type-safe i18n state hook & translate dispatcher
        ├── list_handlers.rs  # Event handlers for list selection & alterations
        ├── list_selector.rs  # List switcher UI component
        ├── login.rs          # Secure login layout centered with absolute buttons
        ├── main.rs           # Router context binding and index app mount
        ├── toast.rs          # Toast notifications component
        ├── todo_form.rs      # Input form for adding tasks
        ├── todo_header.rs    # Title and list selector controls
        ├── todo_item.rs      # Todo item supporting editing, reordering, and drag actions
        ├── todo_items_list.rs# Filters and lists active vs completed tasks
        ├── todo_list_handlers.rs # Event handlers for task addition, completion, deletion
        ├── todo_list.rs      # Todo parent container component
        └── types.rs          # Custom status model properties
```

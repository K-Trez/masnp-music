# Project Context - project-1

## Overview

A Tauri v2 desktop application built with Svelte frontend and Rust backend. This is a minimal Tauri template with basic shell, dialog, and filesystem plugins enabled.

---

## Tech Stack

### Backend (Rust)
- **Tauri v2** - Desktop application framework
- **tauri-plugin-shell** - Shell integration plugin
- **serde/serde_json** - Serialization libraries
- **Rust edition 2021**

### Frontend (Svelte)
- **Svelte** - Reactive JavaScript framework
- **Vite** - Build tool and dev server (devUrl: http://localhost:5173)

---

## Project Structure

```
project-1/
├── src-tauri/              # Rust backend
│   ├── Cargo.toml         # Rust dependencies
│   ├── tauri.conf.json    # Tauri configuration
│   ├── build.rs           # Build script
│   ├── rust-toolchain.toml # Rust version
│   └── src/
│       └── main.rs        # Rust entry point
├── src/                    # Svelte frontend
│   ├── app.html           # HTML template
│   ├── main.js            # Svelte entry point
│   ├── App.svelte         # Main component
│   └── App.css            # Styles
├── package.json            # Node.js dependencies
├── .gitignore             # Git ignore rules
└── README.md              # Documentation
```

---

## Configuration Details

### Application Settings (tauri.conf.json)
- **Product Name**: project-1
- **Version**: 0.1.0
- **Identifier**: com.project-1.app
- **Window Size**: 800x600 (resizable, centered, not fullscreen by default)

### Enabled Plugins
| Plugin | Features |
|--------|----------|
| shell | open: true - Can open external applications/files |
| dialog | all: true - File dialogs, prompts, etc. |
| fs | all: true - Full filesystem access |

### Build Configuration
- **Before Dev Command**: `npm run dev`
- **Before Build Command**: `npm run build`
- **Dev URL**: http://localhost:5173
- **Frontend Dist**: ../src

---

## Dependencies

### Rust (Cargo.toml)
```toml
[dependencies]
tauri = "2"
tauri-plugin-shell = "2"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[build-dependencies]
tauri-build = { version = "2" }
```

### Node.js (package.json)
- Vite and Svelte dependencies (inferred from structure)

---

## Development Workflow

### Prerequisites
- Rust 1.80+
- Node.js 18+
- npm or yarn

### Setup Steps
1. Navigate to project directory
2. Run `npm install` to install frontend dependencies
3. Run `cargo build --release` once for the Rust backend

### Development Commands
```bash
# Start dev server (Vite)
npm run dev

# Build for production
npm run build
cargo build --release

# Preview production build
npm run preview
```

---

## Release Profile Configuration

The release profile is optimized with:
- **Optimization level**: 3
- **Link-time optimization**: Enabled
- **Strip symbols**: Enabled

---

## Security & Bundle Settings

### Content Security Policy (CSP)
- Currently set to `null` (no restrictions)

### Bundle Configuration
- **Category**: DeveloperTool
- **Copyright**: 2024 Your Name
- **Icon sizes**: 32x32, 128x128 (standard sizes)

---

## Notes for Future Development

1. **Frontend**: The Svelte components are in `src/` directory. Check `App.svelte` for the main component structure.

2. **Backend**: The Rust backend is minimal. Add custom commands in `src-tauri/src/main.rs` using Tauri's command API.

3. **Plugins**: Three plugins are enabled by default:
   - Use `tauri-plugin-shell` to open files/URLs
   - Use `dialog` for file selection, save dialogs, etc.
   - Use `fs` for file system operations

4. **Build Process**: Always run `npm run build` before `cargo build --release` to ensure the frontend is bundled correctly.

5. **Platform**: Currently configured for Windows (windows_subsystem = "windows").

---

## Last Updated
2026-10-18

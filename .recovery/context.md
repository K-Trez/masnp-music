# Project Context - masnp-music

## Overview

A Tauri v2 desktop application built with Svelte frontend and Rust backend. This is a **low latency music search aggregator and player** application.

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

## Project Goal: Low Latency Music Search Aggregator & Player

### Purpose
Build a desktop application that aggregates music from multiple sources and provides low-latency playback.

### Key Features to Implement
- **Music Search**: Search across multiple music sources
- **Aggregation**: Combine results from different providers
- **Low Latency Playback**: Minimize buffering and startup time
- **Player Controls**: Play, pause, skip, volume control
- **Playlist Management**: Create and manage playlists

### Architecture Considerations
- Use Rust for performance-critical backend operations
- Leverage Tauri's IPC (Inter-Process Communication) for low-latency audio handling
- Consider using WASM for frontend audio processing if needed
- Implement efficient caching strategies for search results

---
## Development Plan

### Phase 1: Research & Architecture Planning
- [ ] Identify music API sources (Spotify, SoundCloud, YouTube, local files)
- [ ] Design low-latency audio pipeline architecture
- [ ] Plan search aggregation strategy

### Phase 2: Backend (Rust) Development
- [ ] Set up Tauri IPC handlers for audio control
- [ ] Implement music source connectors
- [ ] Build search API endpoints
- [ ] Implement caching layer for search results

### Phase 3: Frontend (Svelte) Development
- [ ] Create main app layout and navigation
- [ ] Build search interface with filters
- [ ] Design player component with controls
- [ ] Implement playlist management UI
- [ ] Add real-time search results display

### Phase 4: Audio Pipeline Optimization
- [ ] Implement low-latency audio buffer management
- [ ] Optimize network streaming for minimal buffering
- [ ] Add audio quality settings (bitrate, codec)
- [ ] Implement background playback support

### Phase 5: Integration & Testing
- [ ] Connect all music sources
- [ ] Test search aggregation across providers
- [ ] Optimize latency metrics
- [ ] Fix bugs and edge cases

### Phase 6: Polish & Deployment
- [ ] Add error handling and user feedback
- [ ] Create installation package
- [ ] Write documentation
- [ ] Prepare for release

---

## Phase 1 Decisions (Research & Planning)

**Music Sources**: Public APIs only, free tier, no login required  
**Latency Budget**: Target < 500ms startup (understream buffering acceptable)  
**Auth Strategy**: None initially, revisit if needed  
**MVP Priority**: Search-first approach

## Phase 1 Research Findings

### Public Music APIs (Free, No Login Required)
| API | Free Tier | Latency | Pros/Cons |
|-----|-----------|---------|-----------|
| Free Sounds API | Unlimited free SFX/music | Fast | Limited music variety |
| Incompetech API | Direct source | Very fast | Kevin MacLeod library only |
| Bensound (sample) | Limited tracks | Medium | Requires attribution |
| Internet Archive / Archive.org | Thousands of samples | Variable | Public domain works |
| MusicBrainz | Search only, no streams | Fast | Metadata/indexing |
| Spotify Web API | Free (requires OAuth) | Fast | User login required - skip for now |
| SoundCloud (limited) | Read-only public endpoints | Medium | Rate limits, auth needed for full access |

**Recommended Starting Points**:
1. **Internet Archive API** - Free, no auth, thousands of audio files (public domain + creative commons)
2. **Free Sounds/Effects APIs** - Specialized for SFX, fast responses
3. **MusicBrainz** - For metadata/search, not streaming

### Development Timeline
- Research & Planning: In progress (Phase 1)
- Backend Setup (Phase 2): ~3-5 days
- Frontend UI (Phase 3): ~4-6 days
- Audio Pipeline (Phase 4): ~3-4 days
- Integration & Testing (Phase 5): ~3-5 days
- Polish & Release (Phase 6): ~2-3 days

**Estimated MVP**: ~3 weeks total

## Last Updated
October 19, 2026 at 20:30 UTC - Session Summary

### Session Progress Summary

#### Phase 1: Research & Architecture ✅ COMPLETE
- Identified public music APIs (free, no login): Internet Archive, MusicBrainz, Free Sounds
- Decided on search-first MVP approach
- Targeted <500ms latency with understream buffering acceptable

#### Phase 2: Backend Setup 🚧 IN PROGRESS
**Completed Work:**
1. ✅ Created `src-tauri/src/lib.rs` with music service layer
   - Defined SearchResult, SearchRequest, AggregatedResult data structures
   - Implemented search functionality placeholders
   - Added MusicError enum for error handling
   - Created mock implementations for development

2. ✅ Updated `src-tauri/Cargo.toml` 
   - Set package name to "masnp-music"
   - Added serde/serde_json dependencies
   - Configured optimized release profiles (LTO, strip)

3. ✅ Updated `src-tauri/src/main.rs`
   - Integrated lib module
   - Registered search_music and get_app_info Tauri commands
   - Configured shell plugin

**Current Blocker:** Registry Cache Issue
- Stale crates in `.cargo/registry/src` require edition2024 feature
- Attempted cleanup: removed registry, cleared cache, locked Cargo.lock
- Issue: Dependencies downloaded still have cached metadata requiring newer Rust features
- **Resolution pending**: Either upgrade Rust toolchain further or pin Tauri to compatible versions

#### Files Created/Modified in Session:
| File | Status |
|------|--------|
| `src-tauri/src/lib.rs` | NEW - Music service layer with data structures |
| `src-tauri/Cargo.toml` | UPDATED - Package config |
| `src-tauri/src/main.rs` | UPDATED - Command registration |
| `.qwen/projects/d--code-project-1/memory/masnp-music.md` | NEW - Session memory |
| `context.md` | UPDATED - Added Phase 1 research & decisions |

### Next Steps When You Return:
1. Continue debugging registry cache issue OR
2. Switch to frontend-first development (Svelte UI is already ready) OR  
3. Reinstall Rust toolchain fresh if needed

---

**Current Project Status**: Backend build blocked by registry cache, 
Frontend work available as alternative path

### Quick Commands (if needed):
- Clear cache: `rmdir /s /q C:\Users\k00k13\.cargo\registry`
- Reset Rust: `rustup uninstall stable` then reinstall  
- Check status: `cargo check` or `cargo fetch`

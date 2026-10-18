# project-1

A Tauri v2 application built with Svelte and Rust.

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
└── README.md              # This file
```

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.80 or higher)
- [Node.js](https://nodejs.org/) (version 18 or higher)
- [npm](https://www.npmjs.com/) or [yarn](https://yarnpkg.com/)

## Installation

1. Navigate to the project directory:
   ```bash
   cd D:\CODE\project-1
   ```

2. Install Node.js dependencies:
   ```bash
   npm install
   ```

3. Build the Rust backend (first time only):
   ```bash
   cargo build --release
   ```

## Development

Run the development server:
```bash
npm run dev
```

This will start a Vite dev server at `http://localhost:5173`. Tauri will automatically open the application window.

## Building for Production

Build the frontend and package the application:
```bash
npm run build
cargo build --release
```

The production build will be available in the `src-tauri/target/release/` directory.

## Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run preview` - Preview production build

## License

This project is licensed under the MIT License - see the LICENSE file for details.

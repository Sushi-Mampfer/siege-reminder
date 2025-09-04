# Axum-Leptos Full-Stack Application

_A "2025" version of the official [Axum-Leptos template](https://github.com/leptos-rs/leptos/tree/main/examples/tailwind_axum)_

## Tech Stack

- **Frontend**:
  - [Leptos](https://leptos.dev/) - A modern Rust web framework for building reactive web applications
  - [TailwindCSS](https://tailwindcss.com/) - A utility-first CSS framework for rapid UI development
  - WebAssembly - For running Rust code in the browser

- **Backend**:
  - [Axum](https://github.com/tokio-rs/axum) - Ergonomic and modular web framework built with Tokio, Tower, and Hyper
  - [Tokio](https://tokio.rs/) - Asynchronous runtime for Rust
  - [Tower](https://github.com/tower-rs/tower) - Tower is a library of modular and reusable components for building robust networking clients and servers.
  - [Tower-http](https://github.com/tower-rs/tower-http) - HTTP specific Tower utilities
  
- **Other**:
  - [reqwasm](https://github.com/koute/reqwasm) - A simple HTTP client for WebAssembly
  - [serde](https://serde.rs/) - A data serialization framework for Rust
  - [serde_json](https://serde.rs/json.html) - A JSON serialization/deserialization library for Rust
  - [cargo-leptos](https://github.com/leptos-rs/cargo-leptos) - Build tool for Leptos applications

## Getting Started

### Prerequisites

1. Install Rust (nightly):
```bash
rustup toolchain install nightly --allow-downgrade
rustup target add wasm32-unknown-unknown
```
2. Install cargo-leptos:
```bash
cargo install cargo-leptos --locked
```

3. Install Node.js dependencies:
```bash
npm install
cd end2end && npm install
```

### Development

1. Start the development server and Tailwind watcher in separate terminals:
```bash
# Terminal 1: Start the Leptos dev server
cargo leptos watch

# Terminal 2: Watch for Tailwind changes
npm run watch
```
This will start your application at `127.0.0.1:3000` and automatically rebuild CSS when Tailwind classes change.

2. For production build:
```bash
cargo leptos build --release
```

### Testing

Run end-to-end tests:
```bash
cargo leptos end-to-end
```

## Project Structure

- `src/`
  - `main.rs` - Server entry point with Axum configuration
  - `lib.rs` - Shared code and WASM hydration setup
  - `app.rs` - Main application component and routing setup
  - `components/` - Reusable UI components
    - `Counter.rs` - Example counter component
    - `mod.rs` - Components module definitions
  - `pages/` - Application pages/routes
    - `Home.rs` - Homepage component
    - `About.rs` - About page with API integration
    - `mod.rs` - Pages module definitions
  - `server/` - Backend server code
    - `handlers.rs` - API endpoint handlers
    - `routes.rs` - API route definitions
    - `mod.rs` - Server module setup
- `style/` - CSS and TailwindCSS files
- `end2end/` - End-to-end tests with Playwright
  - `tests/` - Test specifications
  - `playwright.config.ts` - Playwright configuration
- `public/` - Static assets
- `Cargo.toml` - Rust dependencies and build configuration
- `package.json` - Node.js dependencies
- `tailwind.config.js` - TailwindCSS configuration

## Features

- Full-stack Rust development with shared types
- Server-side rendering (SSR) with hydration
- Client-side routing
- API integration example
- Reactive state management
- Modern CSS with TailwindCSS
- End-to-end testing setup
- Development hot-reload

## License


This project is released under the Unlicense. Feel free to use it as a starting point for your own applications.


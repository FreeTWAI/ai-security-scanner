# Developer Usage

This document outlines how to set up the development environment, build the ai-security-scanner application, and run the test suite.

## Prerequisites

- **Node.js**: Version 24 or newer.
- **Rust**: Version 1.98 or newer.
- **System Dependencies (Linux)**: The Tauri backend requires certain platform dependencies. Install them using:
  ```bash
  sudo apt-get install -y pkg-config libglib2.0-dev libgtk-3-dev libwebkit2gtk-4.1-dev
  ```

## Setup and Build

1. Clone the repository and navigate to the root directory.
2. Install Node.js dependencies:
   ```bash
   npm ci
   ```
3. Build the frontend and backend:
   ```bash
   npm run build
   ```

## Running the Application

To run the application in development mode with a browser preview (using sample data):
```bash
npm run dev
```

To run the full Tauri desktop application:
```bash
npm run tauri dev
```

## Running Tests

The project includes various tests for different components.

### Frontend and Component Tests

- **Type Checking**:
  ```bash
  npm run typecheck
  ```
- **Frontend Tests**:
  ```bash
  npm run test:frontend
  ```
- **Component Tests** (using Vitest):
  ```bash
  npm run test:component
  ```

### Backend (Rust/Tauri) Tests

- **Workspace Tests**:
  ```bash
  cargo test --locked --workspace --no-default-features --features cli
  ```

### Additional Validations

- **Release Evidence**:
  ```bash
  npm run test:release-evidence
  ```
- **Usability Evidence**:
  ```bash
  npm run test:usability-evidence
  ```
- **Engine Validations**:
  ```bash
  npm run validate:engines
  ```

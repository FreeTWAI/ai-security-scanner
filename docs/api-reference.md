# API Reference

This document provides a high-level overview of the core modules in the ai-security-scanner application, covering both the Rust/Tauri backend and the TypeScript frontend.

## Backend (Rust/Tauri) Core Modules

The backend is responsible for interacting with the operating system, managing scans, and providing a secure environment for execution. Core modules are located in `src-tauri/src/`.

- **`case_service.rs`**: Manages the lifecycle and state of security assessment cases. It handles case creation, updates, and persistence.
- **`orchestrator.rs`**: The central coordinator for scan execution. It determines which engines to run based on the selected assets, manages execution plans, and aggregates results.
- **`managed_runtime.rs`**: Provides an isolated execution environment for upstream engines. It handles the lifecycle, isolation, and repair of the runtime environment to ensure safe and deterministic scan execution.

## Frontend (TypeScript/React) Core Modules

The frontend provides the user interface and handles presentation logic, interacting with the backend via Tauri commands. Core modules are located in `src/`.

- **`scanActivity.ts`**: Manages the active state of a scan. It tracks progress, updates the UI with real-time status, and handles user interactions during an ongoing scan.
- **`snapshotWorkspace.ts`**: Handles the preparation and configuration of local project workspaces for scanning. It ensures bounded, read-only snapshots are created correctly before handing off to the backend.
- **`runLifecycle.ts`**: Manages the overall lifecycle of a scan run from setup and readiness checks to final disposition and result presentation.

# Retro Explorer (REX)

## Core Vision

REX is a high-performance emulation frontend and scripting platform. It acts as a wrapper around RetroArch/Libretro cores, bridging the emulation layer with a web-based UI. Creators build custom, game-specific "dashboards" or "scripts" that display live game data and control the emulation state via a JSON-RPC interface. REX integrates with game databases like RetroAchievements and No-Intro to aggregate game metadata and allow exploring the vast library of retro games. User can also attach and view custom game related data like guides and notes to track progress.

## Monorepo Structure

The project is organized into specialized sub-projects to decouple the UI, engine logic, and emulation bridge:

- /game-scripts: Plain HTML/JS/CSS files (the user-facing content). These connect to the core via a JSON-RPC client to display game data or achievements.
- /libretro-proxy: The Rust bridge that interacts directly with Libretro cores.
- /rcheevos-hash-sys: Rust wrapper for rcheevos hash functions for RetroAchievements-compatible hashes.
- /retro-scripting: The libretro core communication logic. Features a full JSON-RPC implementation that serves as the communication backbone between the web scripts and the emulation cores.
- /rex-app: The main Tauri host application.
    - /src: The frontend source/assets for the engine's main "System UI."
    - /src-tauri: Rust backend, window management, and custom request handlers.

## Architecture & Design Choices

- Unified Origin: The application uses a custom scheme (rex://localhost) to ensure the System UI, Game Scripts, and User Scripts share a single security origin. This allows seamless iframe integration of game scripts without CORS issues.
- Virtual Path Routing: A custom handler maps virtual paths to physical locations:
    - /game-scripts/ -> Maps to the monorepo’s /game-scripts directory (dev mode).
    - /user-scripts/ -> Maps to the system AppData folder for user-installed content.
    - / -> Defaults to the embedded assets for the core engine UI. Built in game-scripts and other assets are embedded in the binary.
- Communication Layer: Logic is handled via JSON-RPC. Game scripts (running inside the app or standalone) use a JSON-RPC client to query game-related data from the emulation cores.
- Custom HMR (Hot Module Replacement): A Rust-based file watcher monitors the monorepo (UI and game-scripts) and triggers manual refreshes in the view during development.
- Explorable game data: Store all data in a local sqlite database.

## Current Features

- Main UI interface consisting of "tabs" that display game scripts or other core interfaces using iframes.
- Multi-Source Routing: Functional protocol handler pulling from embedded assets, the monorepo, and local disk simultaneously.
- JSON-RPC Infrastructure: Complete implementation in retro-scripting for core-to-UI communication.
- Emulation Hashing: Wrapper crate exists, but is not being used yet.
- Developer Workflow: Automated file watching and auto-refresh for the entire monorepo environment.

## Next steps

### Collection management

- Scan local directories for game files (should support plain ROM files, disk images and zip files)
- Store collection definition in a sqlite database
- Display collection on the UI
- Hash game files with rcheevos-hash-sys to detect games / integrate with RetroAchievements
- Also integrate with other game databases (like no intro dat files) to detect / categorize games
- Fetch game and achievements data from RetroAchievements and display in the UI

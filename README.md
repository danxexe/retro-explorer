# Retro Explorer (REX)

A collection of tools and a frontend for exploring, managing and interacting with retro games.
This project serves as a monorepo for several related components designed to work together or as standalone libraries.

> **Note:** This project is primarily developed for personal use. While the code is available under the MIT license, the project is in an early stage and features are subject to frequent changes.

## Project Structure

The project is divided into several sub-projects:

* **retro-explorer-app:** A desktop application for browsing game libraries and launching emulators.
* **retro-scripting:** A system for executing scripts that interact with emulated game memory.
* **libretro-proxy:** A bridge for communication between the frontend and Libretro-compatible cores.
* **rcheevos-hash-sys:** A wrapper for the RetroAchievements game hashing library.

Each sub-project is contained within its own directory. Dependencies and build instructions are specific to each component.

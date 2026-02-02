# MiniWM

A tiling Wayland compositor built on top of Smallvil and Smithay - a small project for learning how tiling WMs and wayland compositors work.

## Planned Features

### Core Functionality
- **Tiling Layouts**: Multiple tiling layouts (master-stack, grid, spiral, etc.)
- **Multi-Monitor Support**: Independent workspaces per monitor with output management
- **Workspaces**: Virtual desktops for organizing windows

### Window Management
- **Dynamic Tiling**: Automatic window arrangement with configurable gaps and borders
- **Window Rules**: Per-application window placement and behavior rules
- **Focus Management**: Flexible focus models (focus-follows-mouse, click-to-focus)
- **Window Resizing**: Keyboard and mouse-based window resizing
- **Fullscreen Support**: Native fullscreen window handling

### Configuration
- **Config File**: Configuration for keybindings, layouts, and appearance
- **Hot Reload**: Dynamic configuration reloading without restart

## Dependencies
- libwayland
- libxkbcommon
- libudev
- libinput
- libgbm
- libseat
- xwayland

## Building

```bash
cargo build --release
```

## Installation

```bash
cargo install --path .
```

## Usage

Start the compositor from a TTY:

```bash
miniwm
```

## Current Roadmap

- [ ] Basic tiling layout implementation
- [ ] Keyboard input and keybinding system
- [ ] Multi-monitor support
- [ ] Configuration file parsing

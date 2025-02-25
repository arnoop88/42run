# 42run - Endless Runner Game

![Gameplay Demo](https://imgur.com/a/NCGadaM)

A 3D endless runner game inspired by Temple Run, built with Rust and OpenGL. Run as far as you can through procedurally generated obstacles and unlock new maps and skins!

## Features

- 🏃 **Endless Procedural Generation**  
  Infinite randomly generated obstacles with increasing difficulty
- 🌍 **Multiple Maps**  
  Unlock different environments: Campus, Cave, and Temple
- 🎨 **Customizable Skins**  
  Collect unique character skins with different visual styles
- 🏆 **Progression System**  
  Unlock new content through achievements and high scores
- 🎮 **Gameplay Features**:
  - Jumping/Sliding mechanics
  - Dynamic camera system
  - Speed increases over time
  - Collision detection with different obstacle types
- 🎵 **Audio System**:
  - Background music
  - Sound effects for actions and collisions
  - Different music tracks per map
- 💾 **Save System**  
  Persistent progress saving between sessions

## Controls
| Action          | Key Bindings           |
|-----------------|------------------------|
| Move Left       | `← `/ `A`              |
| Move Right      | `→` / `D`              |
| Jump            | `Space` / `W` / `↑`    |
| Slide           | `S` / `↓`              |
| Play            | `Enter`                |
| Quit            | `Esc` / `Q`            |
| Pause           | `Esc` / `Q`            |
| Retry           | `Enter` / `R`          |
| Resume          | `Enter` / `R`          |

## Installation

### Prerequisites
- Rust 1.60+ (install via [rustup](https://rustup.rs/))
- Cargo (Rust package manager)
- System dependencies:
  - OpenGL 3.3+
  - GLFW
  - libpng

#### Linux (Ubuntu/Debian)
```bash
sudo apt-get install build-essential libglfw3-dev libpng-dev
```

#### macOS
```bash
brew install glfw pkg-config
```

### Windows
Intall pre-built binaries for:
- [GLFW](https://www.glfw.org/download.html)
- [libpng](https://gnuwin32.sourceforge.net/packages/libpng.htm)

### Building & Running
```bash
git clone https://github.com/arnoop88/42run.git
cd 42run
make release
./42run
```
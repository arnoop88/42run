# 42run - Endless Runner Game

[![Gameplay Demo](https://i.imgur.com/k5aOERF.gif)](https://imgur.com/k5aOERF)

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
| Move Left       | `←` / `A`              |
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

#### Install dependencies
```bash
sudo apt-get install build-essential libglfw3-dev libpng-dev libasound2-dev libx11-dev libxrandr-dev libxinerama-dev libxcursor-dev libxi-dev libgl1-mesa-dev
```

### Building & Running
```bash
git clone https://github.com/arnoop88/42run.git
cd 42run
make release
./42run
```

# Particle Life Simulator

![Particle Life Simulation Demo](demo.gif) *Example simulation with red, blue, and green particles*

A Rust implementation of a particle life simulation demonstrating emergent behaviors from simple interaction rules.

## Features

- **Multiple Particle Types**: Red, blue, and green particles with configurable interactions
- **Interactive Controls**:
  - Add particles by clicking (left/right/middle mouse buttons)
  - Adjust interaction parameters in real-time
  - Pause/resume simulation
- **Preset Configurations**: 4 built-in presets showcasing different emergent behaviors
- **Performance Optimized**: Handles thousands of particles efficiently
- **Debug UI**: Real-time simulation statistics and parameter monitoring

## Controls

| Key | Action |
|-----|--------|
| SPACE | Pause/Resume simulation |
| R | Reset simulation |
| D | Toggle debug info |
| 1-4 | Load presets 1-4 |
| ESC | Quit |
| Left Click | Add red particle |
| Right Click | Add blue particle |
| Middle Click | Add green particle |
| F1-F9 | Select interaction parameter |
| +/- | Adjust selected parameter |

## Installation

1. Ensure you have [Rust installed](https://www.rust-lang.org/tools/install)
2. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/particle-life-simulator.git
   ```
3. Run the simulation:
   ```bash
   cd particle-life-simulator
   cargo run --release
   ```

## Configuration

Edit `src/simulation/interaction_matrix.rs` to modify:
- Particle interaction strengths
- Physical parameters
- Simulation rules

## Screenshots

![Preset 1](screenshots/preset1.png)  
*Preset 1: Basic particle interactions*

![Preset 4](screenshots/preset4.png)  
*Preset 4: Complex emergent patterns*

## Development Roadmap

- [ ] Add more particle types
- [ ] Implement GPU acceleration
- [ ] Add recording/playback functionality
- [ ] Create web version using WebAssembly

## License

MIT License - See [LICENSE](LICENSE) for details

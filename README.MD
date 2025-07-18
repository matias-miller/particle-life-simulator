# Particle Life Simulator

![Particle Life Simulation Demo](demo.gif) *Example simulation with red, blue, green, and neon pink particles*

A Rust implementation of a particle life simulation demonstrating emergent behaviors from simple interaction rules.

## Features

- **Multiple Particle Types**: 
  - Red, blue, green, and **neon pink** particles
  - Configurable interactions between all particle types
- **Interactive Controls**:
  - Add particles by clicking:
    - Left Click: Red particles
    - Shift + Left Click: Neon Pink particles
    - Right Click: Blue particles
    - Middle Click: Green particles
  - Adjust interaction parameters in real-time
  - Pause/resume simulation
- **6 Preset Configurations**: Showcasing different emergent behaviors
- **Performance Optimized**: Handles thousands of particles efficiently
- **Debug UI**: Real-time simulation statistics and parameter monitoring

## Controls

| Key | Action |
|-----|--------|
| SPACE | Pause/Resume simulation |
| R | Reset simulation |
| D | Toggle debug info |
| 1-6 | Load presets 1-6 |
| ESC | Quit |
| Left Click | Add red particle |
| **Shift + Left Click** | **Add neon pink particle** |
| Right Click | Add blue particle |
| Middle Click | Add green particle |
| F1-F12 | Select interaction parameter |
| Numpad +/- | Adjust selected parameter |
| =/- keys | Adjust selected parameter |

## Presets

1. **Red and Blue Orbitals**: Particles start in separate regions
2. **Grid Formation**: Particles arranged in a checkerboard pattern
3. **Random Distribution**: 2000 particles randomly placed
4. **Orbiting System**: Green center with red/blue orbiting particles
5. **High-Density Formation**: 8000 particles with complex interactions
6. **Neon Pink Ecosystem**: Features all particle types with custom interactions

## Installation

1. Ensure you have [Rust installed](https://www.rust-lang.org/tools/install)
2. Clone this repository:
   ```bash
   git clone https://github.com/matias-miller/particle-life-simulator.git
   ```
3. Run the simulation:
   ```bash
   cd particle-life-simulator
   cargo run --release
   ```

## Configuration

Edit these files to modify simulation parameters:
- `src/simulation/interaction_matrix.rs`: Particle interaction strengths
- `src/simulation/mod.rs`: Preset configurations
- `src/particle/mod.rs`: Particle physical properties

## Development Roadmap

- [ ] UI for customizable particle life simulations
- [ ] Add 4 more particle colors
- [ ] Implement GPU acceleration
- [ ] Add recording/playback functionality
- [ ] Create web version using WebAssembly
- [ ] Add particle trails visualization

## License

MIT License - See [LICENSE](LICENSE) for details

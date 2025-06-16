# Hammurabi

A terminal-based recreation of the classic 1968 text game, where you rule ancient Babylon as Hammurabi. Manage resources, feed your people, and grow your kingdom through strategic decision-making.

## About

Hammurabi is one of the earliest computer games, originally written by Doug Dyment in 1968. This version brings the classic gameplay to modern terminals with a text user interface (TUI) built using Ratatui.

In this game, you must:
- Manage grain stores to feed your population
- Buy and sell land based on market prices
- Plant crops and hope for good harvests
- Survive random events like plagues and rat infestations
- Rule for 10 years and receive a performance evaluation

## Installation

Ensure you have Rust installed, then:

```bash
git clone https://github.com/yourusername/hammurabi
cd hammurabi
cargo build --release
```

## Usage

Run the game with:
```bash
cargo run
```

For deterministic gameplay (useful for testing or competitions), provide a seed:
```bash
cargo run -- --seed 12345
```

## Game Rules

### Resources
- **Population**: Your citizens who need food and work the land
- **Land**: Measured in acres, used for planting grain
- **Grain**: Measured in bushels, used for food, planting, and trading

### Each Year
1. **Status Report**: View your current resources and last year's events
2. **Land Trading**: Buy or sell land at the current market price
3. **Planting**: Decide how many acres to plant (limited by grain and workers)
4. **Feeding**: Allocate grain to feed your population

### Constraints
- Each person needs 20 bushels of grain per year to survive
- Each person can work up to 10 acres of land
- Planting requires 1 bushel of grain per acre
- Land prices fluctuate between 17-26 bushels per acre

### Random Events
- **Harvest Yield**: 1-5 bushels per planted acre
- **Rats**: May eat 10-30% of stored grain
- **Plague**: Can kill up to half your population

### Win/Lose Conditions
- **Immediate Loss**: >45% of population starves in one year
- **Performance Rating**: After 10 years, evaluated on:
  - Population growth
  - Land per person
  - Total starvation percentage

## Controls

- **Arrow Keys**: Navigate between input fields
- **Enter**: Confirm input
- **Esc**: Exit game
- **Numbers**: Enter values for decisions

## Architecture

The game follows an event-driven architecture with clear separation of concerns:

- **State Management**: Central game state tracks all resources and history
- **Event Loop**: Handles user input and updates game state
- **UI Rendering**: Modular widgets display game information
- **Game Logic**: Deterministic calculations with seedable randomness

## Development

Built with:
- [Ratatui](https://ratatui.rs/) - Terminal UI framework
- [Crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation
- [Rand](https://rust-random.github.io/book/) - Random number generation
- [Clap](https://github.com/clap-rs/clap) - Command-line argument parsing

## License

This project is open source and available under the MIT License.
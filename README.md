# 👑 Hammurabi

A modern terminal-based recreation of the classic 1968 BASIC game "Hammurabi" (originally "The Sumer Game"), written in Rust with a beautiful TUI interface.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Terminal](https://img.shields.io/badge/Terminal-4D4D4D?style=for-the-badge&logo=windows-terminal&logoColor=white)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 🏛️ About

Step into the sandals of ancient Babylon's mighty ruler! As Hammurabi, you must wisely manage your kingdom's resources through ten years of tribulation. Buy and sell land, plant crops, and feed your people while navigating the unpredictable forces of nature.

This implementation features:
- 🎨 Beautiful terminal UI with responsive design
- 🎲 Deterministic gameplay with optional seeded randomness
- 📜 Authentic old English narrative style
- 🌾 Classic resource management mechanics
- 🏆 Performance evaluation after your reign

## 📸 Screenshots

![Hammurabi Splash Screen](https://raw.githubusercontent.com/stjepangolemac/hammurabi/main/screenshots/splash.png)

## 🚀 Installation

### From crates.io (Coming Soon)

```bash
cargo install hammurabi
```

### From source

```bash
git clone https://github.com/stjepangolemac/hammurabi.git
cd hammurabi
cargo install --path .
```

### Development build

```bash
git clone https://github.com/stjepangolemac/hammurabi.git
cd hammurabi
cargo build --release
./target/release/hammurabi
```

## 🎮 How to Play

### Starting the Game

```bash
hammurabi              # Start with random seed
hammurabi --seed 42    # Start with specific seed for reproducible gameplay
```

### Gameplay

You rule for 10 years, making three crucial decisions each year:

1. **Land Management** 🏛️
   - Buy or sell land (prices fluctuate between 17-26 bushels per acre)
   - More land = more potential crops
   - Enter negative numbers to sell

2. **Food Distribution** 🍞
   - Each person needs 20 bushels per year to survive
   - Starving people will die!
   - Dead citizens can't work your fields

3. **Crop Planting** 🌾
   - Each acre requires 1 bushel of grain for seed
   - Each person can work up to 10 acres
   - Harvest yields vary from 1-5 bushels per acre

### Random Events

- 🌾 **Harvests**: Yields vary based on weather conditions
- 🐀 **Rats**: May eat 10-30% of your stored grain
- 💀 **Plague**: Has a 15% chance to kill half your population
- 👥 **Immigration**: New citizens arrive if you govern well

### Winning

After 10 years, your performance is evaluated based on:
- Average starvation rate
- Population growth
- Land per citizen

Achieve greatness and be remembered alongside history's finest leaders!

## 🛠️ Features

- **Responsive Design**: Adapts to terminal sizes (optimized for 80x24 and larger)
- **Message Variety**: 10 unique variations for each game message
- **Historical Atmosphere**: Old English style text and period-appropriate language
- **Clean UI**: Minimalist design focusing on readability
- **Deterministic Randomness**: Use seeds for speedrun competitions or debugging

## 📋 Requirements

- Rust 1.70 or higher
- Terminal with UTF-8 support
- Minimum terminal size: 40x20 (recommended: 80x24)

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🎯 Roadmap

- [ ] Add difficulty levels
- [ ] Implement save/load functionality
- [ ] Add achievements system
- [ ] Create web assembly version
- [ ] Add sound effects (optional)
- [ ] Localization support

## 🙏 Acknowledgments

- Original BASIC game by Doug Dyment (1968)
- Inspired by David Ahl's version in "BASIC Computer Games" (1978)
- Built with [Ratatui](https://github.com/ratatui-org/ratatui) TUI framework
- ASCII art generated with [tui-big-text](https://github.com/joshka/tui-big-text)

## 📚 Historical Note

The original Hammurabi game was one of the first strategy/resource management games ever created. It was written in BASIC in 1968 and became widely popular after being published in David Ahl's "BASIC Computer Games" book. This implementation stays true to the original mechanics while providing a modern, polished terminal experience.

---

<p align="center">
  Made with ❤️ and Rust
</p>
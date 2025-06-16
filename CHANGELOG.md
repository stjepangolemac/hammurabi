# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of Hammurabi terminal game
- Beautiful TUI interface with responsive design
- Splash screen with ASCII art title
- Instructions screen for new players
- Old English narrative style throughout
- 10 message variations for each game event
- Deterministic gameplay with optional seed support
- Command-line interface with `--seed` option
- Comprehensive game state tracking
- Year-end performance evaluation
- Support for terminals as small as 40x20
- Adaptive UI that adjusts to terminal size

### Game Features
- Classic resource management mechanics
- Land trading with fluctuating prices (17-26 bushels/acre)
- Crop planting system with worker limitations
- Population feeding requirements (20 bushels/person/year)
- Random events: harvests, rats, plague, immigration
- Win/loss conditions based on performance
- Historical scoring system comparing to great leaders

### Technical Features
- Built with Ratatui TUI framework
- Cross-platform terminal support via Crossterm
- Modular architecture with clean separation of concerns
- Seedable random number generation for reproducible games
- Efficient event-driven state management
- Comprehensive error handling
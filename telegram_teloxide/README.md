# Telegram Bot with Inline Keyboard and Basic Command features

## Table of Contents

- [About](#about)
- [Getting Started](#getting_started)
- [Usage](#usage)
- [Contributing](../CONTRIBUTING.md)

## About <a name = "about"></a>

This project is a Telegram bot built using Rust and `teloxide 0.13`. The bot responds to user commands with an inline keyboard, allowing users to interact with various options. It also fetches cryptocurrency data when specific options are selected.

## Features

- **Command-based Interaction**: The bot responds to commands like `/start`, `/help`, and `/menu`.
- **Inline Keyboard**: Provides users with clickable options.
- **Callback Query Handling**: Processes user actions when they click on inline keyboard buttons.
- **Cryptocurrency Data Retrieval**: Fetches real-time cryptocurrency data when specific options are chosen.
- **Logging**: Uses `pretty_env_logger` for detailed logging.

## Commands

- `/start` - Displays a welcome message.
- `/help` - Shows available commands.
- `/menu` - Displays a menu with inline keyboard options.

### Next Features/Ideas:

- Addd functional and meaningful menu
- Persistent data storage for users will provide better user experience.
- Fetching data from API Endoint

# nplayers.ini

## Overview

The `nplayers.ini` file format is used to define configurations related to the number of players and game types for various ROMs in the MAME ecosystem. This file plays a crucial role in identifying how many players can participate in each game, as well as the type of gameplay supported (e.g., simultaneous, alternate).

## Structure

The `nplayers.ini` file is organized into a single section `[NPlayers]`, where each entry corresponds to a specific ROM and its associated player count or game type. Each line in this section follows a simple key-value format:

### Format

```ini
ROM_Name=Player_Count_Or_Game_Type
```

Where:

- **`ROM_Name`**: The name of the ROM file.
- **`Player_Count_Or_Game_Type`**: Describes the number of players or the type of game associated with the ROM. Can have multiple values separated by `/`.

### Possible Values

The possible values for `Player_Count_Or_Game_Type` include:

- **`1P`**: Single-player game.
- **`2P alt`**: Alternate two-player mode.
- **`2P sim`**: Simultaneous two-player mode.
- **`3P sim`**: Simultaneous three-player mode.
- **`3P alt`**: Alternate three-player mode.
- **`4P alt`**: Alternate four-player mode.
- **`4P sim`**: Simultaneous four-player mode.
- **`5P alt`**: Alternate five-player mode.
- **`6P alt`**: Alternate six-player mode.
- **`6P sim`**: Simultaneous six-player mode.
- **`8P alt`**: Alternate eight-player mode.
- **`9P alt`**: Alternate nine-player mode.
- **`Pinball`**: Pinball game.
- **`BIOS`**: BIOS or system ROM.
- **`Device`**: Non-playable device.
- **`Non-arcade`**: Non-arcade game.
- **`???`**: Unknown or unspecified number of players.

### Comments and Section Headers

- Lines that start with `[` or `;`, or are empty, are treated as comments or section headers and are ignored during parsing.

## Example

Here is an example of how a typical `nplayers.ini` entry might look:

```ini
[pacman]
Players=1P

[street_fighter]
Players=2P sim

[pinball_game]
Players=Pinball
```

In this example:

- `pacman` supports only a single player (`1P`).
- `street_fighter` supports two players simultaneously (`2P sim`).
- `pinball_game` is categorized as a pinball game (`Pinball`).

## Usage

The `nplayers.ini` file is typically used by MAME-related tools and applications to determine player configurations for different ROMs. This information is essential for understanding the multiplayer capabilities of arcade games and for configuring multiplayer setups in emulation environments.

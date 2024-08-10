# catver.ini

## Overview

The `catver.ini` file is used in the MAME system to classify games by category and subcategory. It provides a structured way to organize and manage game information, making it easier for users to sort and categorize their ROM collections. This file is particularly useful for identifying mature content within games.

## Structure

The `catver.ini` file is organized into lines, where each line corresponds to a game entry with its category and subcategory. Additionally, it may contain and optional attribute indicating if the content is mature.

### Game Entries

Each game entry follows the format:

```ini
`<ROM Name>=<Category> / <Subcategory> * Mature *`
```

Where:

- **`<ROM Name>`**: The name of the ROM being configured.
- **`<Category>`**: The category of the game.
- **`<Subcategory>`**: The subcategory of the game. This may be followed by `* Mature *` if the game is marked as mature content.

### Important Notes

- The `category` and `subcategory` are separated by `/`.
- The `* Mature *` marker is optional and indicates that the game is considered mature content.
- Lines that start with `[` or are empty are treated as section headers or comments and are ignored during processing.

## Example

Here is an example of how a typical `catver.ini` entry might look:

```ini
pacman=Maze / Classic
street_fighter=Fighting / Versus
night_trap=Interactive Movie / Horror * Mature *
```

In this example:

- `pacman` is classified under the "Maze" category with a "Classic" subcategory.
- `street_fighter` is classified under the "Fighting" category with a "Versus" subcategory.
- `night_trap` is classified under the "Interactive Movie" category with a "Horror" subcategory and is marked as mature content.

## Usage

The `catver.ini` file is typically used by MAME-related tools and front-ends to help organize and categorize games. It allows users to filter and sort games based on category, subcategory, and maturity rating, making it easier to manage large collections of ROMs.

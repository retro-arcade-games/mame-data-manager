# series.ini

## Overview

The `series.ini` file is used to organize and associate different ROMs with specific game series in the MAME system. The file is structured into sections, each representing a game series, and within each section, the ROMs associated with that series are listed.

## Structure

The `series.ini` file is organized into sections, where each section corresponds to a specific game series. Additionally, there are sections for folder settings and a placeholder section for root folder configurations.

### Series Sections

Each game series is represented by a section header labeled with the series identifier, followed by entries that list the ROMs associated with that series.

- **`[<Series>]`**: This is the section header, where `<Series>` is the identifier for the game series (e.g., `[Street Fighter]`, `[Super Mario]`).
- **Entries**: Each entry under a series section is the name of a ROM associated with that specific game series.

## Example

Here is an example of how a typical `series.ini` entry might look:

```ini
[Street Fighter]
street_fighter
street_fighter_ii

[Super Mario]
super_mario_bros
super_mario_bros_3
```

In this example:

- The `[Street Fighter]` section lists `street_fighter` and `street_fighter_ii` as ROMs associated with the Street Fighter series.
- The `[Super Mario]` section lists `super_mario_bros` and `super_mario_bros_3` as ROMs associated with the Super Mario series.

## Usage

The `series.ini` file is typically used by MAME-related tools and front-ends to help organize and categorize ROMs based on their associated game series. This allows users to filter and select games by series, enhancing the usability of large ROM collections.

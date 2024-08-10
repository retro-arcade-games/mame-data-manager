# languages.ini

## Overview

The `languages.ini` file is used to organize and associate different ROMs with specific languages in the MAME system. The file is structured into sections, each representing a language, and within each section, the ROMs associated with that language are listed.

## Structure

The `languages.ini` file is organized into sections, where each section corresponds to a specific language.

### Sections and Format

#### Language Sections

Each language is represented by a section header labeled with the language identifier, followed by entries that list the ROMs associated with that language.

- **`[<Language>]`**: This is the section header, where `<Language>` is the identifier for the language (e.g., `[English]`, `[Spanish]`).
- **Entries**: Each entry under a language section is the name of a ROM associated with that specific language.

## Example

Here is an example of how a typical `languages.ini` entry might look:

```ini
[English]
pacman
space_invaders

[Spanish]
pacman
la_abadia_del_crimen
```

In this example:

- The `[English]` section lists `pacman` and `space_invaders` as ROMs associated with the English language.
- The `[Spanish]` section lists `pacman` and `la_abadia_del_crimen` as ROMs associated with the Spanish language.

## Usage

The `languages.ini` file is typically used by MAME-related tools and front-ends to help organize and categorize ROMs based on their associated languages. This allows users to filter and select games by language, enhancing the usability of multi-language ROM collections.

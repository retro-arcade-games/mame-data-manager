# history.xml

## Overview

The `history.xml` file is used to store detailed historical information about arcade games within the MAME system. It organizes data about the systems that run the game, related software, and various textual sections that describe different aspects of the game. This file is structured in XML format, making it easy to parse and extract specific information.

## Structure

The XML file follows this general structure:

### `<entry>`

Each game is represented by an `<entry>` element, which contains several sub-elements that provide detailed information about the game.

### `<systems>`

The `<systems>` section lists the systems that can run the game. Each system is represented by a `<system>` element with the following attribute:

- **`name`**: The name of the system that can run the game.

### `<software>`

The `<software>` section provides information about software related to the game. Each piece of software is represented by an `<item>` element with the following attributes:

- **`list`**: The name of the software list to which this item belongs.
- **`name`**: The name of the software.

_Note: The software tag is not used for parsing._

### `<text>`

The `<text>` section contains various sections of text about the game. These sections are embedded directly within the `<text>` element as plain text, using headings and formatted text to separate different types of information. The possible sections include:

- **DESCRIPTION**: Provides a general description of the game.
- **TECHNICAL**: Details technical aspects or specifications of the game.
- **TRIVIA**: Contains trivia or interesting facts about the game.
- **UPDATES**: Lists updates or changes made to the game.
- **SCORING**: Details on scoring or how the game is scored.
- **TIPS AND TRICKS**: Offers tips and tricks for playing the game.
- **SERIES**: Information about the game series or franchise.
- **STAFF**: Lists the staff or developers involved with the game.
- **PORTS**: Details on different ports or versions of the game.
- **CONTRIBUTE**: Information on how to contribute or support the game.

## Example

Here is a complete example of how a typical `history.xml` entry might look:

```xml
<entry>
    <systems>
        <system name="mk" />
        <system name="mkprot4" />
    </systems>
    <software>
        <item list="vgmplay" name="mk" />
    </software>
    <text>
Arcade Video game published 32...

- TECHNICAL -

Mortal Kombat arcade runs ...

- TRIVIA -

Mortal Kombat was released...

- UPDATES -

PROTO 4.0 (July 14, ...

- SCORING -

* Basic Move...

- TIPS AND TRICKS -

* Secret EJB MENU:...

- SERIES -

1. Mortal Kombat ...

- STAFF -

Design and software: ...

- PORTS -

* CONSOLES:...

- CONTRIBUTE -

Edit this entry: ...
</text>
</entry>
```

## Usage

The `history.xml` file is typically used by MAME-related tools and applications to provide detailed background information, technical specifications, and other relevant details about each game. This information enhances the user's experience by offering insights into the history and development of arcade games, as well as tips and trivia that add context to the gameplay.

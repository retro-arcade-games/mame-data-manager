# resources.dat

## Overview

The resources file (`pS_AllProject....dat`) is used to manage and organize various resources associated with arcade machines within an emulation environment. These resources include artwork, snapshots, and other media files that enhance the emulation experience. The file is structured into `machine` elements, each representing a group of resources related to a specific arcade machine.

## Structure

The resources file is organized into elements, where each `machine` element represents a specific resource grouping. The structure includes attributes and child nodes that describe the resources associated with each machine.

### Machine

Each `machine` element represents a resource group associated with a specific arcade machine. The `machine` element has the following components:

- **`name`**: The unique identifier for the machine or resource group (attribute).

  - Possible values include: `artpreview`, `bosses`, `cabinets`, `covers`, `cpanel`, `devices`, `ends`, `flyers`, `gameover`, `howto`, `icons`, `logo`, `manuals`, `marquees`, `pcb`, `scores`, `select`, `snap`, `titles`, `versus`, `videosnaps`, `warning`.

- **`description`**: A textual description of the resource group (child node).

- **`roms`**: A collection of `rom` elements, each representing a specific resource file associated with the machine (child nodes).
  - Each `rom` element has the following attributes:
    - **`name`**: The name of the resource file, including the file path (e.g., `artpreview\005.png`).
    - **`size`**: The size of the resource file in bytes.
    - **`crc`**: The CRC32 checksum of the resource file, used for integrity verification.
    - **`sha1`**: The SHA1 hash of the resource file, providing a more secure integrity check.

## Example

Here is an example of how a typical `resources.dat` entry might look:

```xml
<machine name="artpreview">
    <description>artpreview</description>
    <roms>
        <rom name="artpreview\pacman.png" size="57355" crc="2fd366c4" sha1="079174056f520cb9cecd9b0d4f12ece47db32982"/>
    </roms>
</machine>
```

In this example:

- The `machine` element is identified by the `name` attribute "artpreview".
- The `description` node provides a brief description of the resources, such as "artpreview".
- The `roms` node contains individual `rom` elements, each detailing a resource file associated artpreview, including its name, size, CRC32 checksum, and SHA1 hash.

## Usage

The `resources.dat` file is typically used in MAME-related tools and emulation environments to organize and reference additional content that can be associated with arcade machines. This structure makes it easier to manage large collections of resources, ensuring that artwork, snapshots, and other media are correctly linked to their respective machines.

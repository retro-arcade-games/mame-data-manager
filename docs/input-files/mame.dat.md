# mame.dat

## Overview

The `mame.dat` file format represents data about arcade machines and their components. It is a critical file used by MAME (Multiple Arcade Machine Emulator) to understand the details of each arcade machine, including its ROMs, devices, and other associated elements.

This document outlines the structure used for parsing the `mame.dat` file, detailing the key elements and attributes.

## Structure

### Machine

Represents a single arcade machine with various attributes:

- **`name`**: The unique identifier for the machine (attribute).
- **`source_file`**: Optional source file for the machine's data (attribute).
- **`rom_of`**: Optional; the ROM depends on files from another ROM to function correctly (attribute).
- **`clone_of`**: Optional; the ROM is a modified version or variant of another ROM known as the parent ROM (attribute).
- **`is_bios`**: Optional flag indicating if the machine is a BIOS (attribute).
- **`is_device`**: Optional flag indicating if the machine is a device (attribute).
- **`runnable`**: Optional flag indicating if the machine is runnable (attribute).
- **`is_mechanical`**: Optional flag indicating if the machine is mechanical (attribute).
- **`sample_of`**: Optional; the ROM uses specific sound samples from another ROM (attribute).

### Description

- **`description`**: Optional textual description of the machine (child node).

### Year

- **`year`**: Optional year of release (child node).

### Manufacturer

- **`manufacturer`**: Optional manufacturer name (child node).

### BIOS Sets

Optional list of BIOS sets related to the machine (child nodes):

- **`biosset`**:
  - **`name`**: Name of the BIOS set (attribute).
  - **`description`**: Description of the BIOS set (attribute).

### ROMs

Optional list of ROMs associated with the machine (child nodes):

- **`rom`**:
  - **`name`**: Name of the ROM (attribute).
  - **`size`**: Size of the ROM (attribute).
  - **`merge`**: Optional merge attribute (attribute).
  - **`status`**: Optional status attribute (attribute).
  - **`crc`**: Optional CRC value (attribute).
  - **`sha1`**: Optional SHA1 value (attribute).

### Device References

Optional list of device references related to the machine (child nodes):

- **`device_ref`**:
  - **`name`**: Name of the device reference (attribute).

### Software Lists

Optional list of software associated with the machine (child nodes):

- **`softwarelist`**:
  - **`name`**: Name of the software (attribute).

### Samples

Optional list of samples associated with the machine (child nodes):

- **`sample`**:
  - **`name`**: Name of the sample (attribute).

### Driver Status

- **`driver_status`**: Optional status of the machine's driver (child node).

### Disks

Optional list of disks related to the machine (child nodes):

- **`disk`**:
  - **`name`**: Name of the disk (attribute).
  - **`sha1`**: Optional SHA1 value (attribute).
  - **`merge`**: Optional merge attribute (attribute).
  - **`status`**: Optional status attribute (attribute).
  - **`region`**: Optional region attribute (attribute).

## Example

Here’s an example of a `mame.dat` entry for a machine:

```xml
<machine name="pacman">
  <description>Pac-Man (Midway)</description>
  <year>1980</year>
  <manufacturer>Midway</manufacturer>
  <rom name="pacman.6e" size="4096" crc="c1e6ab10" sha1="72bc456e3d0a0ee7d6e3a14745f5f2b8ec51edcc"/>
  <biosset name="bios1" description="Main BIOS"/>
  <disk name="pacman_disk" sha1="72bc456e3d0a0ee7d6e3a14745f5f2b8ec51edcc" region="USA"/>
</machine>
```

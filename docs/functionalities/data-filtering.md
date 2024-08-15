# Data Filtering

## Overview

The data filtering module in MAME Data Manager allows you to remove machines that are not relevant to your needs. This is useful to clean up your data set, focusing only on the machines that matter for arcade game management.

## Filtering Options

The filtering process is divided into multiple options to give you control over which machines to remove:

### 1. **Remove machines with non-game categories**

This option removes machines that belong to categories unrelated to arcade games, such as "Calculator," "Slot Machine," "Medical Equipment," and others. The predefined list of categories to ignore ensures that only relevant arcade machines remain in your data set.

### 2. **Remove device machines**

This filter removes machines classified as devices. Devices are non-playable components used by other machines, and typically include things like input/output controllers or other hardware support modules.

### 3. **Remove BIOS machines**

This filter removes machines that are classified as BIOS systems, which are essential for some setups but are not directly playable.

### 4. **Remove mechanical machines**

This option filters out mechanical machines, such as pinball machines, which do not fit the target data set.

### 5. **Remove modified machines**

The "modified machines" filter removes machines identified as bootlegs, prototypes, or systems with invalid manufacturers. The application analyzes machine descriptions and other metadata to detect these entries.

### 6. **Remove clones**

This filter excludes machines that are clones, meaning they are duplicates or slight variations of original machines.

### 7. **Remove ALL non-game machines (apply all machine filters)**

This comprehensive filter applies all the filtering options described above, removing all non-game-related machines in one step.

## How It Works

The filtering process is performed in two main phases:

1. **Identifying Machines to Remove**: The application iterates over the data and checks if each machine matches the criteria specified by the active filter(s).
2. **Removing Machines**: Once the machines to be removed are identified, they are deleted from the data collection. The application updates the progress bar to provide feedback during this process.

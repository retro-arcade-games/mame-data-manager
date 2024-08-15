# Statistics

## Overview

The statistics module in MAME Data Manager provides insights into various aspects of the data processed by the application. All statistics are generated from data stored in memory, ensuring that the information reflects the current state of the dataset after filtering and modifications.

## Available Statistics

The following statistics can be accessed through the menu:

### 1. **General Stats**

Displays an overview of the main data points, including:

- Total machines
- Total original machines
- Total clones
- Number of manufacturers
- Number of categories
- Number of subcategories
- Number of series
- Number of languages
- Player information details
- Machines with history sections
- Machines with associated resources

### 2. **Top 10 Categories**

Shows the top 10 most frequent categories among the machines, providing insight into the dominant types of games or systems.

### 3. **Top 10 Subcategories**

Displays the top 10 subcategories, which give more granular details about the classification of the machines.

### 4. **Top 10 Manufacturers**

Lists the top 10 manufacturers based on the number of machines they produced, highlighting the most prominent creators in the dataset.

### 5. **Top 10 Series**

Shows the top 10 game series by the number of entries in the dataset.

### 6. **Top 10 Languages**

Provides a list of the top 10 languages found across the machines.

### 7. **Top 10 Player Configurations**

Displays the top 10 configurations for player information, such as single-player, multiplayer, and their respective frequencies.

## How It Works

The statistics are calculated dynamically based on the data in memory. This approach allows for real-time updates as data is filtered or modified. The statistics module groups and sorts the data to present the most relevant insights in a user-friendly format.

# Data Exporting

This section explains how to export the data from MAME Data Manager into different formats like SQLite, JSON, and CSV.

## Overview

The data exporting feature in MAME Data Manager allows you to save the processed data in various formats suitable for different use cases. You can choose to export the data as a structured database (SQLite), a collection of JSON files, or a set of CSV files. Each format has its own benefits depending on the context in which you want to use the data.

### Export Options

- **Export to SQLite**: The data is stored in a relational SQLite database, with tables that include machines, categories, manufacturers, ROMs, BIOS sets, and more. This format is ideal if you plan to run queries or integrate with other applications that require relational data.

- **Export to JSON**: The data is exported as a collection of JSON files. Each entity like machines, manufacturers, categories, and subcategories is stored in its own JSON file. This format is useful if you need to work with structured data in applications or environments that prefer JSON.

- **Export to CSV**: The data is saved as a collection of CSV files, which can be easily opened and manipulated in spreadsheet applications like Excel or Google Sheets. Each entity is stored in its own CSV file, providing flexibility for data analysis and reporting.

### How It Works

The export process involves multiple steps, depending on the format chosen:

1. **SQLite Export**: The application creates an SQLite database and sets up the necessary tables. It then inserts the machines data along with related entities such as ROMs, BIOS sets, and categories. The application also creates relationships between machines and other entities like languages and players.

2. **JSON Export**: The application generates a series of JSON files, each representing different parts of the dataset. For example, `machines.json` contains all the machines, while `categories.json` and `manufacturers.json` provide additional information. Each file is structured to allow easy navigation and linkage between related data.

3. **CSV Export**: The application generates several CSV files, each representing different entities like machines, ROMs, BIOS sets, and more. The files are designed for easy import into spreadsheet applications. The application also supports exporting collections such as manufacturers, series, and languages into separate CSV files.

### Example of Exported Data

Here’s a breakdown of how the data is organized in each export format:

- **SQLite**:

  - Tables include: `machines`, `roms`, `bios_sets`, `categories`, `languages`, `manufacturers`, `series`, and more.
  - Relationships between tables allow for complex queries and filtering.

- **JSON**:

  - Files include: `machines.json`, `categories.json`, `manufacturers.json`, `languages.json`, `series.json`, `subcategories.json`, and more.
  - JSON objects are nested where necessary, allowing for easy traversal.

- **CSV**:
  - Files include: `machines.csv`, `roms.csv`, `bios_sets.csv`, `categories.csv`, `languages.csv`, `manufacturers.csv`, `series.csv`, and more.
  - Each row represents a specific record, with headers indicating the fields.

### Additional Information

For more details about the schemas used in the application and the structure of the exported data, visit the [Schemas](../schemas/README.md) section.

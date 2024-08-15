# File Reading

## Overview

Once the files are downloaded from the sources and extracted, the application proceeds to read them one by one. During this process, the application stores the data in normalized structures in memory, ensuring that the information remains as close as possible to the original sources.

## How It Works

- **Reading Process**: The application reads each file sequentially, extracting the relevant data and storing it in memory. The focus is on maintaining the data as it is in the original sources without alterations.
- **Data Storage**: The extracted information is kept in normalized structures, allowing for efficient management and easy access. These structures are designed to reflect the format of the source data closely.

- **Extended Data**: Any modifications or additional information are stored in a separate structure called `extended data`. This approach ensures that the original data remains intact, while any enhancements or changes are isolated for later use. Examples of `extended data` include normalized names for machines (`name`) and normalized names for manufacturers (`manufacturer`), among others.

This methodology allows for flexibility in working with both the original data and any modified or extended versions without compromising the integrity of the source information.

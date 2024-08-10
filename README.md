<h2 align="center">MAME Data Manager</h2>

<p align="center">Download, read, manage and export MAME information</p>

<p align="center">
  <a href="#about">About</a> •
  <a href="#why">Why</a> •
  <a href="#running-the-application">Running the Application</a> •
  <a href="#how-to-use">How to Use</a> •
  <a href="#documentation">Documentation</a> •
  <a href="#credits">Credits</a> •
  <a href="#contribute">Contribute</a> •
  <a href="#license">License</a>
</p>

## About

MAME Data Manager is a tool designed to help manage and manipulate data related to MAME (Multiple Arcade Machine Emulator). It allows users to efficiently handle large datasets, extract relevant information, and generate useful reports or export data in various formats.

## Why

Managing MAME information can be challenging because the data is scattered across various files, each with its own format and structure. This fragmentation makes it difficult for users to work with the data effectively. MAME Data Manager aims to solve this problem by providing a tool that automatically downloads the latest versions of these files and consolidates all the information into a single, unified database. This allows users to access and manipulate MAME data more easily and efficiently.

## Running the application

MAME Data Manager is a Rust-based application that allows you to manage and manipulate MAME-related data. Below are the instructions for running the application by either downloading the binary or running it locally from the source code.

### Option 1: Download the Binary

1. **Download the Binary**:

   - Visit the [releases section](https://github.com/retro-arcade-games/mame-data-manager/releases) in this repository.
   - Download the latest version of the binary for your operating system (Windows, macOS, Linux).

2. **Run the Application**:
   - Navigate to the folder where you downloaded the binary.
   - On Windows:
     - Double-click the downloaded file or run it from the command line:
       ```sh
       ./mame-data-manager.exe
       ```
   - On macOS/Linux:
     - Ensure the file has execution permissions and then run it:
       ```sh
       chmod +x mame-data-manager
       ./mame-data-manager
       ```

### Option 2: Clone the Repository and Run Locally

1. **Clone the Repository**:

   - Open your terminal and clone this repository using Git:
     ```sh
     git clone https://github.com/retro-arcade-games/mame-data-manager.git
     cd mame-data-manager
     ```

2. **Build the Project**:

   - Make sure you have Rust installed on your system. If not, you can install it by following the instructions at [rust-lang.org](https://www.rust-lang.org/).
   - Build the project by running:
     ```sh
     cargo build --release
     ```

3. **Run the Application**:
   - After building, the binary will be available in the `target/release/` directory.
   - Run the application from that location:
     ```sh
     ./target/release/mame-data-manager
     ```

## How to Use

Once the application is running, an interactive menu will appear in the terminal. Use the arrow keys to navigate through the menu and select the desired options.

The menu options include:

TODO

Simply follow the on-screen instructions to perform the desired actions.

## Documentation

More detailed documentation about the project can be found [here](./docs/README.md)

## Credits

MAME Data Manager wouldn't be possible without the invaluable contributions and resources provided by the following individuals and communities:

- **The MAME Community**: A special thanks to the entire MAME community for their continuous efforts in preserving arcade history and making it accessible to everyone. Your work is the foundation upon which this project is built.

- **AntoPISA and Progetto-SNAPS**: AntoPISA's [Progetto-SNAPS](https://www.progettosnaps.net) project has been an essential resource for MAME artwork and other assets. Thank you for your dedication and hard work in creating and maintaining this incredible resource.

- **Motoschifo and Arcade Database (ADB)**: Motoschifo's [Arcade Database](http://adb.arcadeitalia.net) is a comprehensive resource for MAME data, providing detailed information about arcade games and machines.

- **Arcade-History**: The team behind [Arcade-History](https://www.arcade-history.com) has done an amazing job in documenting the history of arcade games.

- **NPlayers Team**: The [NPlayers](https://nplayers.arcadebelgium.be) project by Arcade Belgium is a fantastic resource for information on multiplayer arcade games.

- **@zombiesbyte and XMLTractor**: Special thanks to zombiesbyte for [XMLTractor](https://github.com/zombiesbyte/xmltractor) project.

## Contribute

Contributions are welcome! If you'd like to contribute, please fork the repository, create a new branch, and submit a pull request. Make sure to follow the project's coding guidelines and include tests where applicable.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

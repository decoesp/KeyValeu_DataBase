Key-Value Store
===============

This is a simple key-value store implementation in Rust. It provides basic functionality to store, retrieve, and remove key-value pairs. The key-value pairs are stored in a HashMap and persisted to a file.

Usage
-----

To use this key-value store, follow the steps below:

### Prerequisites

Make sure you have [Rust](https://www.rust-lang.org/) and [Cargo](https://doc.rust-lang.org/cargo/) installed on your system.

### Installation

Clone the repository or download the source code files.

Copy code

`https://github.com/decoesp/KeyValeu_DataBase.git`

### Build

Open a terminal and navigate to the project directory. Build the project using Cargo:

`cargo build`

### Run

To run the key-value store application, use the following command:

`cargo run`

The application will start and present a prompt where you can enter commands.

### Commands

The following commands are available:

-   `get <key>`: Retrieves the value associated with the specified key.
-   `set <key> <value>`: Inserts a new key-value pair into the store.
-   `remove <key>`: Removes the specified key-value pair from the store.
-   `list`: Lists all key-value pairs in the store.
-   `clear`: Clears the store, removing all key-value pairs.
-   `exit`: Exits the application.

Note: Ensure that you provide the correct number of arguments for each command.

### Testing

To run the test suite, use the following command:

`cargo test`

The tests cover the basic functionality of the key-value store.

# ShellCommander

This is a simple TCP server-client application written in Rust that allows a user to execute commands on a remote client machine. The server listens for incoming connections and sends commands to the client, which executes them and returns the output.

## Table of Contents

- [Features](#features)
- [Requirements](#requirements)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#Contributing)

## Features

- TCP server that listens for client connections.
- Ability to send commands from the server to the client.
- The client executes commands and returns the output.
- Supports both Windows and Unix-like operating systems.

## Requirements

- Rust (1.50 or later)
- Cargo (comes with Rust)

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/creepcomp/ShellCommander.git
   cd ShellCommander
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

## Usage

### Running the Server

1. Navigate to the server directory (if the project is structured by directories, otherwise just run from the root):

   ```bash
   cd path/to/server
   ```

2. Run the server:

   ```bash
   cargo run
   ```

   The server will start listening on `0.0.0.0:443`. You can change the listening address in the source code if needed.

### Running the Client

1. Navigate to the client directory (if applicable):

   ```bash
   cd path/to/client
   ```

2. Run the client:

   ```bash
   cargo run
   ```

   The client will attempt to connect to the server at `127.0.0.1:443`. You can change the connection address in the source code if needed.

### Interacting with the Server

Once the client is connected, you can enter commands in the server terminal. The server will send these commands to the client, which will execute them and return the output.

- To exit the server, type `exit` and press Enter.

### Contributing
This is an open source project and contributions are welcome! Feel free to fork the repository, make improvements, and submit pull requests. Whether it's fixing bugs, adding features, improving documentation, or enhancing tests, your help is highly appreciated.
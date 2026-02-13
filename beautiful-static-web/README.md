# Beautiful Static Web on Freenet

This project demonstrates a static website hosted entirely on a Freenet contract.

## Project Structure

- `contracts/`: Contains the Rust source code for the smart contract that stores and serves the website.
- `web/`: The HTML and CSS files for the website.
- `Makefile`: Automates the build and deployment process.

## Prerequisites

- Rust and Cargo
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- Freenet Development Tool (`fdev`)
- A running Freenet local node (`freenet local`)

## How to Deploy

1. **Start Freenet Local Node**:
   Open a terminal and run:
   ```bash
   RUST_LOG=freenet=debug freenet local
   ```

2. **Build and Publish**:
   In this directory, run:
   ```bash
   make publish
   ```

3. **View the Website**:
   The `make publish` command will output a Contract Key (e.g., `8s7...`).
   Open your browser to:
   `http://127.0.0.1:7509/contract/<CONTRACT_KEY>/`

## How it Works

- The `simple-web-contract` logic is compiled to WebAssembly.
- The `web/` folder is compressed into a `.tar.xz` file.
- `fdev publish` uploads the WASM (logic) and the `.tar.xz` (state) to your local node.
- The command used is `fdev publish --code <WASM> --parameters "" contract --state <TAR>`.
- The Freenet kernel acts as a web server, using the contract to read the archive and serve the files.

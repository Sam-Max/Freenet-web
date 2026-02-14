# Secure Static Web on Freenet

This project deploys a secure, signature-gated static website on the Freenet network. Unlike a simple file upload, this architecture ensures that **only the owner** (holder of the private key) can update the website content.

## Project Structure

- **`web-container-contract/`**: The Rust smart contract that serves the website and verifies signatures on updates.
- **`delegates/web-delegate/`**: A local agent that securely manages your private key and signs updates.
- **`public/`**: Your static web assets (HTML, CSS, JS).
- **`signer/`**: A custom CLI tool for generating keys and signed metadata.
- **`Makefile.toml`**: The build automation script (uses `cargo-make`).

## Prerequisites

1.  **Rust & Cargo**: [Install Rust](https://www.rust-lang.org/tools/install)
2.  **WASM Target**: `rustup target add wasm32-unknown-unknown`
3.  **Freenet Dev Tool (`fdev`)**: `cargo install fdev`
4.  **Cargo Make**: `cargo install cargo-make`
5.  **Local Node**: A running Freenet node (`freenet local`).

## How to Deploy

1.  **Start your Freenet Node**:
    ```bash
    fdev local
    ```

2.  **Build and Publish**:
    This single command builds the contract, generates keys, signs your `public/` folder, and deploys everything:
    ```bash
    cargo make publish-public
    ```

3.  **Access Your Site**:
    The command output will show a **Contract Key** (e.g., `G85z...`).
    Open: `http://127.0.0.1:7509/v1/contract/web/<CONTRACT_KEY>/`

## Security Model

This project implements a "Owner-Signed" update model:

1.  **Initialization**:
    - A unique **Ed25519 Keypair** is generated.
    - The Contract is deployed with the **Public Key** as its parameter.
    - The initial content is signed with the **Private Key**.

2.  **The Contract (Gatekeeper)**:
    - Stores the Public Key.
    - On every update, it verifies the **Digital Signature** in the metadata against the Public Key.
    - If the signature is valid, the update is accepted. Otherwise, it is rejected.

3.  **The Delegate (Key Manager)**:
    - Runs locally on your node.
    - Safely holds the **Private Key** (never exposed to the network).
    - Signs new content when you request an update.

## Updating the Site

To update your website:
1.  Modify files in `public/`.
2.  Run `cargo make publish-public` again.
    - This will re-sign the new content and publish the update to your contract.

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

## How to Deploy

1.  **Start your Freenet Node**:
    ```bash
    freenet local
    ```

2.  **Choose your Deployment Target**:

    **Option A: Local Testing (Private)**
    Best for development. The site is only visible on your machine.
    ```bash
    cargo make deploy-local
    ```

    **Option B: Public Network (Real)**
    Propagates your contract to the Freenet network. Visible to anyone with the key.
    ```bash
    cargo make deploy-real
    ```

    *The deployment script will automatically check prerequisites, build your site, and print the **Contract URL** at the end.*

## Updating the Site & Versioning

To update your website, modify files in `public/`.

**Crucial:** You should increment the version number for every update so the network (and search engines) detect the change.

```bash
VERSION=2 cargo make deploy-real
```

If you don't specify `VERSION`, it defaults to `1`.

## Security Model

This project implements a "Owner-Signed" update model:

1.  **Initialization**:
    - A unique **Ed25519 Keypair** is generated in `build/`.
    - **WARNING**: Do not run `cargo make clean` unless you have backed up your keys! It will delete `build/` and you will lose control of your contract.

2.  **The Contract (Gatekeeper)**:
    - Stores the Public Key.
    - On every update, it verifies the **Digital Signature** in the metadata against the Public Key.

3.  **The Delegate (Key Manager)**:
    - Runs locally on your node.
    - Safely holds the **Private Key** (never exposed to the network).
    - Signs new content when you request an update.

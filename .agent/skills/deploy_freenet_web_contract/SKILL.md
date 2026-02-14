---
name: deploy_freenet_web_contract
description: Instructions and best practices for building and deploying a static website contract on Freenet, including critical state formatting rules.
---

# Deploy Freenet Web Contract

This skill guides you through the process of building and deploying a static website on Freenet using a Rust contract. It covers the specific requirements for the contract build configuration and the strict binary state format expected by the Freenet Gateway.

## 1. Project Structure

Ensure your project has the following structure:
```
my-freenet-site/
├── contracts/
│   └── simple-web-contract/
│       ├── Cargo.toml
│       ├── build.rs       <-- CRITICAL
│       └── src/lib.rs
├── web/                   <-- Your static site content (index.html, css, etc.)
└── Makefile               <-- Automation script
```

## 2. Contract Build Configuration

To ensure the correct WASM exports are generated, you MUST force the `freenet-main-contract` feature flag.

**contracts/simple-web-contract/build.rs**:
```rust
fn main() {
    println!("cargo:rustc-cfg=feature=\"freenet-main-contract\"");
}
```

**contracts/simple-web-contract/Cargo.toml**:
```toml
[package]
name = "simple-web-contract"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
freenet-stdlib = { version = "0.1", features = ["contract"] }
freenet-macros = "0.1"
serde = { version = "1.0", features = ["derive"] }
sha2 = "0.10"
```

## 3. State Format Requirements (Using --webapp-archive)

The Freenet Gateway expects the contract state to be a **tar archive** prefixed with a specific **16-byte binary header** (`[8 bytes Metadata Length] [8 bytes Content Length]`).

While this can be done manually, the **`fdev`** tool provides a specialized flag that handles this packaging automatically.

**The Idiomatic Way:**
Use the `--webapp-archive` flag in `fdev publish`.

## 4. Automation (Makefile)

Use this `Makefile` pattern to handle the build and packaging automatically. Notice how much simpler the `package` rule becomes when using the native flag.

```makefile
# Adjust paths as needed
CONTRACT_DIR = contracts/simple-web-contract
WEB_DIR = web
TARGET_DIR = $(CONTRACT_DIR)/target/wasm32-unknown-unknown/release
WASM_FILE = $(TARGET_DIR)/simple_web_contract.wasm
STATE_FILE = build/web-content.tar.xz

all: build package

build:
	@echo "Building contract..."
	cd $(CONTRACT_DIR) && cargo build --release --target wasm32-unknown-unknown

package:
	@echo "Packaging web content..."
	mkdir -p build
	# Create a reproducible XZ-compressed tar archive of the web content
	# This is now passed directly to fdev using --webapp-archive
	cd $(WEB_DIR) && tar -cJf ../$(STATE_FILE) *

publish: build package
	@echo "Publishing to local Freenet node..."
	# Usage: fdev publish --code <WASM> contract --webapp-archive <STATE_FILE>
	fdev publish --code $(WASM_FILE) contract --webapp-archive $(STATE_FILE)

publish-public: build package
	@echo "Publishing to PUBLIC Freenet network..."
	# Uses the --release flag for public network propagation
	fdev publish --release --code $(WASM_FILE) contract --webapp-archive $(STATE_FILE)
```

## 5. Deployment

1.  Start your local Freenet node:
    ```bash
    freenet local
    ```
2.  Run the publish command:
    ```bash
    make publish
    ```
3.  Access the site using the returned Contract Key:
    `http://127.0.0.1:7509/v1/contract/web/<CONTRACT_KEY>/`

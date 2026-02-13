.PHONY: all build package publish clean

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
	# Create a reproducible tar archive of the web content (XZ compressed)
	# This is now passed directly to fdev using --webapp-archive
	cd $(WEB_DIR) && tar -cJf ../$(STATE_FILE) *

publish: build package
	@echo "Publishing to local Freenet node..."
	# This requires fdev to be installed and freenet running in local mode
	echo -n "0" > build/params.bin
	fdev publish --code $(WASM_FILE) --parameters build/params.bin contract --webapp-archive $(STATE_FILE)

publish-public: build package
	@echo "Publishing to PUBLIC Freenet network..."
	# IMPORTANT: Ensure 'freenet' (not 'freenet local') is running!
	echo -n "0" > build/params.bin
	fdev network publish --code $(WASM_FILE) --parameters build/params.bin contract --webapp-archive $(STATE_FILE)

clean:
	rm -rf build
	cd $(CONTRACT_DIR) && cargo clean

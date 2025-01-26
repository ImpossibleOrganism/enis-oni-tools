# Eni's Oxygen Not Included Tools - Makefile

# === Phony Targets ============================================================
# I make these phony just in case someone makes a file with the same name
.PHONY: build run serve serve-angular serve-svelte clean echo
# These actually need to be phony targets because they have dependents
.PHONY: wasm

# === Variables ================================================================
# ----- Directories -----
# Directory where Cargo puts is build artifacts
CARGO_TARGET_DIR := ./target
# Directory for auto-generated WebAssembly and JavaScript bindings
GENERATED_WASM_DIR := ./javascript/generated
# Lightweight development server directory
DEV_SERVER_DIR := ./www
# Angular development server directory
ANGULAR_SERVER_DIR := ./www-angular
# Svelte development server directory
SVELTE_SERVER_DIR := ./www-svelte

# ----- File Collections -----
# All the .rs source files
RUST_FILES := $(shell find rust -name '*.rs') build.rs
# Data files sometimes built into the crate
DATA_FILES := $(shell find data -name '*')

# ----- Build Dependencies -----
# All source files for the crate
CRATE_SOURCE := $(RUST_FILES) $(DATA_FILES) Cargo.toml
# The generated WebAssembly and JavaScript / TypeScript bindings
# These are created by `wasm-pack` and are used by Node.
WASM_SOURCE := $(GENERATED_WASM_DIR)/oni_tools.d.ts \
	$(GENERATED_WASM_DIR)/oni_tools.js \
	$(GENERATED_WASM_DIR)/oni_tools_bg.js \
	$(GENERATED_WASM_DIR)/oni_tools_bg.wasm \
	$(GENERATED_WASM_DIR)/oni_tools_bg.wasm.d.ts

# === Rust / Cargo Toolchain ===================================================
# Build the crate with Cargo
# This has no dependencies because Cargo does incremental building already.
build:
	cargo build

# Run the crate with Cargo
# This has no dependencies because Cargo does incremental building already.
run:
	cargo run

# === WASM / Node Toolchain ====================================================
$(WASM_SOURCE): $(CRATE_SOURCE)
	@wasm-pack build \
		--dev \
		--no-pack \
		--target bundler \
		--out-dir $(GENERATED_WASM_DIR) \
		-- --features wasm
	# This is necessary because if you re-run `wasm-pack`, it doesn't update
	#  files unnecessarily. I need to tell make that the files were updated.
	@touch $(WASM_SOURCE)

# Build WebAssembly and JavaScript / TypeScript bindings.
wasm: $(WASM_SOURCE)

# Run the development server
serve: wasm
	@npm run start --prefix $(DEV_SERVER_DIR)

# Run the Angular development server
serve-angular: wasm
	@npm run start --prefix $(ANGULAR_SERVER_DIR)

# Run the Svelte development server
serve-svelte:
	@npm run dev --prefix $(SVELTE_SERVER_DIR)

# === Other Utilities ==========================================================
# Remove all auto-generated files
clean:
	@rm -rf $(GENERATED_WASM_DIR)
	@rm -rf $(CARGO_TARGET_DIR)
	@rm -rf "$(ANGULAR_SERVER_DIR)/.angular/cache"
	@rm -rf "$(SVELTE_SERVER_DIR)/build"
	@rm -rf "$(SVELTE_SERVER_DIR)/.svelte-kit"

# Print all variables, for debugging this file.
echo:
	$(foreach variable, $(.VARIABLES), $(info $(variable) = $($(variable))))

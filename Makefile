default: clean compile upload

clean: ## Remove all build files
	cargo clean

compile: ## Compile rust
	cargo clean
	cargo build

upload: ## Upload the firmware to the board
	cargo espflash flash

log: ## Start the logs
	cargo espflash monitor


.PHONY: all build
all: import

CART_NAME = wizards-tower
TIC_EXT = .tic

$(CART_NAME)$(TIC_EXT):
	tic80 --cli --cmd "new wasm & save $(CART_NAME)"

build:
	cargo build --release

import: target/wasm32-unknown-unknown/release/cart.wasm $(CART_NAME)$(TIC_EXT)
	tic80 --cli \
		--cmd "load wizards-tower/wizards-tower.tic & import binary wizards-tower/$< & save"

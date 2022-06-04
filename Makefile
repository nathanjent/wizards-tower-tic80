CART_NAME=wizards-tower
CART_EXT=.tic
WASM_BINARY=target/wasm32-unknown-unknown/release/cart.wasm

all: $(CART_NAME)$(CART_EXT)

$(WASM_BINARY): src/*.rs
	cargo build --release

# Load cart data, import WASM binary, and save cart
$(CART_NAME)$(CART_EXT): $(WASM_BINARY)
	rm -f $@
	tic80 --cli --fs . \
		--cmd 'load wasmdemo.wasmp & import binary $< & save $@'

open: $(CART_NAME)$(CART_EXT)
	tic80 --fs . --cmd 'load $<' &

clean:
	cargo clean
	rm -f $(CART_NAME)$(CART_EXT)

.PHONY: all clean open

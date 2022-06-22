CART_NAME=wizards-tower
CART_EXT=.tic
CART_FILE=$(CART_NAME)$(CART_EXT)
WASM_BINARY=target/wasm32-unknown-unknown/release/cart.wasm

all: $(CART_FILE)

$(WASM_BINARY): src/*.rs
	cargo build --release

# Load cart data, import WASM binary, and save cart
$(CART_FILE): $(WASM_BINARY)
	rm -f $@
	tic80 --cli --fs . \
		--cmd 'load wasmdemo.wasmp & import binary $< & save $@'

run: $(CART_FILE)
	tic80 --fs . --cmd 'load $< & run' &

clean:
	cargo clean
	rm -f $(CART_FILE)

.PHONY: all clean run

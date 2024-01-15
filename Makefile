
.PHONY: run-web
run-web:
	cargo run --target wasm32-unknown-unknown

.PONY: watch-web
watch-web:
	cargo watch -x "run --target wasm32-unknown-unknown"

.PHONY: install-deps
install-deps:
	cargo install wasm-server-runner cargo-watch

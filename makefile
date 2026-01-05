web_run: web_build
	basic-http-server ./web

web_build:
	cargo build --release --target wasm32-unknown-unknown
	cp ./target/wasm32-unknown-unknown/release/gravity-simulation.wasm web/gravity-simulation.wasm
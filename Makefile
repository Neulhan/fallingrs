build:
	wasm-pack build --target web

publish: build
	wasm-pack publish
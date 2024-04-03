build:
	rm -rf pkg && wasm-pack build --release --target bundler

build-dev:
	wasm-pack build --dev --target web --out-dir dev

publish: build
	wasm-pack publish

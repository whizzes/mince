clean:
	rm -rf ./pkg

build: clean
	wasm-pack build --target web --release

dev: clean
	wasm-pack build --target web --dev

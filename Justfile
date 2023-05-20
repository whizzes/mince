clean:
	rm -rf ./pkg

build: clean
	wasm-pack build --target web --release

dev: clean
	wasm-pack build --target web --dev
	rm -rf ./testing/pkg
	cp -R ./pkg ./testing/pkg

serve:
	http-server --verbose --logger ./testing

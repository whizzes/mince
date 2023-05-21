clean:
	rm -rf ./pkg

build: clean
	wasm-pack build --release --scope whizzes --target no-modules

dev: clean
	wasm-pack build --target web --dev
	rm -rf ./testing/pkg
	cp -R ./pkg ./testing/pkg

serve:
	./http-server --verbose --logger ./testing

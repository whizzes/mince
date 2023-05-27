clean:
	rm -rf ./pkg

build: clean
	wasm-pack build --release --scope whizzes --target web

dev: clean
	wasm-pack build --target web --dev
	rm -rf ./testing/pkg
	cp -R ./pkg ./testing/pkg

serve:
	./http-server --verbose --logger ./www

test:
	wasm-pack test --headless --firefox --geckodriver ./drivers/geckodriver

test_ci:
	wasm-pack test --headless --firefox

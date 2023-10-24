build-wasm-sync:
	set -ex
	wasm-pack build --target web

build-pages:
	rm -rf dist
	pnpm run build:pages
	cp -r public/ dist
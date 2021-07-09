cargo-dev:
	cargo watch -x run

cargo-update:
	cargo update

npm-install:
	cd client && npm install

start-client:
	cd client && npm run serve

build-client:
	cd client && npm run build
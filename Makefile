all: clean test build

clean: 
	cargo clean

build:
	cargo clean
	cargo test
	cargo build
	cargo build --release
	cargo build --release --target x86_64-unknown-linux-gnu
	cargo build --release --target x86_64-unknown-linux-musl
	cargo build --release --target x86_64-pc-windows-gnu
	ls -lh target/debug
	ls -lh target/release
	ls -lh target/x86_64-unknown-linux-gnu/release
	ls -lh target/x86_64-unknown-linux-musl/release
	ls -lh target/x86_64-pc-windows-gnu/release

docker:
	docker stop eq-ala || true
	docker rm eq-ala || true
	docker rmi eq-ala || true
	docker build -t eq-ala .
	docker run -d --name eq-ala eq-ala

test:
	cargo test

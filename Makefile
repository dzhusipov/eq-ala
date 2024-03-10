all: clean test fullbuild

clean: 
	cargo clean

fullbuild:
	cargo build
	cargo build --release
	cargo build --target x86_64-pc-windows-gnu --release
	cargo build --target x86_64-unknown-linux-gnu --release
	cargo build --target x86_64-unknown-linux-musl --release

build:
	cargo build

docker:
	docker stop eq-ala || true
	docker rm eq-ala || true
	docker rmi eq-ala || true
	docker build -t eq-ala .
	docker run -d --name eq-ala eq-ala

test:
	cargo test

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
	docker stop zhus_sip_stack || true
	docker rm zhus_sip_stack || true
	docker rmi zhus_sip_stack_img || true
	docker build -t zhus_sip_stack_img .
	docker run -d --name zhus_sip_stack -p 8080:8080 -p 5060:5060 zhus_sip_stack_img

test:
	cargo test

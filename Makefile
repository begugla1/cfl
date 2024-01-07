attach_cfl:
	chmod +x scripts/attach.sh
	./scripts/attach.sh "$(shell pwd)/dist/cfl" cfl

build:
	cargo build --release
	cp target/release/cfl dist
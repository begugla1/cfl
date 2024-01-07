attach_cfl:
	chmod +x scripts/attach.sh
	./scripts/attach.sh "$(shell pwd)/target/release/cfl" cfl

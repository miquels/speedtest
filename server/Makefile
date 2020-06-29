all:	build

install:	all

build:
	@echo "server: building"
	cargo build --release
	cp -p target/release/speedtest-server .

.PHONY: all build install

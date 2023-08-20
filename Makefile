NAME=bsq

release:
	cargo build --release
	@cp target/release/$(NAME) .

debug:
	cargo build
	@cp target/debug/$(NAME) .

check:
	cargo check

clean:
	cargo clean

test: debug
	cd ./tester && ./test.sh

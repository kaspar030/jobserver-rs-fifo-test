all:
	echo $$MAKEFLAGS
	cargo build
	+target/debug/jobserver-rs-fifo-test

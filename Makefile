.PHONY: trace

debug:
	cargo run --features tracing-off

release:
	cargo run --release --features tracing-off

trace:
	cargo run --release --features tracy
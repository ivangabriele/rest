test:
	cargo test --no-fail-fast
test-cover:
	cargo tarpaulin --no-fail-fast --out Xml --skip-clean
test-trace:
	export RUST_BACKTRACE=1 && cargo test --no-fail-fast --workspace -- --nocapture
test-quiet:
	cargo test -q --no-fail-fast --workspace -- --nocapture
test-watch:
	cargo watch -x "test --workspace -- --nocapture"

upgrade:
	cargo upgrade --exclude glob

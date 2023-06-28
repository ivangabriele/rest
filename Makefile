doc:
	cargo doc && firefox ./target/doc/jrest/index.html

test:
	cargo test --no-fail-fast
test-cover:
	cargo tarpaulin --frozen --no-fail-fast --out Xml --release --skip-clean
test-trace:
	export RUST_BACKTRACE=1 && cargo test --no-fail-fast -- --nocapture
test-watch:
	cargo watch -x "test -- --nocapture"

upgrade:
	cargo upgrade

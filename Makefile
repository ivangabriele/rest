doc:
	cargo doc && firefox ./target/doc/jrest/index.html

publish:
	cd ./jrest_hooks && cargo doc && cargo publish
	cd ./jrest && cargo doc && cargo publish
	cd ./cargo-jrest && cargo doc && cargo publish

test:
	cargo test --no-fail-fast
test-cover:
	cargo tarpaulin --frozen --no-fail-fast --out Xml --skip-clean
test-trace:
	export RUST_BACKTRACE=1 && cargo test --no-fail-fast -- --nocapture
test-watch:
	cargo watch -x "test -- --nocapture"

upgrade:
	cargo upgrade

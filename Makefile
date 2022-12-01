.PHONY: run
run:
	cargo run -- run $(day) $(bench)

.PHONY: bench
bench:
	cargo run -- run $(day) --bench

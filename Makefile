.PHONY: run sp1 r0

run: sp1 r0

sp1:
	@echo "##################################################"
	@echo "Running SP1"
	@echo "##################################################"

	cd sp1/script/ && cargo run --release -- --execute

r0:
	@echo "##################################################"
	@echo "Running RISC Zero"
	@echo "##################################################"

	cd r0/ && RISC0_DEV_MODE=1 cargo run --release

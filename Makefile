LOGS_DIR = "$(shell pwd)/logs"
LOG_TO_FILE ?= 1

.PHONY: run sp1 r0

run: sp1 r0

sp1:
		@echo "##################################################"
		@echo "Running SP1"
		@echo "##################################################"

		@if [[ "$(LOG_TO_FILE)" == "1" ]]; then \
				cd sp1/script/ && cargo run --release -- --execute 2>&1 | tee -a $(LOGS_DIR)/sp1.log; \
		else \
				cd sp1/script/ && cargo run --release -- --execute; \
		fi

r0:
		@echo "##################################################"
		@echo "Running RISC Zero"
		@echo "##################################################"

		@if [ "$(LOG_TO_FILE)" == "1" ]; then \
				cd r0/ && RISC0_DEV_MODE=1 cargo run --release 2>&1 | tee -a $(LOGS_DIR)/r0.log; \
		else \
				cd r0/ && RISC0_DEV_MODE=1 cargo run --release; \
		fi

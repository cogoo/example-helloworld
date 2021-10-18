.PHONY: config_cli
config_cli:
	@echo "config_cli"
	@solana config set --url  localhost

.PHONY: create_cli_keypair
create_cli_keypair:
	@echo "create_cli_keypair"
	@solana-keygen new --force

.PHONY: start_test_validator
start_test_validator:
	@echo "start_test_validator"
	@solana-test-validator

.PHONY: solana_logs
solana_logs:
	@echo "solana_logs"
	@solana logs -u localhost

.PHONY: build_onchain_program
build_onchain_program:
	@echo "build_onchain_program"
	@npm run build:program

.PHONY: deploy_onchain_program
deploy_onchain_program: build_onchain_program
	@echo "deploy_onchain_program"
	@solana program deploy dist/program/helloworld.so

.PHONY: say_hello
say_hello:
	@echo "start"
	@npm run start:hello


.PHONY: say_bye
say_bye:
	@echo "start"
	@npm run start:bye

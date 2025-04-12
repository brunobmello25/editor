
lint-pedantic:
	cargo clippy -- -W clippy::all -W clippy::pedantic

lint-fix:
	cargo clippy --fix -- -W clippy::all  -W clippy::pedantic

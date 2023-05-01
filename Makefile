CC=cargo
FMT=fmt

OPTIONS=

default: fmt lint
	$(CC) build
	@make example

fmt:
	$(CC) fmt --all

lint:
	$(CC) clippy --all --tests

check:
	$(CC) test --all

example:
	@echo "No example yet"

clean:
	$(CC) clean

install:
	$(CC) install --locked --path ./penguin_cmd

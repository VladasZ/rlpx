
all: rlpx

rlpx:
	cargo build

geth:
	cd go-ethereum; \
    make geth; \

run-geth:
	go-ethereum/build/bin/geth --goerli

test:
	cargo test -- --show-output

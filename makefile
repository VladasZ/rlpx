
all: rlpx

rlpx:
	cargo build


ifeq ($(OS),Windows_NT)
geth: geth-win
run-geth: run-geth-win
else
geth: geth-unix
run-geth: run-geth-unix
endif

geth-unix:
	cd go-ethereum && make geth

geth-win:
	cd go-ethereum && go get -u -v golang.org/x/net/context && go install -v ./cmd/... && refreshenv

run-geth-win:
	geth --goerli

run-geth-unix:
	go-ethereum/build/bin/geth --goerli

test:
	cargo test -- --show-output

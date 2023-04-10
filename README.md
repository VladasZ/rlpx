# rlpx

Implementation of RPLx protocol. `geth` is used as a test node.

## Dependencies

Rust: https://rustup.rs/

Go: https://go.dev/doc/install 1.20 minumum

## Build

Clone repository with submodules:

```bash
git clone --recursive https://github.com/VladasZ/rlpx.git
cd rlpx
```

Build `rplx` library:
```bash
make
```

Build `geth` to start p2p node locally:
```bash
make geth
```

## Testing

TODO
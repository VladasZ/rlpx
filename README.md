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

This test will run `geth` test node locally and perform handshake with it.

```bash
make test
```

Successful test output example:
```json lines
Geth node started.
NodeConnection {
    public_key: "cc3a313d9894d23fac7decfd268bb052887c415dea339c301c053548ac30243be32d78898c2055f2a2a934638396f6a5906e732da67ab4116a8b13f0c85cc63e",
    ip: "127.0.0.1",
    port: 30303,
}
Handshake successful:
RemoteAck {
    ephemeral: 0xb91675f2bdbac9b88cfcae72971191adcc2d9dbf3520139d44dbc9a7f7fc04a448a4e35da9639be155e0fee8537b6c6d82af643b3d7476895a305324cd96eb66,
    nonce: 0xe902f73c70068ad9f820ad2f7cd1153e7ab695d387bb327c646115848388e9a5,
    protocol_version: 4,
}
```

### Manual test:

You can run geth test node manually with:
```bash
make run-geth
```

Look for enode URL. It looks similar to this:
`enode://8db16132ebd913b23538cc87a6d6c0c88ba9178164c0ec9e978b59e11909c7820039d29237c69f7e8c68a4ed786961332f0ab559f6f40f44be6f975181199dac@192.168.10.228:30303`

Pass it as argument:

```bash
 cargo run -- enode://cc3a313d9894d23fac7decfd268bb052887c415dea339c301c053548ac30243be32d78898c2055f2a2a934638396f6a5906e732da67ab4116a8b13f0c85cc63e@127.0.0.1:30303
```

Successful test output should be the same.

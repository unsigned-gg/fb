# fb

> fb (Fast Build & Framebuffer Engine)  
> Terminal canvas, pixel and glyph rendering primitives, and fast build graph execution for dagr.  
> Dual-licensed under `Apache-2.0 OR MIT`.

## Philosophical Foundations
- **Champion Inspiration:** Carl Lerche & Manish Goregaokar
- **Zero Days Open Source Initiative (ZOSI):** Machine sympathy, low-level efficiency, and verified contracts.
- **Architecture:** Zero-I/O domain model in `fb-domain` separated from systems runtime in `fb`.

## Crates
- `crates/fb`: Systems implementation and runtime interfaces.
- `crates/fb-domain`: Pure zero-I/O domain models, validation, and algebraic state machines.

## Building
All builds must specify `-j 2` to prevent memory exhaustion on shared multi-agent hosts:
```bash
cargo build -j 2
cargo test -j 2
```

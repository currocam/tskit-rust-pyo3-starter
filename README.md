# tskit-maturin-starter

A minimal starter template for building [tskit](https://tskit.dev/) extensions in Rust with Python bindings.

- In the Rust side, we use the bindings to the C library in [tskit-rust](https://github.com/tskit-dev/tskit-rust). For more information, read [tskit-rust book](https://tskit-dev.github.io/tskit-rust/introduction.html).
- We generate all Python bindings to Rust code with [PyO3](https://pyo3.rs/v0.28.2/) and use [maturin](https://www.maturin.rs/) for building.

This Python library re-implements a [haploid Wright-Fisher simulation](https://github.com/tskit-dev/tskit/blob/main/c/examples/haploid_wright_fisher.c) in Rust and exposes it to the user from Python. 

```python
import tskit_maturin_starter

ts = tskit_maturin_starter.sim_haploid_wright_fisher(
    population_size=100,
    num_generations=500,
    random_seed=42,
    simplify_interval=100,
)
╔═════════════════════════╗
║TreeSequence             ║
╠═══════════════╤═════════╣
║Trees          │      996║
╟───────────────┼─────────╢
║Sequence Length│        1║
╟───────────────┼─────────╢
║Time Units     │  unknown║
╟───────────────┼─────────╢
║Sample Nodes   │      100║
╟───────────────┼─────────╢
║Total Size     │155.7 KiB║
╚═══════════════╧═════════╝
╔═══════════╤═════╤═════════╤════════════╗
║Table      │Rows │Size     │Has Metadata║
╠═══════════╪═════╪═════════╪════════════╣
║Edges      │3,455│108.0 KiB│          No║
╟───────────┼─────┼─────────┼────────────╢
║Individuals│    0│ 24 Bytes│          No║
╟───────────┼─────┼─────────┼────────────╢
║Migrations │    0│  8 Bytes│          No║
╟───────────┼─────┼─────────┼────────────╢
║Mutations  │    0│ 16 Bytes│          No║
╟───────────┼─────┼─────────┼────────────╢
║Nodes      │  756│ 20.7 KiB│          No║
╟───────────┼─────┼─────────┼────────────╢
║Populations│    0│  8 Bytes│          No║
╟───────────┼─────┼─────────┼────────────╢
║Provenances│    0│ 16 Bytes│          No║
╟───────────┼─────┼─────────┼────────────╢
║Sites      │    0│ 16 Bytes│          No║
╚═══════════╧═════╧═════════╧════════════╝
```


## Useful parts

- `src/ffi.rs` implements pointer conversion to pass Rust a `tskit::TableCollection` into the Python runtime and construct a `tskit.TreeSequence`[^1].
- `src/haploid_wright_fisher.rs` implements the simulation algorithm using C bindings and Rust-specific libraries. 
- `src/lib.rs` defines how we define the PyO3 module. 

[^1]: _Caveat emptor_: I used LLMs to write this section. 

## Development

```bash
make dev
make build
make test
```

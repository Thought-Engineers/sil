# Sil Compression Engine

`sil` is a cross-platform, production-grade compression and decompression CLI tool based on **The Silence Protocol** (V3.0 Base-4 Temporal Channel). 

It implements a **Self-Extracting Topological Archive (FractalZip)**. Unlike standard compression algorithms like `zip` that require storing complex dictionary headers, `sil` translates data into a multi-dimensional Base-4 fractal tree. It achieves high compression on repetitive text by constructing the dictionary on-the-fly directly into the payload stream using raw topological branching.

In benchmarks against highly repetitive text payloads, `sil` outperforms standard `zip` compression while maintaining 100% structural integrity (lossless compression, preserving exact whitespace and punctuation).

## Features
- **Lossless Compression**: Intelligently tokenizes alphanumeric words, whitespace, and punctuation to guarantee byte-for-byte exact matches upon decompression.
- **Cross-Platform & Multi-Language**: Initially built in Go, and now includes a 100% compatible Rust implementation (`rsil`) for high-performance use cases.
- **Topological Dictionary**: Employs Base-4 fractal pointer sequences instead of standard flat-file headers, allowing for massive theoretical compression on repetitive structures.

## Installation

You can download the pre-compiled binaries from the [Releases](https://github.com/somaos-nc/sil/releases/latest) page, or build it from source.

### Build Go Version
Ensure you have [Go](https://golang.org/doc/install) installed.

```bash
git clone https://github.com/yourusername/sil.git
cd sil
go build -o sil cmd/sil/main.go
```

### Build Rust Version (`rsil`)
The Rust version (`rsil`) is managed as a Git submodule hosted at [somaos-nc/rsil](https://github.com/somaos-nc/rsil). Ensure you have [Rust](https://rustup.rs/) installed.

```bash
git clone --recursive https://github.com/somaos-nc/sil.git
cd sil/rsil
cargo build --release
# The compiled binary will be located at target/release/rsil
```

## Usage

The CLI is straightforward. It takes a command (`compress` or `decompress`), an input file, and an output file. Both the Go (`sil`) and Rust (`rsil`) versions use the identical interface and are fully cross-compatible.

### Compress a File
```bash
./sil compress my_text.txt my_text.sil
# or
./rsil/target/release/rsil compress my_text.txt my_text.sil
```

### Decompress a File
```bash
./sil decompress my_text.sil restored_text.txt
# or
./rsil/target/release/rsil decompress my_text.sil restored_text.txt
```

## How It Works (The Silence Protocol)
`sil` is a software implementation of the theoretical interstellar communication framework known as *The Silence Protocol*. You can read the full theoretical whitepaper here: [THE_SILENCE_PROTOCOL.pdf](./docs/THE_SILENCE_PROTOCOL.pdf).

The protocol dictates abandoning traditional binary (1s and 0s) arrays in favor of a Quaternary (Base-4) system representing structural commands:
- `0`: Data State 0
- `1`: Data State 1
- `2`: Branch In (Enter new sub-sphere)
- `3`: Branch Out (Return to parent sphere)

By navigating these branches, the `sil` engine dynamically maps unique molecules (words, spaces, punctuation) into a **Dictionary Sphere** and references them in a **Payload Sphere**, effectively recreating complex data structures through the universal grammar of topology.

## Integration: `pcompress`
The Sil compression engine has been integrated into the powerful, multi-threaded `pcompress` archiver as a native C-compatible backend. `pcompress` can now leverage the Base-4 Temporal Channel algorithm for repetitive datasets, combining it with chunking and parallelism to significantly reduce Sil's memory overhead on large files.

The integration wraps the Rust `rsil` static library (`librsil.a`) via a C Foreign Function Interface (FFI). 

> **Note on Submodules:** The `pcompress` integration is managed as a Git submodule. Because it is hosted on Codeberg, GitHub's web interface may fail to render the submodule folder link correctly. You can view and contribute to the `pcompress` fork here:
> **[https://codeberg.org/noam-cohen/pcompress](https://codeberg.org/noam-cohen/pcompress)**

To build `pcompress` with Sil support:
```bash
git clone --recursive https://github.com/somaos-nc/sil.git
cd sil/rsil
cargo build --release  # Build the Rust C-FFI static library

cd ../pcompress
./config
make
```

## License
The `sil` and `rsil` engines are licensed under the **MIT License**.

### Third-Party Licenses
The `pcompress` integration and its submodule are dual-licensed under the **GNU Lesser General Public License v3 (LGPLv3)** and the **Mozilla Public License v2 (MPLv2)**. 

Please note that while most of `pcompress` is covered under MPLv2, certain bundled third-party components (like LZP and PackJPG) use LGPLv2/3. See `pcompress/README.LICENSE` for full details on distributing the integrated binary.

# Sil Compression Engine

`sil` is a cross-platform, production-grade compression and decompression CLI tool based on **The Silence Protocol** (V3.0 Base-4 Temporal Channel). 

It implements a **Self-Extracting Topological Archive (FractalZip)**. Unlike standard compression algorithms like `zip` that require storing complex dictionary headers, `sil` translates data into a multi-dimensional Base-4 fractal tree. It achieves high compression on repetitive text by constructing the dictionary on-the-fly directly into the payload stream using raw topological branching.

In benchmarks against highly repetitive text payloads, `sil` outperforms standard `zip` compression while maintaining 100% structural integrity (lossless compression, preserving exact whitespace and punctuation).

## Features
- **Lossless Compression**: Intelligently tokenizes alphanumeric words, whitespace, and punctuation to guarantee byte-for-byte exact matches upon decompression.
- **Cross-Platform**: Built in Go, compiling effortlessly to single binaries for Windows, macOS, and Linux.
- **Topological Dictionary**: Employs Base-4 fractal pointer sequences instead of standard flat-file headers, allowing for massive theoretical compression on repetitive structures.

## Installation

You can download the pre-compiled binaries from the [Releases](https://github.com/somaos-nc/sil/releases/latest) page, or build it from source.

### Build from Source
Ensure you have [Go](https://golang.org/doc/install) installed.

```bash
git clone https://github.com/yourusername/sil.git
cd sil
go build -o sil cmd/sil/main.go
```

## Usage

The CLI is straightforward. It takes a command (`compress` or `decompress`), an input file, and an output file.

### Compress a File
```bash
./sil compress my_text.txt my_text.sil
```

### Decompress a File
```bash
./sil decompress my_text.sil restored_text.txt
```

## How It Works (The Silence Protocol)
`sil` is a software implementation of the theoretical interstellar communication framework known as *The Silence Protocol*. You can read the full theoretical whitepaper here: [THE_SILENCE_PROTOCOL.pdf](./docs/THE_SILENCE_PROTOCOL.pdf).

The protocol dictates abandoning traditional binary (1s and 0s) arrays in favor of a Quaternary (Base-4) system representing structural commands:
- `0`: Data State 0
- `1`: Data State 1
- `2`: Branch In (Enter new sub-sphere)
- `3`: Branch Out (Return to parent sphere)

By navigating these branches, the `sil` engine dynamically maps unique molecules (words, spaces, punctuation) into a **Dictionary Sphere** and references them in a **Payload Sphere**, effectively recreating complex data structures through the universal grammar of topology.

## License
MIT License

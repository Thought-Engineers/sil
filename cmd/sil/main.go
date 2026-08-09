package main

import (
	"encoding/binary"
	"fmt"
	"io"
	"os"
	"sil/fractalzip"
)

func main() {
	if len(os.Args) < 4 {
		fmt.Println("Usage: sil <compress|decompress> <input_file> <output_file>")
		os.Exit(1)
	}

	command := os.Args[1]
	inputFile := os.Args[2]
	outputFile := os.Args[3]

	fz := fractalzip.New()

	switch command {
	case "compress":
		err := compressFile(fz, inputFile, outputFile)
		if err != nil {
			fmt.Printf("Error compressing: %v\n", err)
			os.Exit(1)
		}
		fmt.Println("Successfully compressed.")
	case "decompress":
		err := decompressFile(fz, inputFile, outputFile)
		if err != nil {
			fmt.Printf("Error decompressing: %v\n", err)
			os.Exit(1)
		}
		fmt.Println("Successfully decompressed.")
	default:
		fmt.Printf("Unknown command: %s\n", command)
		os.Exit(1)
	}
}

func compressFile(fz *fractalzip.FractalZip, input, output string) error {
	data, err := os.ReadFile(input)
	if err != nil {
		return err
	}

	stream := fz.Compress(string(data))
	packed := fractalzip.Pack(stream)

	out, err := os.Create(output)
	if err != nil {
		return err
	}
	defer out.Close()

	// Write length of stream as uint64
	lengthBuf := make([]byte, 8)
	binary.LittleEndian.PutUint64(lengthBuf, uint64(len(stream)))
	if _, err := out.Write(lengthBuf); err != nil {
		return err
	}

	// Write packed data
	if _, err := out.Write(packed); err != nil {
		return err
	}

	return nil
}

func decompressFile(fz *fractalzip.FractalZip, input, output string) error {
	in, err := os.Open(input)
	if err != nil {
		return err
	}
	defer in.Close()

	// Read length
	lengthBuf := make([]byte, 8)
	if _, err := io.ReadFull(in, lengthBuf); err != nil {
		return err
	}
	streamLen := binary.LittleEndian.Uint64(lengthBuf)

	// Read packed data
	packed, err := io.ReadAll(in)
	if err != nil {
		return err
	}

	stream := fractalzip.Unpack(packed, int(streamLen))
	recovered := fz.Decompress(stream)

	return os.WriteFile(output, []byte(recovered), 0644)
}

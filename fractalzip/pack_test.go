package fractalzip_test

import (
	"sil/fractalzip"
	"testing"
)

func TestPackUnpack(t *testing.T) {
	stream := []int{0, 1, 2, 3, 3, 2, 1, 0, 1, 2}
	packed := fractalzip.Pack(stream)
	
	// 0(00), 1(01), 2(10), 3(11) -> 00 01 10 11 -> 0x1B
	// 3(11), 2(10), 1(01), 0(00) -> 11 10 01 00 -> 0xE4
	// 1(01), 2(10), p(00), p(00) -> 01 10 00 00 -> 0x60
	// 10 symbols = 3 bytes (padded). We also need to know the exact length to unpack correctly.
	
	unpacked := fractalzip.Unpack(packed, len(stream))

	if len(stream) != len(unpacked) {
		t.Fatalf("Expected length %d, got %d", len(stream), len(unpacked))
	}

	for i := range stream {
		if stream[i] != unpacked[i] {
			t.Errorf("At index %d: expected %d, got %d", i, stream[i], unpacked[i])
		}
	}
}

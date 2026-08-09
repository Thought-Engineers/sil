package fractalzip

func Pack(stream []int) []byte {
	var packed []byte
	var currentByte byte
	var bitCount int

	for _, symbol := range stream {
		currentByte = (currentByte << 2) | byte(symbol&3)
		bitCount += 2

		if bitCount == 8 {
			packed = append(packed, currentByte)
			currentByte = 0
			bitCount = 0
		}
	}

	if bitCount > 0 {
		currentByte = currentByte << (8 - bitCount)
		packed = append(packed, currentByte)
	}

	return packed
}

func Unpack(packed []byte, expectedLen int) []int {
	var stream []int
	var totalUnpacked int

	for _, b := range packed {
		for i := 6; i >= 0; i -= 2 {
			if totalUnpacked >= expectedLen {
				return stream
			}
			symbol := (b >> i) & 3
			stream = append(stream, int(symbol))
			totalUnpacked++
		}
	}

	return stream
}

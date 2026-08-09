package fractalzip_test

import (
	"sil/fractalzip"
	"testing"
)

func TestCompressDecompress(t *testing.T) {
	fz := fractalzip.New()
	targetText := "the alien sent the signal and the alien waited for the signal"

	payload := fz.Compress(targetText)
	recoveredText := fz.Decompress(payload)

	if targetText != recoveredText {
		t.Errorf("Expected %q, got %q", targetText, recoveredText)
	}
}

func TestLosslessCompressDecompress(t *testing.T) {
	fz := fractalzip.New()
	targetText := "Hello, World!\nThis is a lossless test.\n\n\tTabs,   spaces, and newlines."

	payload := fz.Compress(targetText)
	recoveredText := fz.Decompress(payload)

	if targetText != recoveredText {
		t.Errorf("Expected %q, got %q", targetText, recoveredText)
	}
}

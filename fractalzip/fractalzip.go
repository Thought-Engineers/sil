package fractalzip

import (
	"unicode"
)

type FractalZip struct{}

func New() *FractalZip {
	return &FractalZip{}
}

func textToBinary(text string) []int {
	var bin []int
	for _, c := range []byte(text) {
		for i := 7; i >= 0; i-- {
			bit := (c >> i) & 1
			bin = append(bin, int(bit))
		}
	}
	return bin
}

func binaryToText(binArray []int) string {
	var bytes []byte
	for i := 0; i < len(binArray); i += 8 {
		var b byte
		for j := 0; j < 8; j++ {
			if i+j < len(binArray) && binArray[i+j] == 1 {
				b |= (1 << (7 - j))
			}
		}
		bytes = append(bytes, b)
	}
	return string(bytes)
}

func tokenize(text string) []string {
	var tokens []string
	var current []rune

	state := 0 // 0=start, 1=word, 2=space, 3=other

	for _, r := range text {
		currentState := 3
		if unicode.IsLetter(r) || unicode.IsDigit(r) {
			currentState = 1
		} else if unicode.IsSpace(r) {
			currentState = 2
		}

		if state != 0 && state != currentState {
			tokens = append(tokens, string(current))
			current = []rune{}
		}
		current = append(current, r)
		state = currentState
	}
	if len(current) > 0 {
		tokens = append(tokens, string(current))
	}
	return tokens
}

func (fz *FractalZip) Compress(rawText string) []int {
	words := tokenize(rawText)

	var uniqueWords []string
	wordMap := make(map[string]bool)

	for _, word := range words {
		if !wordMap[word] {
			uniqueWords = append(uniqueWords, word)
			wordMap[word] = true
		}
	}

	transmission := []int{2} // BRANCH IN (Start Dictionary Sphere)
	for _, word := range uniqueWords {
		transmission = append(transmission, 2) // Branch IN (Start Word Definition)
		transmission = append(transmission, textToBinary(word)...)
		transmission = append(transmission, 3) // Branch OUT (End Word Definition)
	}
	transmission = append(transmission, 3) // BRANCH OUT (End Dictionary Sphere)

	transmission = append(transmission, 2) // BRANCH IN (Start Payload Sphere)
	for _, word := range words {
		var wordID int
		for i, w := range uniqueWords {
			if w == word {
				wordID = i
				break
			}
		}
		transmission = append(transmission, 2) // Branch IN (Start Pointer)

		// Write ID in binary
		if wordID == 0 {
			transmission = append(transmission, 0)
		} else {
			var binID []int
			tempID := wordID
			for tempID > 0 {
				binID = append([]int{tempID % 2}, binID...)
				tempID /= 2
			}
			transmission = append(transmission, binID...)
		}

		transmission = append(transmission, 3) // Branch OUT (End Pointer)
	}
	transmission = append(transmission, 3) // BRANCH OUT (End Payload Sphere)

	return transmission
}

type Node struct {
	Children []*Node
	Data     []int
}

func (fz *FractalZip) Decompress(stream []int) string {
	root := &Node{}
	stack := []*Node{root}
	current := root

	for _, symbol := range stream {
		if symbol == 0 || symbol == 1 {
			current.Data = append(current.Data, symbol)
		} else if symbol == 2 {
			newNode := &Node{}
			current.Children = append(current.Children, newNode)
			stack = append(stack, newNode)
			current = newNode
		} else if symbol == 3 {
			if len(stack) > 1 {
				stack = stack[:len(stack)-1]
				current = stack[len(stack)-1]
			}
		}
	}

	if len(root.Children) != 2 {
		return "" // Invalid stream, expects Dictionary and Payload spheres
	}

	dictSphere := root.Children[0]
	payloadSphere := root.Children[1]

	rebuiltDictionary := make(map[int]string)
	for i, wordNode := range dictSphere.Children {
		rebuiltDictionary[i] = binaryToText(wordNode.Data)
	}

	var finalText string
	for _, pointerNode := range payloadSphere.Children {
		pointerID := 0
		for _, b := range pointerNode.Data {
			pointerID = (pointerID << 1) | b
		}
		finalText += rebuiltDictionary[pointerID]
	}

	return finalText
}

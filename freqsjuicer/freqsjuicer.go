package freqsjuicer

import (
	"io"
	"net/http"
	"os"
	"regexp"
)

var markupTagPatt = regexp.MustCompile(`<[^>]*>`)

func JuiceURL(url string) ([]int, error) {
	resp, err := http.Get(url)
	if err != nil {
		return nil, err
	}
	return juiceNotes(resp.Body)[:2000], nil
}

func JuiceFile(filePath string) ([]int, error) {
	file, err := os.Open(filePath)
	if err != nil {
		return nil, err
	}
	return juiceNotes(file), err
}

func juiceNotes(reader io.ReadCloser) (freqs []int) {
	data, err := io.ReadAll(reader)
	if err != nil {
		panic(err)
	}
	defer reader.Close()

	for _, freq := range removeMarkupTags(data) {
		freqs = append(freqs, int(freq))
	}

	return
}

func removeMarkupTags(input []byte) []byte {
	return markupTagPatt.ReplaceAll(input, []byte(""))
}

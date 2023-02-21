package freqsjuicer

import (
	"io"
	"net/http"
	"os"
)

func JuiceURL(url string) ([]int, error) {
	resp, err := http.Get(url)
	if err != nil {
		return nil, err
	}
	return juiceNotes(resp.Body), nil
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

	for _, freq := range data {
		freqs = append(freqs, int(freq))
	}

	return
}

package savefile

import (
	"bytes"
	"encoding/binary"
	"io"
	"math"

	"github.com/mbaraa/grievous/music"
)

const (
	BitDepth = 32

	// Chunk descriptor
	ChunkId              = "RIFF"
	ChunkSizePlaceholder = "----"
	Format               = "WAVE"

	// fmt sub chunk
	SubChunk1Id   = "fmt "
	SubChunk1Size = 16
	AudioFormat   = 1
	NumChannels   = 2
	SampleRate    = 44100
	ByteRate      = SampleRate * NumChannels * (SubChunk1Size / 8)
	BlockAlign    = NumChannels * (SubChunk1Size / 8)
	BitsPerSample = BitDepth

	// Data sub chunk
	SubChunk2Id             = "data"
	SubChunk2SizePlaceholer = "----"
)

type SineOscillator struct {
	freq, amp, angle, offset float64
}

func writeBytes(w io.Writer, value any) {
	binary.Write(w, binary.LittleEndian, value)
}

func writeBytesAt(w io.WriterAt, value any, at int64) {
	buf := bytes.NewBuffer([]byte{})
	binary.Write(buf, binary.LittleEndian, value)
	w.WriteAt(buf.Bytes(), at)
}

type Writer interface {
	io.Writer
	io.WriterAt
	io.Seeker
}

func WriteWaveFileWithScale(w Writer, freqs []int, scale music.Scale) error {
	return writeWaveFile(w, music.GetScaledFreqs(freqs, scale))
}

func WriteWaveFile(w Writer, freqs []int) error {
	return writeWaveFile(w, freqs)
}

func writeWaveFile(w Writer, freqs []int) error {
	// RIFF chunk descriptor
	w.Write([]byte(ChunkId))              // chunk id
	w.Write([]byte(ChunkSizePlaceholder)) // chunk size
	w.Write([]byte(Format))               // format

	// fmt sub chunk
	w.Write([]byte(SubChunk1Id))         // sub chunk1 id
	writeBytes(w, uint32(SubChunk1Size)) // sub chunk1 size
	writeBytes(w, uint16(AudioFormat))   // audio format
	writeBytes(w, uint16(NumChannels))   // number of channels
	writeBytes(w, uint32(SampleRate))    // sample rate
	writeBytes(w, uint32(ByteRate))      // byte rate
	writeBytes(w, uint16(BlockAlign))    // block align
	writeBytes(w, uint16(BitsPerSample)) // bit depth

	// data chunk
	w.Write([]byte(SubChunk2Id))             // sub chunk2 id
	w.Write([]byte(SubChunk2SizePlaceholer)) // sub chunk2 size

	preDataPosition, err := w.Seek(0, io.SeekCurrent)
	if err != nil {
		return err
	}

	for _, freq := range freqs {
		maxAmplitude := (1 << (BitDepth - 1)) - 1
		duration := 0.15

		for i := 0.5; i < SampleRate*duration; i += 1 {
			amplitude := i / SampleRate * float64(maxAmplitude)
			sample := math.Sin(2.0 * math.Pi * i * float64(freq) / SampleRate)

			channel1 := amplitude * sample
			// channel2 := (maxAmplitude - int(amplitude)) * int(sample)
			writeBytes(w, int32(channel1))
			writeBytes(w, float32(channel1))
		}
	}

	postDataPosition, err := w.Seek(0, io.SeekCurrent)
	if err != nil {
		return err
	}

	writeBytesAt(w, uint32(postDataPosition-preDataPosition), preDataPosition-4)
	writeBytesAt(w, uint32(postDataPosition-8), 4)

	return nil
}

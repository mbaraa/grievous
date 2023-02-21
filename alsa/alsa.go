package alsa

// #cgo LDFLAGS : -lasound -lm
// #include "alsa.h"
import "C"

const (
	Duration   = 0.3   // tone's duration [s]
	SampleRate = 44100 // PCM rate [Hz]
	Frequency  = 440   // tone's frequency [Hz]
)

func Init() {
	C.init()
}

func Destroy() {
	C.destroy()
}

func PlayFrequency(freq int) {
	C.play_frequency(C.int(freq))
}

func PlayFrequencyCustom(freq, rate int, latency, duration float32) {
	C.play_frequency_with_custom_params(C.int(freq), C.int(rate), C.float(latency), C.float(duration))
}

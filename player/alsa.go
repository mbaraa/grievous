package player

import (
	"fmt"
	"time"

	"github.com/mbaraa/grievous/alsa"
)

func PlaySound(freqs []int) {
	for _, freq := range freqs {
		fmt.Println(freq)
		alsa.PlayFrequencyCustom(freq, alsa.SampleRate, 0.2, 0.3)
		time.Sleep(time.Second / 8)
	}
}

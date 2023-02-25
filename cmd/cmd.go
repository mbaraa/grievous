package cmd

import (
	"flag"
	"fmt"
	"os"

	"github.com/mbaraa/grievous/freqsjuicer"
	"github.com/mbaraa/grievous/music"
	"github.com/mbaraa/grievous/player"
	"github.com/mbaraa/grievous/savefile"
)

var (
	flags     = flag.NewFlagSet("Grievous", flag.ExitOnError)
	url       = new(string)
	file      = new(string)
	wav       = new(string)
	scaleName = new(string)
)

func Start() {
	registerFlags()
	flags.Parse(os.Args[1:])
	runWithGivenArgs()
}

func quitIfError(err error) {
	if err != nil {
		_, _ = fmt.Fprintln(os.Stderr, err.Error())
		os.Exit(1)
	}
}

func runWithGivenArgs() {
	var freqs []int
	var err error
	if len(*url) > 0 {
		freqs, err = freqsjuicer.JuiceURL(*url)
		quitIfError(err)
	} else if len(*file) > 0 {
		freqs, err = freqsjuicer.JuiceFile(*file)
		quitIfError(err)
	} else {
		flags.Usage()
		return
	}

	if scale, found := music.Scales[*scaleName]; found {
		freqs = music.GetScaledFreqs(freqs, scale)
	}

	if len(*wav) > 0 {
		out, err := os.Create(*wav)
		quitIfError(err)
		savefile.WriteWaveFile(out, freqs)
	} else {
		player.PlaySound(freqs)
	}
}

func registerFlags() {
	flags.StringVar(url, "url", "", "generate noises from a specific URL")
	flags.StringVar(file, "file", "", "generate noises from a specific file")
	flags.StringVar(wav, "wav", "", "save noises to the specified wav file")
	flags.StringVar(scaleName, "scale", "", "choose a scale from the built-in scales")
}

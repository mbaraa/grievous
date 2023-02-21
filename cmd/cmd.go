package cmd

import (
	"flag"
	"fmt"
	"os"

	"github.com/mbaraa/grievous/freqsjuicer"
	"github.com/mbaraa/grievous/player"
)

var (
	flags = flag.NewFlagSet("Grievous", flag.ExitOnError)
	url   = new(string)
	file  = new(string)
)

func Start() {
	registerFlags()
	flags.Parse(os.Args[1:])
	runWithGivenArgs()
}

func runWithGivenArgs() {
	if len(*url) > 0 {
		freqs, err := freqsjuicer.JuiceURL(*url)
		if err != nil {
			_, _ = fmt.Fprintln(os.Stderr, err.Error())
			return
		}
		player.PlaySound(freqs)
	} else if len(*file) > 0 {
		freqs, err := freqsjuicer.JuiceFile(*file)
		if err != nil {
			_, _ = fmt.Fprintln(os.Stderr, err.Error())
			return
		}
		player.PlaySound(freqs)
	} else {
		flags.Usage()
	}
}

func registerFlags() {
	flags.StringVar(url, "url", "", "generate noises from a specific URL")
	flags.StringVar(file, "file", "", "generate noises from a specific file")
}

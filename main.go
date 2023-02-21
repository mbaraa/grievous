package main

import (
	"github.com/mbaraa/grievous/alsa"
	"github.com/mbaraa/grievous/cmd"
)

func main() {
	alsa.Init()
	defer alsa.Destroy()
	cmd.Start()
}

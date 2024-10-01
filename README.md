# Grievous

Named after [General Grievous](https://starwars.fandom.com/wiki/Grievous), where it generates noises from text files or a provided URL, just like how Grievous makes weird noises when he talks.

# Usage:

## Dependencies

- rust (duh)
- cargo (duh2)
- cmake
- libssl
- alsa

## Install the thing

```bash
; cargo install grievous
```

_OR_

- Clone the repo

```bash
; git clone https://github.com/mbaraa/grievous
```

- Run it using cargo (might take some time compiling)

```bash
; cargo run play url https://google.com
```

# More usage:

```bash
; grievous [? RUN MODE] [SOURCE TYPE] [SOURCE PATH] [? SCALE NAME]

    RUN MODE: (optional) either \"play\" or \"wav\", and defaults to play
        - play: reads the input and blasts it out of a speaker.
        - wav: saves it into a file of the format <orig_file_name>_grievous.wav
    SOURCE TYPE: input file type, it can be either \"url\", \"file\"
        - url: reads the input from a url
        - file: reads the input from a file
    SOURCE PATH: a valid url or a file path
    SCALE NAME: (optional) either a scale from the list under \"./scales.json\", or without a scale if not specified.

Examples:
    ; grievous play url https://rustup.rs
    ; grievous play file ./README.md
    ; grievous play file ./README.md saba
```

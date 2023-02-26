package music

type Scale map[rune]int

var Scales = map[string]Scale{
	"hijjaz": {
		'a': 392,
		'b': 1486,
		'c': 1174,
		'd': 466,
		'e': 185,
		'f': 523,
		'g': 587,
		'h': 622,
		'i': 293,
		'j': 740,
		'k': 784,
		'l': 880,
		'm': 1760,
		'n': 1568,
		'o': 311,
		'p': 370,
		'q': 146,
		'r': 196,
		's': 440,
		't': 220,
		'u': 261,
		'v': 1244,
		'w': 155,
		'x': 1046,
		'y': 233,
		'z': 932,
	},
	"saba": {
		'a': 440,
		'b': 174,
		'c': 466,
		'd': 523,
		'e': 554,
		'f': 277,
		'g': 586,
		'h': 638,
		'i': 698,
		'j': 1276,
		'k': 1172,
		'l': 293,
		'm': 319,
		'n': 146,
		'o': 185,
		'p': 370,
		'q': 220,
		'r': 261,
		's': 1046,
		't': 159,
		'u': 880,
		'v': 233,
		'w': 740,
		'x': 349,
		'y': 1108,
		'z': 932,
	},
}

func GetScaledFreqs(freqs []int, scale Scale) []int {
	scaledFreqs := make([]int, len(freqs))
	for i, freq := range freqs {
		newFreq, ok := scale[rune(freq)]
		if !ok {
			newFreq = ((freq - int('a')) % 26) + int('a')
		}
		if newFreq == 0 || newFreq == 123 {
			newFreq = scale['a']
		}
		scaledFreqs[i] = newFreq
	}
	return scaledFreqs
}

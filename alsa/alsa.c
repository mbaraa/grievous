#include "alsa.h"

const int RATE = 44100;
const float DURATION = 0.5;
const float LATENCY = 0.1;
snd_pcm_t *handle;

long long second_to_micro(float seconds) { return (long long)(seconds * 1e6); }

void init() {
  snd_pcm_open(&handle, "default", SND_PCM_STREAM_PLAYBACK,
               0 /* blocked mode */);
}

void destroy() { snd_pcm_close(handle); }

void play_frequency(int freq) {
  play_frequency_with_custom_params(freq, RATE, LATENCY, DURATION);
}

void play_frequency_with_custom_params(int freq, int rate, float latency,
                                       float duration) {
  latency = second_to_micro(latency);

  unsigned char buffer[(int)(rate * duration)];

  for (int i = 0; i < sizeof(buffer); i++) {
    buffer[i] = 0xFF * sin(2 * M_PI * freq * i / rate);
  }

  snd_pcm_set_params(handle, SND_PCM_FORMAT_U8, SND_PCM_ACCESS_RW_INTERLEAVED,
                     1 /* channels */, rate /* rate [Hz] */,
                     1 /* soft resample */, latency /* latency [us] */);

  snd_pcm_writei(handle, buffer, sizeof(buffer));
}

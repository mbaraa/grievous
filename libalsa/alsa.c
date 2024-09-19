#include "alsa.h"

const unsigned int LATENCY = 100000;
snd_pcm_t *handle;

long long second_to_micro(float seconds) { return (long long)(seconds * 1e6); }

int init_alsa() {
  return snd_pcm_open(&handle, "default", SND_PCM_STREAM_PLAYBACK,
                      0 /* blocked mode */);
}

int destroy_alsa() { return snd_pcm_close(handle); }

int play_frequency(float freq, unsigned int rate, float duration) {
  unsigned char buffer[(int)(rate * duration)];

  for (size_t i = 0; i < sizeof(buffer); i++) {
    buffer[i] = 0xFF * sin(2 * M_PI * freq * i / rate);
  }

  if (0 != snd_pcm_set_params(handle, SND_PCM_FORMAT_U8,
                              SND_PCM_ACCESS_RW_INTERLEAVED, 1 /* channels */,
                              rate /* rate [Hz] */, 1 /* soft resample */,
                              LATENCY /* latency [us] */)) {
    return 0;
  }

  snd_pcm_writei(handle, buffer, sizeof(buffer));

  return 0;
}

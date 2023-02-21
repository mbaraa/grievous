#ifndef ALSA_GO_H
#define ALSA_GO_H

#include <alsa/asoundlib.h>
#include <math.h>

void init();
void destroy();
void play_frequency(int freq);
void play_frequency_with_custom_params(int freq, int rate, float latency,
                                       float duration);

#endif

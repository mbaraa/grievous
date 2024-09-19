#ifndef ALSA_RS_H
#define ALSA_RS_H

#include <alsa/asoundlib.h>
#include <math.h>

extern int init_alsa();
extern int destroy_alsa();
extern int play_frequency(float freq, unsigned int rate, float duration);

#endif

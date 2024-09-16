#ifndef ALSA_RS_H
#define ALSA_RS_H

#include <alsa/asoundlib.h>
#include <math.h>
#include <sys/types.h>

extern int init();
extern int destroy();
extern int play_frequency_with_custom_params(u_int16_t freq, u_int16_t rate,
                                             float latency, float duration);

#endif

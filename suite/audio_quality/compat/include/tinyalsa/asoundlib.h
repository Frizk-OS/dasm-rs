#pragma once

#include <stdlib.h>

// Host-only compatibility shim for the legacy TinyALSA API used by this project.
// Fedora does not ship a tinyalsa-devel package, so we provide a minimal stub that
// keeps the host build compiling without actual audio hardware access.

#ifdef __cplusplus
extern "C" {
#endif

struct pcm_config {
    unsigned int channels;
    unsigned int rate;
    unsigned int period_size;
    unsigned int period_count;
    unsigned int format;
    unsigned int start_threshold;
    unsigned int stop_threshold;
    unsigned int silence_threshold;
};

enum {
    PCM_IN = 0,
    PCM_OUT = 1,
    PCM_FORMAT_S16_LE = 0
};

struct pcm {
    int fd;
};

static inline struct pcm* pcm_open(int /*card*/, int /*device*/, int /*stream*/,
        const struct pcm_config* /*config*/) {
    return nullptr;
}

static inline int pcm_is_ready(struct pcm* /*pcm*/) {
    return 0;
}

static inline const char* pcm_get_error(struct pcm* /*pcm*/) {
    return "tinyalsa shim: host audio backend unavailable";
}

static inline int pcm_write(struct pcm* /*pcm*/, const void* /*data*/, unsigned int /*size*/) {
    return 0;
}

static inline int pcm_read(struct pcm* /*pcm*/, void* /*data*/, unsigned int /*size*/) {
    return 0;
}

static inline int pcm_stop(struct pcm* /*pcm*/) {
    return 0;
}

static inline int pcm_close(struct pcm* /*pcm*/) {
    return 0;
}

static inline int pcm_get_buffer_size(struct pcm* /*pcm*/) {
    return 0;
}

struct mixer {
    int dummy;
};

struct mixer_ctl {
    int dummy;
};

static inline struct mixer* mixer_open(int /*card*/) {
    return nullptr;
}

static inline int mixer_get_num_ctls(struct mixer* /*mixer*/) {
    return 0;
}

static inline struct mixer_ctl* mixer_get_ctl(struct mixer* /*mixer*/, int /*index*/) {
    return nullptr;
}

static inline const char* mixer_ctl_get_name(struct mixer_ctl* /*ctl*/) {
    return "tinyalsa-shim";
}

static inline const char* mixer_ctl_get_type_string(struct mixer_ctl* /*ctl*/) {
    return "tinyalsa-shim";
}

static inline int mixer_ctl_get_num_values(struct mixer_ctl* /*ctl*/) {
    return 0;
}

static inline int mixer_close(struct mixer* /*mixer*/) {
    return 0;
}

#ifdef __cplusplus
}
#endif

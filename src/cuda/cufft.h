#ifndef CUFFT_H
#define CUFFT_H

#include <cufft.h>

extern "C" {
    void compute_fft(cufftComplex* input, cufftComplex* output, int size);
}

#endif

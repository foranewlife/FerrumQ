#include <cufft.h>
#include "cufft.h"

void compute_fft(cufftComplex* input, cufftComplex* output, int size) {
    cufftHandle plan;
    cufftPlan1d(&plan, size, CUFFT_C2C, 1);
    cufftExecC2C(plan, input, output, CUFFT_FORWARD);
    cufftDestroy(plan);
}

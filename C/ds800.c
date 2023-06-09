#include <stdlib.h>
#include <stdio.h>
#include <assert.h>
#include <math.h>
#include <string.h>
#include <omp.h>
#include "common.h"

/** pi / 2 */
#define PI_2 (3.14159 / 2.0)
/** distribution function used to smudge pixels together */
#define SKEW(X) ((PI_2 - atan(X)) / PI_2)

/** smudge pixels in a target pixel's vecinity together */
static inline void _modif(pixel_t* p, img_t const img, float vh, float vw, size_t i, size_t j)
{
    float r = .0f, g = .0f, b = .0f;
    float s = .0f;
    long ii, jj;

    // define the vecinity
    long minh = (long)(((long)i - 1) * (vh));
    long minw = (long)(((long)j - 1) * (vw));
    long maxh = (long)(((long)i + 1) * (vh));
    long maxw = (long)(((long)j + 1) * (vw));

    // transform
    for(ii = minh; ii <= maxh; ++ii) {
        if(ii < 0 || ii >= img.h) continue;
        for(jj = minw; jj <= maxw; ++jj) {
            if(jj < 0 || jj >= img.w) continue;

            float y = (vh * i) - (ii);
            float x = (vw * j) - (jj);
            float vv = (vh + vw) / 2.0;
            float dist = abs( y * y + x * x ) / (vv * vv);
            float skew = 0.0;

            dist = dist * dist;
            skew = SKEW(dist);

            r += skew * A(img, ii, jj).r;
            g += skew * A(img, ii, jj).g;
            b += skew * A(img, ii, jj).b;
            s += skew;
        }
    }

    // save pixel
    (*p).r = (int)(r / s);
    (*p).g = (int)(g / s);
    (*p).b = (int)(b / s);
}

typedef struct {
    img_t* ret;
    img_t img;
    float vh, vw;
    size_t i, retw;
} tdata_t;

static void tDS(void* data)
{
    tdata_t* mydata = (tdata_t*)data;
    size_t j;
    for(j = 0; j < mydata->retw; ++j) {
        _modif(&A((*(mydata->ret)), mydata->i, j), mydata->img, mydata->vh, mydata->vw, mydata->i, j);
    }
}

/** downsample a picture to 800x800 */
img_t downSample(img_t const img, int w, int h)
{
    {
        float f1 = (float)w / (float)img.w;
        float f2 = (float)h / (float)img.h;
        float f = f1 < f2 ? f1 : f2;
        w = (int)(img.w * f);
        h = (int)(img.h * f);
    }

    img_t ret = { w, h, (pixel_t*)malloc(w * h * sizeof(pixel_t)) };

    int i, j;

    float vw = (float)img.w / ret.w;
    float vh = (float)img.h / ret.h;

    if(img.w < ret.w || img.h < ret.h) {
        printf("copping out\n");
        memcpy(ret.pixels, img.pixels, sizeof(pixel_t) * img.w * img.h);
        ret.w = img.w;
        ret.h = img.h;
        return ret;
    }

    assert(img.w >= ret.w && img.h >= ret.h);

    tdata_t* datas = (tdata_t*)malloc(sizeof(tdata_t) * ret.h);

#pragma omp parallel for
    for(i = 0; i < ret.h; ++i) {
        tdata_t* data = &datas[i];
        data->ret = &ret;
        data->img = img;
        data->vh = vh;
        data->vw = vw;
        data->i = i;
        data->retw = ret.w;
        tDS(data);
    }

    free(datas);

    return ret;
}

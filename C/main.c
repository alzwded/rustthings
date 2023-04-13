#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include "common.h"
#include <getopt.h>

// external subroutines
extern img_t readPixels(char const*);
extern int savePixels(img_t const, char const*);

extern img_t downSample(img_t const, int w, int h);

extern img_t mobord(img_t const);

extern img_t faith(img_t const);
extern img_t rgfilter(img_t const);

/** generate the output file name */
static char* getOutFileName(char const* file, const char* ext)
{
    size_t len = strlen(file);
    char const* i = file + len - 1;
    for(; i - file >= 0; --i) {
        if(*i == '/') break;
        if(*i == '.') {
            char* ret = (char*)malloc((i - file) + strlen(ext) + 1);
            strncpy(ret, file, i - file);
            strcpy(ret + (i - file), ext);
            return ret;
        }
    }
    char* ret = (char*)malloc(len + strlen(ext));
    strcpy(ret, file);
    strcpy(ret + len, ext);
    return ret;
}

/** apply transformations on a file */
static void process(char const* file, int w, int h)
{
    printf("%s: reading pixels\n", file);
    img_t img = readPixels(file);
    img_t alt = img;
    img = downSample(img, w, h);
    free(alt.pixels);

    char* outFile = getOutFileName(file, ".out.jpg");
    printf("%s: saving as %s\n", file, outFile);
    savePixels(img, outFile);
    free(outFile);
    free(img.pixels);
}

int main(int argc, char* argv[])
{
    int i;
    int opt;
    int w = 800, h = 600;

    while((opt = getopt(argc, argv, "r:V")) != -1) {
        switch(opt) {
            case 'r':
                {
                    int read = sscanf(optarg, "%dx%d", &w, &h);
                    if(read != 2) abort();
                }
                break;
            case 'V':
                printf("%s\n", VERSION);
                exit(1);
                break;
            default:
                abort();
                break;
        }
    }

    if(optind >= argc) {
        fprintf(stderr, "Missing files\n");
        exit(2);
    }


    for(i = optind; i < argc; process(argv[i++], w, h))
        ;

    return 0;
}

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <getopt.h>

#include <imageproc_bindings.h>

/** apply transformations on a file */
static void process(char const* file, int w, int h)
{
    Resolution resolution = {
        w, h
    };
    if(convertw(file, NULL, &resolution) == -1) {
        fprintf(stderr, "Failed to process %s: %s\n",
                file,
                errstr());
    }
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

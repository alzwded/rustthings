#include "test.h"
#include <dlfcn.h>
#include <unistd.h>

int magic(const char** s)
{
    char buf[4096]; // Note: can't use thread_local as that's a GNU extension o_O and not in clang
    int rval = 0;
    ssize_t buflen = 0;
    void* h = NULL;
    const char* (*gnu_get_libc_version)(void) = NULL;

    // resolve libc.so
    if((buflen = readlink("/lib/libc.so.6", buf, sizeof(buf))) == -1) {
        rval = -3;
        goto magic_out;
    }
    // readlink does not write out a null terminator
    if(buflen >= (ssize_t)sizeof(buf)) {
        // ... but it theoretically could fill up our buffer
        rval = -4;
        goto magic_out;
    }
    buf[buflen] = '\0';

    // open the so
    h = dlopen(buf, RTLD_NOW|RTLD_LOCAL);
    if(!h) {
        rval = -1;
        goto magic_out;
    }

    // resolve the symbol, if it's there
    gnu_get_libc_version = (const char* (*)(void))dlsym(h, "gnu_get_libc_version");
    if(!gnu_get_libc_version) {
        rval = -2;
        goto dlsym_failed;
    }

    // call the symbol
    *s = gnu_get_libc_version();

    // end
dlsym_failed:
    dlclose(h);
magic_out:
    return rval;
}

#ifndef TEST_H
# define TEST_H
#endif
#ifdef __cplusplus
extern "C" {
#endif

    /** plops open libc with dlopen and calls gnu_get_libc_version if any
      *
      * returns -1 on error, 0 on success
      *
      * the output parameter is written to only if successful
      */
    int magic(const char**);

#ifdef __cplusplus
}
#endif

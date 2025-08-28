#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <stdarg.h>

#define ARRAY_LEN(arr) (sizeof(arr) / sizeof((arr)[0]))

char* sprintf_to_heap(const char *fmt, ...) {
    va_list args;
    va_start(args, fmt);

    int len = vsnprintf(NULL, 0, fmt, args);
    va_end(args);

    if (len < 0) return NULL;

    char *buf = malloc(len + 1);
    if (!buf) return NULL; // ufck

    va_start(args, fmt);
    vsnprintf(buf, len + 1, fmt, args);
    va_end(args);

    // memory owned by caller
    return buf;
}


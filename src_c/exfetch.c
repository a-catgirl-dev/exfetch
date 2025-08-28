#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <sys/sysinfo.h>

#include "exfetch.h"

// see branch rewrite-it-in-rust
// this is gonna be a 1:1 copy because im fricking lazy to do it the C way
// (except possibly for some memcpy(2) calls caused by trying to make the borrow checker happy)
// probably
// idk

typedef char* (*StringFn)(void);
typedef char* (*OptionalStringFn)(void);

typedef struct {
    // Option<fn() -> String>
    StringFn func1;
    // Option<fn() -> Option<String>>
    OptionalStringFn func2;
    const char* label;
} FunctionRegistry;

// actual readouts
char* memory_readout(void) {
    struct sysinfo info;
    sysinfo(&info);

    char* ret = sprintf_to_heap("%lu/%lu", info.totalram - info.freeram, info.totalram);
    return ret;
}
// end actual readouts

FunctionRegistry HARDWARE_REGISTRY[4] = {
    {NULL, NULL, "CPU"},
    {memory_readout, NULL, "Mem"},
    {NULL, NULL, "Arch"},
    {NULL, NULL, "Up"},
};

FunctionRegistry SOFTWARE_REGISTRY[5] = {
    {NULL, NULL, "sh"},
    {NULL, NULL, "PKGs"},
    {NULL, NULL, "OS"},
    {NULL, NULL, "Init"},
    {NULL, NULL, "Swap"},
};

#define LENGTHS ARRAY_LEN(HARDWARE_REGISTRY) + ARRAY_LEN(SOFTWARE_REGISTRY)

typedef struct {
    const char* icon;
    char* string;
} Cache;

/**
 * I KNOW A DYNAMIC ARRAY WHEN I SEE ONE
 * - tsoding (probably, misattributed (2025) legit punjabi no virus)
 */
typedef struct {
    Cache *cache;
    size_t count;
    size_t capacity;
} RegistryCache;

void push_rc(RegistryCache *rc, char* string, const char* icon) {
    if (rc->count >= rc->capacity) {
        if (rc->capacity == 0) rc->capacity = 1;
        else rc->capacity *= 2; // probably not very smart in this case for exfetch, but yolo

        puts("reallocating rc");
        rc->cache = realloc(rc->cache, rc->capacity * sizeof(*rc->cache));
    }
    size_t count = rc->count++;
    rc->cache[count].string = string;
    rc->cache[count].icon = icon;
}

void deallocate_rc(RegistryCache *rc) {
    // free everything else first
    for (size_t i = 0; i < rc->count; i++) {
        free(rc->cache[i].string);
        rc->cache[i].string = NULL;
    }
    // and then free ourselves (from this eternal hell)
    free(rc->cache);
    rc->cache = NULL;
}

void iterate(RegistryCache* registry, StringFn func1, OptionalStringFn func2, const char* icon, int lengths[LENGTHS], int id) {
    if (func1 != NULL) {
        char* str = func1();
        int len = strlen(str);

        lengths[id] = len;
        push_rc(registry, str, icon);
    } else if (func2 != NULL) {
        char* str = func2();
        if (str == NULL) return;

        int len = strlen(str);

        lengths[id] = len;
        push_rc(registry, str, icon);
    } else {
        // puts("invalid state: both function ptrs are NULL. aborting");
        // exit(1);
        // __builtin_unreachable();
    }
}

void print(Cache* registry, int length) {
    printf("│ %s ~ %s (%i)\n", registry->icon, registry->string, length);
}

int main(void) {
    RegistryCache hardware_registry_cache = {0};
    RegistryCache software_registry_cache = {0};

    int lengths[LENGTHS] = {0};

    for (int i = 0; i < 4; i++) {
        // printf("%i: %p\n", i, HARDWARE_REGISTRY[i].func1);
        iterate(&hardware_registry_cache, HARDWARE_REGISTRY[i].func1, HARDWARE_REGISTRY[i].func2, HARDWARE_REGISTRY[i].label, lengths, i);
    }

    for (size_t i = 0; i < hardware_registry_cache.count; i++) {
        print(&hardware_registry_cache.cache[i], lengths[i]);
    }

    deallocate_rc(&hardware_registry_cache);
    deallocate_rc(&software_registry_cache);
}


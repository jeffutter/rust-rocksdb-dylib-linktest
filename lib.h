#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct DB DB;

uintptr_t strlen(const char *s);

struct DB *rust_open(const char *path);

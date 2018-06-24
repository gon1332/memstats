#include <stdio.h>
#include <stdlib.h>


static void check_alloc(const void *ptr, const char *fn);


int main(void)
{
    char *p;
    for (int i = 0; i < 5; i++) {
        check_alloc(p = malloc(10), "malloc");
    }

    check_alloc(p = calloc(5, 1), "calloc");

    check_alloc(p = realloc(p, 20), "realloc");

    free(p);

    return EXIT_SUCCESS;
}


void check_alloc(const void *ptr, const char *fn)
{
    if (!ptr) {
        perror(fn);
        exit(EXIT_FAILURE);
    }
}

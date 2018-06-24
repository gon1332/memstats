#include <stdio.h>
#include <stdlib.h>


int main(void)
{

    int *p = malloc(10);
    if (!p) {
        perror("malloc");
        exit(EXIT_FAILURE);
    }
    free(p);

    return EXIT_SUCCESS;
}

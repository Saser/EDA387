#include <errno.h>
#include <libgen.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/param.h>

#include "resolve.h"

int main(int argc, char* argv[]) {
    if (argc != 2) {
        printf("Wrong number of arguments.\n");
        print_usage(argv[0]);
        exit(1);
    }

    char hostname[HOST_NAME_MAX];
    if (gethostname(hostname, HOST_NAME_MAX) != 0) {
        perror("Error resolving our hostname");
        exit(1);
    }
    printf("Our hostname: `%s`\n", hostname);
}

void print_usage(char *name) {
    /* The man page for basename(3) states that the POSIX variant of the
     * function -- which is what we are using -- may modify the string pointed
     * to by its argument, so we are advised to make a copy of it. */
    size_t n = strlen(name);
    char copy[n];
    strcpy(copy, name);
    printf("Usage: %s <hostname>\n", basename(copy));
}

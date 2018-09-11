#include <errno.h>
#include <libgen.h>
#include <netdb.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include <sys/param.h>
#include <sys/socket.h>
#include <sys/types.h>

#include "resolve.h"

int main(int argc, char* argv[]) {
    if (argc != 2) {
        fprintf(stderr, "Wrong number of arguments.\n");
        print_usage(argv[0]);
        exit(1);
    }

    char hostname[HOST_NAME_MAX];
    if (gethostname(hostname, HOST_NAME_MAX) != 0) {
        perror("Error resolving our hostname");
        exit(1);
    }

    struct addrinfo hints;
    struct addrinfo *result;

    memset(&hints, 0, sizeof(struct addrinfo));
    hints.ai_family = AF_INET;
    hints.ai_addr = NULL;
    hints.ai_canonname = NULL;
    hints.ai_next = NULL;

    int gai_result = getaddrinfo(argv[1], NULL, &hints, &result);
    if (gai_result != 0) {
        fprintf(stderr, "Error resolving '%s': %s\n", argv[1], gai_strerror(gai_result));
        exit(2);
    }
    printf("I resolved something\n");
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

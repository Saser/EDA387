#include <errno.h>
#include <libgen.h>
#include <netdb.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include <arpa/inet.h>
#include <sys/param.h>
#include <sys/socket.h>
#include <sys/types.h>

void print_usage(char *name);

int main(int argc, char* argv[]) {
    if (argc != 2) {
        fprintf(stderr, "Wrong number of arguments.\n");
        print_usage(argv[0]);
        return 1;
    }

    char hostname[HOST_NAME_MAX + 1];
    if (gethostname(hostname, HOST_NAME_MAX + 1) != 0) {
        perror("Error resolving our hostname");
        return 1;
    }

    struct addrinfo hints;
    struct addrinfo *result;

    memset(&hints, 0, sizeof(struct addrinfo));
    hints.ai_family = AF_UNSPEC;
    hints.ai_socktype = SOCK_STREAM;
    hints.ai_protocol = IPPROTO_TCP;
    hints.ai_addr = NULL;
    hints.ai_canonname = NULL;
    hints.ai_next = NULL;

    int gai_result = getaddrinfo(argv[1], NULL, &hints, &result);
    if (gai_result != 0) {
        fprintf(stderr, "Error resolving '%s': %s\n", argv[1], gai_strerror(gai_result));
        return 2;
    }

    int result_count = 0;
    for (struct addrinfo *rp = result; rp != NULL; rp = rp->ai_next) {
        char address[rp->ai_addrlen];
        void *addr_p = NULL;
        if (rp->ai_family == AF_INET) {
            addr_p = &((struct sockaddr_in *) rp->ai_addr)->sin_addr;
        } else if (rp->ai_family == AF_INET6) {
            addr_p = &((struct sockaddr_in6 *) rp->ai_addr)->sin6_addr;
        } else {
            fprintf(stderr, "Unknown address family: %d\n", rp->ai_family);
            return 3;
        }

        if (inet_ntop(rp->ai_family, addr_p, address, rp->ai_addrlen) == NULL) {
            perror("Error converting address to string");
            return 4;
        }

        if (rp->ai_family == AF_INET) {
            printf("IPv4 address: ");
        } else {
            printf("IPv6 address: ");
        }
        printf("%s\n", address);

        result_count += 1;
    }
    printf("got a total of %d results\n", result_count);
    freeaddrinfo(result);

    return 0;
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

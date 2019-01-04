/*
  Test:
  the st_size field of stat struct in the conditional statement.
  if the conditional statement is failed, it tells us we should resize the input.
*/
#include "stdio.h"
#include "stdint.h"
#include "stdlib.h"
#include "string.h"
#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>

int main (int argc, char** argv) {
  if (argc < 2) return 0;

  struct stat sb;
  if (stat(argv[1], &sb) == -1) {
    perror("stat");
    exit(EXIT_FAILURE);
  }


  if (sb.st_size > 100) {
    printf("hey, you hit it \n");
    abort();

  }
 return 0;
}

/*
  test:
  memcmp function.
*/

#include "stdint.h"
#include "stdio.h"
#include "stdlib.h"
#include "string.h"

int main(int argc, char **argv) {
  if (argc < 2)
    return 0;

  FILE *fp;
  char buf[255];
  size_t ret;

  fp = fopen(argv[1], "rb");

  if (!fp) {
    // printf("st err\n");
    return 0;
  }

  int len = 20;

  ret = fread(buf, sizeof *buf, len, fp);
  fclose(fp);
  if (ret < len) {
    // printf("input fail \n");
    return 0;
  }

  char b[10] = {1, 1, 1, 1, 1, 2, 3, 4, 5, 0};

  if (memcmp(b, buf, 9) == 0) {
    abort();
  }

  return 0;
}

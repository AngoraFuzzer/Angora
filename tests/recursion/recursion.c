/*
  Test:
  Recursion
*/
#include "stdint.h"
#include "stdio.h"
#include "stdlib.h"
#include "string.h"

int __attribute__((noinline)) bar(int *buf, int i, int len) {
  if (i > 1000) {
    return 0;
  }
  if (buf[i % len] == 66) {
    bar(buf, i + 1, len);
  }
}

int main(int argc, char **argv) {
  if (argc < 2)
    return 0;

  FILE *fp;
  char buf[255];
  size_t ret;

  fp = fopen(argv[1], "rb");

  if (!fp) {
    printf("st err\n");
    return 0;
  }

  int len = 100;

  ret = fread(buf, sizeof *buf, len, fp);
  fclose(fp);
  if (ret < len) {
    printf("input fail \n");
    return 0;
  }

  int32_t x = 0;

  memcpy(&x, buf + 1, 4); // x 0 - 1

  bar(buf, 0, len);

  return 0;
}

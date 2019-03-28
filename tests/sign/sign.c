/*
  Test:
  to verify that the sign of y is neg.
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
    printf("st err\n");
    return 0;
  }

  int len = 20;

  ret = fread(buf, sizeof *buf, len, fp);
  fclose(fp);
  if (ret < len) {
    printf("input fail \n");
    return 0;
  }

  uint16_t x = 0;

  memcpy(&x, buf + 1, 2);

  // if y is less than 32 bits, it has not nsw flag.
  int16_t y = x * -3;

  if (y == -12) {
    printf("hey, you hit it \n");
    abort();
  }

  return 0;
}

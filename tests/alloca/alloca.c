/*
  Test:
  a[x] will alloca memory.
  we test that if we can make make it try to apply large memory to triger crash.

*/

#include "stdio.h"
#include "stdint.h"
#include "stdlib.h"
#include "string.h"

int main (int argc, char** argv) {
  if (argc < 2) return 0;

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
  memcpy(&x, buf + 1, 2); // x 0 - 1

  int a[x];

  memset(a, 0, x);

  int sum = 0;
  for (int i = 0; i < 1; i ++) {
    sum += a[i];
  }

  printf("sum %d\n", sum);

  return 0;
}

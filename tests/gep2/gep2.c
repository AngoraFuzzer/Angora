/*
  Test:
  GEP/LEA ins.
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

  int len = 10;

  ret = fread(buf, sizeof *buf, len, fp);
  fclose(fp);
  if (ret < len) {
    printf("input fail \n");
    return 0;
  }

  uint8_t x = 0;
  int a[244];
  memset(a, 0, 244);

  memcpy(&x, buf + 8, 1);

  if (a[x] > 0) {

    printf("hey, you hit it \n");
    abort();
  }
  return 0;
}

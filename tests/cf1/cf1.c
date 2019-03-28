/*
  Test:
  implicit data flow due to simple if.
  The easiest case.
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

  int8_t x = 0;
  int32_t y = 0;
  int32_t m = 0;

  memcpy(&x, buf, 1);
  memcpy(&y, buf + 8, 4);
  memcpy(&m, buf + 15, 4);
  int z = 0;
  if (x == 1) {
    z = 123;
  } else {
    z = 998;
  }

  if (z == 123) {

    printf("hey, you hit it \n");
    abort();
  }
  return 0;
}

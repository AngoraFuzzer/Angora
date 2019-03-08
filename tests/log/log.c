/*
  Test:
  Nested `if` conditional statements.
  It is difficult for other fuzzers, but it is easy for Angora.
*/
#include "stdint.h"
#include "stdio.h"
#include "stdlib.h"
#include "string.h"
#include <math.h>

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
  // dfsan_read_label(&(len), sizeof *buf);
  ret = fread(buf, sizeof *buf, len, fp);
  fclose(fp);
  if (ret < len) {
    printf("input fail \n");
    return 0;
  }

  uint64_t x = 0;

  memcpy(&x, buf, 8);
  if (log(x) == 1.0) {
    printf("hey, you hit it \n");
    abort();
  }
  return 7;
}

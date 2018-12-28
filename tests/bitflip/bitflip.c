/*
  Test:
  bitflip
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

  uint32_t x = 0;
  memcpy(&x, buf, 4);

  if (
      (1 & (x >> 29)) &&
      !(1 & (x >> 28)) &&
      (1 & (x >> 27)) &&
      (1 & (x >> 26)) &&
      !(1 & (x >> 25)) &&
      (1 & (x >> 24)) &&
      (1 & (x >> 23)) &&
      (1 & (x >> 22)) &&
      !(1 & (x >> 21)) &&
      !(1 & (x >> 20)) &&
      (1 & (x >> 19)) &&
      (1 & (x >> 18))
      ){
    abort();
  }

  return 0;
}

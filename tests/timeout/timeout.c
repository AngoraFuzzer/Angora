/*
  Test:
  It is not a crash.
  TTTTTTTTTTTTTTTTIMEOUT!
 */
#include "stdint.h"
#include "stdio.h"
#include "stdlib.h"
#include "string.h"
#include "unistd.h"

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

  uint16_t x = 0;

  memcpy(&x, buf + 1, 2); // x 0 - 1

  if (x == 0xF000) {
    sleep(5);
  }

  return 0;
}

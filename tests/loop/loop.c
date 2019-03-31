/*
  Test:
  Loops
*/

#include "stdint.h"
#include "stdio.h"
#include "stdlib.h"
#include "string.h"

int __attribute__((noinline)) bar(uint16_t x, uint16_t y) {

  if (x == y * y + 23) {
    return 1;
  } else {
    return 0;
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

  int len = 20;

  ret = fread(buf, sizeof *buf, len, fp);
  fclose(fp);
  if (ret < len) {
    printf("input fail \n");
    return 0;
  }

  uint16_t a[4];
  memcpy(&a[0], buf, 2);
  memcpy(&a[1], buf + 4, 2);
  memcpy(&a[2], buf + 10, 2);
  memcpy(&a[3], buf + 15, 2);

  for (int i = 0; i < 4; i++) {
    if (!bar(a[i], i)) {
      break;
    }

    if (i == 3) {
      abort();
    }
  }

  return 0;
}

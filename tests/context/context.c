/*
  Test:
  call context.
*/
#include "stdio.h"
#include "stdint.h"
#include "stdlib.h"
#include "string.h"

int __attribute__ ((noinline)) foo(uint64_t x, uint64_t y) {
  //int z = x - y + 10;
  if (x + y == 3122) return 1;
  return 0;
}

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
  uint32_t y = 0;

  memcpy(&x, buf, 4);
  memcpy(&y, buf + 8, 4);

  if (x > 41) {
    if (foo(y, 570)) {
      printf("hey \n");
      if (x == 12345) {
        abort();
      }
    }
  } else {
    if (foo(y, 312)) {
      printf("hey \n");
      abort();
    }
  }

  return 0;
}

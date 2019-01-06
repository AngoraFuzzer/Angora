/*
  Test:
  nested function calls.
  test that the taints are transferred coreectly.
 */
#include "stdio.h"
#include "stdint.h"
#include "stdlib.h"
#include "string.h"

void  __attribute__ ((noinline))  bar(int y) {
  if(y == 12334) {
    printf("hey, you hit it \n");
    abort();
  }
}


void  __attribute__ ((noinline))  foo(int y) {
  bar(y - 1);
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

  int32_t y = 0;
  memcpy(&y, buf + 4, 4); // y 4 - 7

  foo(y);

  int x = y;
  if (x == 123) {
    abort();
  }

  return 0;
}

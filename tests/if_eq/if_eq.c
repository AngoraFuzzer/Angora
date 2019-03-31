/*
  Test:
  Simple `if` conditional statement.
  its both side are variable influenced by the input.
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

  uint16_t x = 0;
  uint16_t y = 0;

  memcpy(&x, buf + 1, 2); // x 0 - 1
  memcpy(&y, buf + 4, 2); // y 4 - 7

  if (x == y) {

    printf("hey, you hit it \n");
    abort();
  }
  return 0;
}

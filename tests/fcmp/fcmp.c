/*
  Test:
  comparison for float.
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

  double x = 0;
  float y = 0;
  memcpy(&x, buf, sizeof x);
  memcpy(&y, buf + 10, sizeof y);

  if (x == 1.2) {
    printf("hey, you hit it2 \n");
  }
  if (y == 2.1f) {
    printf("hey, you hit it \n");
    abort();
  }

  return 0;
}

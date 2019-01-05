/*
  Test:
  comparison for pointer.
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

  int len = 8;
  ret = fread(buf, sizeof *buf, len, fp);
  fclose(fp);
  if (ret < len) {
    printf("input fail \n");
    return 0;
  }

  void* x = NULL;
  printf("%lu\n", x);
  memcpy(&x, buf, sizeof x);
  printf("%lu\n", x);
  if (!x) {
    abort();
  }

  return 0;
}

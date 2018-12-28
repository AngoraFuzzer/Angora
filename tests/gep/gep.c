/*
  Test:
  Exploit GEP ins.
 */

#include "stdio.h"
#include "stdint.h"
#include "stdlib.h"
#include "string.h"
int arr[] = {1, 12, 123, 1234, 555, 11};

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
  memcpy(&x, buf + 1, 4); // x 0 - 1

  printf("%d\n", arr[x%6]);

  return 0;
}

/*
  Test:
  implicit data flow due to the value computed by if in loop.
  The arr should contain three '3'.
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

  int i = 0;
  int j = 0;
  int8_t arr[10];
  memcpy(arr, buf, 10);
  for (i = 0; i < 10; i++) {
    if (arr[i] == 3) j++;
  }

  if (j == 3) {
    abort();
  }

  return 0;
}

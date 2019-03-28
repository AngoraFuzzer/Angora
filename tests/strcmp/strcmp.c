/*
  Test:
  strcmp function.
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
    // printf("st err\n");
    return 0;
  }

  int len = 20;

  ret = fread(buf, sizeof *buf, len, fp);
  fclose(fp);
  if (ret < len) {
    // printf("input fail \n");
    return 0;
  }

  char a[10];
  char b[10] = {1, 1, 1, 1, 1, 2, 3, 4, 5, 0};

  /* int dd = memcmp(buf, "12313", 5); */
  /* if (dd) { */
  /*   printf("hey, you hit it \n"); */
  /* } */

  memcpy(a, buf, 9);
  a[9] = 0;

  if (strcmp(a, b) == 0) {
    printf("hey, you hit it \n");
    abort();
  }

  a[4] += 10;
  if (strcmp(a, b) == 0) {
    // printf("hey, you hit it \n");
    abort();
  }

  a[4] += 244;
  if (strcmp(a, b) == 0) {
    // printf("hey, you hit it \n");
    abort();
  }

  a[4] -= 99;
  if (strcmp(a, b) == 0) {
    // printf("hey, you hit it \n");
    abort();
  }

  return 0;
}

/*
  Test:
  Test the taint propagation is work with >> and & OPs.
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

  int len = 20;

  ret = fread(buf, sizeof *buf, len, fp);
  fclose(fp);
  if (ret < len) {
    printf("input fail \n");
    return 0;
  }

  uint32_t x = 0;

  memcpy(&x, buf, 4);
  /* if ((int)(x & 0xFF) == 12) { */
  if (((int)(x >> 24) & 0xFF) == 11 && ((int)(x >> 16) & 0xFF) == 22 &&
      ((int)(x >> 8) & 0xFF) == 33 && (int)(x & 0xFF) == 44) {

    printf("hey, you hit it \n");
    abort();
  }
  return 0;
}

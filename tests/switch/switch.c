/*
  Test:
  Simple switch,
  contains one crash in one of its condition cases.
 */
#include "stdint.h"
#include "stdio.h"
#include "stdlib.h"
#include "string.h"

int main(int argc, char **argv) {
  /***** common part *******/
  if (argc < 2)
    return 0;
  char buf[255];
  int len = 20;
  FILE *fp;
  size_t ret;
  fp = fopen(argv[1], "rb");
  ret = fread(buf, sizeof *buf, len, fp);
  if (ret < len) {
    printf("input fail \n");
    return 0;
  }
  int b = 0;
  memcpy(&b, buf + 2, 4);
  int x = 0;
  memcpy(&x, buf + 6, 4);

  printf("cond: %d\n", b);
  switch (b) {
  case 12312213:
    printf("11\n");
    break;
  case 13201000:
    printf("22\n");
    break;
  case -1111:
    printf("3\n");
    break;
  case 3330000:
    printf("4\n");
    if (x == b) {
      abort();
    }
    break;
  case 5888:
    printf("5\n");
    break;
  case -897978:
    printf("6\n");
    break;
  default:
    printf("123\n");
    break;
  }
}

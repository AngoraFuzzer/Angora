/*
  Test:
  I just want to find the optimizations for switch..
 */
#include "stdio.h"
#include "stdint.h"
#include "stdlib.h"
#include "string.h"

int main (int argc, char** argv) {
  /***** common part *******/
  if (argc < 2) return 0;
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
  /*************************/
  int b = 0;
  memcpy(&b, buf + 2, 2);

  switch(b) {
  case 1:
    printf("11");
    break;
  case 2:
    printf("22");
    break;
  case 3:
    printf("3");
    break;
  case 4:
    printf("4");
    abort();
    break;
  case 5:
    printf("5");
    break;
  case 6:
    printf("6");
    break;
  case 7:
    printf("6");
    break;
  case 8:
    printf("6");
    break;
  case 9:
    printf("6");
    break;
  case 10:
    printf("6");
    break;
  case 9999:
    printf("6");
    break;
  case 10000:
    printf("6");
    abort();
    break;
  case 10001:
    printf("6");
    break;
  default:
    printf("123");
    break;
  }

}

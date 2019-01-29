/*
  Test:
  strcmp function.
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
    //printf("st err\n");
    return 0;
  }

  int len = 20;
  fgets(buf, len, fp);
  char a[20] = {1, 1, 1, 1, 7, 8, 9, 1, 45, 2, 88, 1,
                1, 2, 3, 4, 5, 0};


  char b[10] = {1, 1, 1, 1,
                1, 2, 3, 4, 5, 0};

  if (strcmp(buf, a) == 0) {
      printf("hey, you hit it \n");
      abort();
  }


  if (strcmp(buf, b) == 0) {
      printf("hey, you hit it \n");
      abort();
  }

  return 0;
}

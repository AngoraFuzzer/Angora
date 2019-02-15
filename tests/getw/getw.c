#include <stdio.h>
#include <stdlib.h>
#include <wchar.h>

int main (int argc, char** argv) {
  if (argc < 2) return 0;
  FILE * pFile;
  wint_t wc;
  pFile = fopen (argv[1], "r");
  if (pFile != NULL) {
    wc = getw(pFile);
    if (wc == 1819043090) {
      abort();
    }
    fclose (pFile);
  } else {
    perror("fopen");
  }
  return 0;
}

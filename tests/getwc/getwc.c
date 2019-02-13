// Refer: http://www.cplusplus.com/reference/cwchar/fgetwc/
// Refer: http://pubs.opengroup.org/onlinepubs/009696899/functions/getwc.html
/* getwc example */
#include <stdio.h>
#include <stdlib.h>
#include <wchar.h>

int main (int argc, char** argv) {
  if (argc < 2) return 0;
  FILE * pFile;
  wint_t wc;
  pFile = fopen (argv[1], "r");
  if (pFile!=NULL) {
    wc = getwc (pFile);
    if (wc == L'$') {
      abort();
    }
    fclose (pFile);
  } else {
    printf("I cannot open infile! TAT\n");
  }
  return 0;
}
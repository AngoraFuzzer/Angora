// Refer: http://www.cplusplus.com/reference/cwchar/fgetws/
/* fgetws example */
/*
  Note: fuzzer cannot solve this example.
  Fuzzer changes the input, and the new input might cause
  fgetws: Invalid or incomplete multibyte or wide character.
*/
#include <locale.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <wchar.h>

int main(int argc, char** argv) {
  if (argc < 2) return 0;
  setlocale(LC_ALL, "");
  FILE * pFile;
  wchar_t mystring [10000];

  pFile = fopen (argv[1] , "r");
  if (pFile != NULL) {
    if (fgetws (mystring , 10000 , pFile) != NULL ) {
      if (mystring[1] == WEOF) {
        abort();
      }
    } else {
      perror("fgetws");
    }
    fclose (pFile);
  } else {
    perror("fopen");
  }
  return 0;
}
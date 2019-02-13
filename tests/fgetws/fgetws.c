// Refer: http://www.cplusplus.com/reference/cwchar/fgetws/
/* fgetws example */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <wchar.h>

int main(int argc, char** argv) {
  if (argc < 2) return 0;
  FILE * pFile;
  wchar_t mystring [10000];

  pFile = fopen (argv[1] , "r");
  if (pFile != NULL) {
    if (fgetws (mystring , 10000 , pFile) != NULL ) {
      if (mystring[1] == WEOF) {
        abort();
      }
    } else {
      printf("Buffer is not long enough!\n");
    }
    fclose (pFile);
  } else {
     printf("I cannot open infile! TAT\n");
  }
  return 0;
}
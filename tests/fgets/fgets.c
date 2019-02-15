// Refer: https://www.tutorialspoint.com/c_standard_library/c_function_fgets.htm
/* fgets example */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char** argv) {
  if (argc < 2) return 0;
  FILE * pFile;
  char mystring [100];

  pFile = fopen (argv[1] , "r");
  if (pFile != NULL) {
    if (fgets (mystring , 100 , pFile) != NULL ) {
      if (mystring[2] == 'D' && mystring[3] == 'K') {
        printf("You got it!\n");
        abort();
      }
    } else {
      perror("fgets");
    }
    fclose (pFile);
  } else {
     perror("fopen");
  }
  return 0;
}
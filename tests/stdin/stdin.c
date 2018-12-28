/*
  Test:
  Angora supports stdin.
*/
#include <stdio.h>

int main () {
  int ch;
  while ((ch=getchar()) != EOF) {
    printf("%c\n", ch);
  }

}

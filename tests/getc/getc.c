#include <stdio.h>
#include <stdlib.h>

int main() {
  int a = getc(stdin);

  if (a == 10) { abort(); }
  if (a + a *10 - 2 == 119) { abort(); }

  return 0;
}
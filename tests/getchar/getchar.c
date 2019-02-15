#include <stdio.h>
#include <stdlib.h>
#include <wchar.h>

int main() {
  char c = getchar();
  if (c ==  '.')  {
    abort();
  }
  return 0;
}
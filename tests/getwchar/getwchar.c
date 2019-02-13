#include <stdio.h>
#include <stdlib.h>
#include <wchar.h>

int main() {
  wint_t wc = getwchar();
  if (wc ==  L'.')  {
    abort();
  }
  return 0;
}
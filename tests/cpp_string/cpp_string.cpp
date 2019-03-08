/*
  Test:
  c++ string
 */
#include <cerrno>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <sstream>
#include <string>

int main(int argc, char **argv) {
  if (argc < 2)
    return 0;

  std::FILE *fp = std::fopen(argv[1], "rb");
  if (!fp)
    return 0;
  std::string contents;
  std::fseek(fp, 0, SEEK_END);
  contents.resize(std::ftell(fp));
  std::rewind(fp);
  std::fread(&contents[0], 1, contents.size(), fp);
  std::fclose(fp);

  if (contents.size() < 6)
    return 0;

  // if (contents.substr(0, 7) == "iamback") {
  //   std::cout <<" hhe\n";
  //   abort();
  // }

  // if (contents[1] == 'y' && contents[2] == 'x') {
  //   abort();
  // }

  std::string val(contents);
  // char val[1000];
  // __builtin_memcpy(val, &contents[0], 4);
  if (val[0] == 'z' && val[1] == 'a' && val[2] == 'c') {
    std::cout << "wowowo" << std::endl;
    printf("wowow\n");
  }

  return 0;
}

/*
  Test:
  c++ fstream
 */
#include <string>
#include <cstdio>
#include <cerrno>
#include <iostream>
#include <fstream>
#include <cstdlib>
#include <cstring>

int main (int argc, char** argv) {
  if (argc < 2) return 0;
  std::fstream in_file;
  in_file.open(argv[1], std::ios::in | std::ios::binary);
  if (!in_file.is_open()) return 0;

  in_file.seekg (0, in_file.end);
  int length = in_file.tellg();
  in_file.seekg (0, in_file.beg);

  if (length <= 3) {
    return 0;
  }

  char *val = new char[length];

  in_file.read(val, length);

  if (val[0] == 'z' && val[1] == 'a' && val[2] == 'c') {
   abort();
  }

  return 0;
}

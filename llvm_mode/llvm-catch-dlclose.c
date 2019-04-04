
// https://bugs.llvm.org/show_bug.cgi?id=39321

int dlclose(void *handle) { return 0; }
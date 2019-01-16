# Run Angora on LAVA dataset
- Dataset: [Download](http://panda.moyix.net/~moyix/lava_corpus.tar.xz)

## Compile
In this tutorial, we use [gllvm](https://github.com/SRI-CSL/gllvm) to get LLVM bitcode, then use Angora to compile it. But you can choose other compile approach described in [Building a Target Program](build_target.md). 

``` sh
# we use base64 as the example, the other three programs are the same.
cd /path-to-base64/
CC=gclang CFLAGS="-g -O0" ./configure --disable-shared --prefix=`pwd`/lava-install
make
get-bc base64
~/angora-open-source/bin/angora-clang base64.bc -o base64.fast
USE_TRACK=1 ~/angora-open-source/bin/angora-clang base64.bc -o base64.tt 
```

## base64
- seeds: any random input
- command
```
~/angora/angora_fuzzer  -i ./input -o ./output -j 1 -t ./base64.tt -- ./base64.fast -d @@
```
- validation: `python3 ~/angora/tools/lava_validation.py ./output /path-to-lava/LAVA-M/base64/validated_bugs ./base64 -d`
- we can find 48 bugs. Also unlisted bugs: [274, 521, 526, 527].

## md5sum

- Fix lava bugs (they exists in my environment).
``` c
// In src/md5sum.c line 541
- char *filename IF_LINT ( = NULL);
+ char* filename = NULL;
// In src/md5sum.c line 543
- unsigned char *hex_digest IF_LINT ( = NULL);
+ unsigned char *hex_digest = NULL; 
```
- seeds: md5sum need providing real program in your file system to check the md5 digests. Otherwise, Angora can't pass those checks.
```
870c9a2edda2a9400179487e4be0f8fe  /bin/tempfile
43ff9c3b7c5f3d045feb32ca6bad3348  /bin/touch
1091007513eea6d4158a3563ae3be888  /bin/rm
1091007513eea6d4158a3563ae3be888  /bin/rm
```

- command
```
~/angora/angora_fuzzer  -i ./input -o ./output -j 1 -t ./md5sum.tt -- ./md5sum.fast -c @@
```
- Angora can find 57 bugs, including 4 unlisted bugs: [281, 287, 314, 499]. But Angora can not find [555, 387, 571, 305]. Because the related variables for these bugs are collected in line 618-631 in src/md5sum.c, they can not be visited if we have set hex_digest as NULL to fix the *real* bug.

## uniq
- uniq may use some inline code to read input. We need to modify the makefile to make it use a libc function call to read input, so we can hook these functions.
```sh
# run below scripts after configure
find . -type f -name "*.h" -exec sed -i 's/#define\s*HAVE_GETC_UNLOCKED\s*[0-9]/#undef HAVE_GETC_UNLOCKED/' {} +
find . -type f -name "*.h" -exec sed -i 's/#define\s*HAVE_DECL_GETC_UNLOCKED\s*[0-9]/#undef HAVE_GETC_UNLOCKED/' {} +
```

## who
- seeds: we create a minimal seed by:
``` c
#include <utmp.h>
#include <stdio.h>
#include <stdlib.h>
int main() {
  struct utmp *u;
  while((u = getutent()))
    {
        FILE *f1 = fopen("utmp0", "wb");
        int r1 = fwrite(u, sizeof (struct utmp), 1, f1);
        fclose(f1);

        break;
    }
  endutent();
}
```

- the program who won't output bug id, so we need modify `src/who.c`
```c
// move to somewhere after #include "..."
unsigned int lava_get(unsigned int bug_num) {

#define SWAP_UINT32(x) (((x) >> 24) | (((x) & 0x00FF0000) >> 8) | (((x) & 0x0000FF00) << 8) | ((x) << 24))
  if (0x6c617661 - bug_num == lava_val[bug_num] ||
      SWAP_UINT32(0x6c617661 - bug_num) == lava_val[bug_num]) {
    printf("Successfully triggered bug %d, crashing now!\n", bug_num);
    fflush(0);
    //exit(0);
  }
  else {
    //printf("Not successful for bug %d; val = %08x not %08x or %08x\n", bug_num, lava_val[bug_num], 0x6c617661 + bug_num, 0x6176616c + bug_num);
  }
  return lava_val[bug_num];
}
```

- command
``` sh
# -M 0 to set unlimited memory since who has "memory exhausted" warning.
~/angora/angora_fuzzer  -i ./input -o ./output -M 0 -j 1 -t ./who.tt -- ./who.fast @@
```

- Angora can find 1400~1700 bugs (including unlisted bugs).

**Update Jan 15, 2019**

Angora is able to find ~2400 bugs after using the fix explained here [Angora LAVA `who` Fix](./lava-who-fix.md).

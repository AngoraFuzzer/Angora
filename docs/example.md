# Fuzz program *file* by Angora

## Build programs

### Download
``` sh
wget https://github.com/file/file/archive/FILE5_32.tar.gz
tar -xvzf FILE5_32.tar.gz
```

### Compile program for tracking
``` sh
cp -r file-FILE5_32 track
cd track
autoreconf -i
CC=~/angora/bin/angora-clang ./configure --prefix=`pwd`/install --disable-shared

USE_TRACK=1 make
```

We found some errors:
```
/usr/bin/ld: ./.libs/libmagic.a(compress.o): in function `uncompresszlib':
/home/xx/example/track/src/compress.c:507: undefined reference to `dfs$inflateInit_'
/usr/bin/ld: /home/xx/example/track/src/compress.c:507: undefined reference to `dfs$inflateInit2_'
/usr/bin/ld: /home/xx/example/track/src/compress.c:511: undefined reference to `dfs$inflate'
/usr/bin/ld: /home/xx/example/track/src/compress.c:516: undefined reference to `dfs$inflateEnd'
/usr/bin/ld: /home/xx/example/track/src/compress.c:525: undefined reference to `dfs$zError'
```
Because *file* depends on *zlib*, we should models *zlib*'s function in taint analysis. we create a file call *zlib_abilist.txt*, and it contains:
```
fun:inflateInit_=uninstrumented
fun:inflateInit2_=uninstrumented
fun:inflateInit2__=uninstrumented
fun:inflate=uninstrumented
fun:inflateEnd=uninstrumented
fun:zError=uninstrumented
fun:inflateInit_=discard
fun:inflateInit2_=discard
fun:inflateInit2__=discard
fun:inflate=discard
fun:inflateEnd=discard
fun:zError=discard
```
We ignore all taints while calling *zlib*'s function.
You also can get it by:  (described in [Build Target](./build_target.md))
```
./angora/tools/gen_library_abilist.sh /usr/lib/x86_64-linux-gnu/libz.so  discard > zlib_abilist.txt
```

Then, we set it in our environment and compile again.
```
export ANGORA_TAINT_RULE_LIST=~/path-to/zlib_abilist.txt 
make clean
USE_TRACK=1 make
make install
```

### Compile program for branch counting
``` sh
cd ..
cp -r file-FILE5_32 fast
cd fast
autoreconf -i
CC=~/angora/bin/angora-clang ./configure --prefix=`pwd`/install --disable-shared
make 
make install
```


## Seeds
```
cd ..
mkdir seeds
echo "Hello World" > seeds/seed.txt
```

## Run Angora

```
 ~/angora/angora_fuzzer -i seeds -o output -t ./track/install/bin/file -- ./fast/install/bin/file -m ./fast/install/share/misc/magic.mgc @@   
```

## Re-run
```
# Termination with ^C
~/angora/angora_fuzzer -i - -o output -t ./track/install/bin/file -- ./fast/install/bin/file -m ./fast/install/share/misc/magic.mgc @@
```

## Run alongside AFL
Angora has implemented some AFL like random mutation approaches, but they are too simple. You can disable it and run Angora alongside AFL.
### Build with AFL
```
cp -r file-FILE5_32 afl
cd afl
autoreconf -i
CC=~/afl/afl-clang-fast ./configure --prefix=`pwd`/install --disable-shared
make 
make install
```

### Run!!!
```
~/afl/afl-fuzz  -i seeds -o output -S afl_s -- ./afl/install/bin/file @@

# --sync_afl to allow sync seeds with AFL
# -A to disable AFL's random mutation in Angora.
~/angora/angora_fuzzer --sync_afl -A -i seeds -o output -t ./track/install/bin/file -- ./fast/install/bin/file -m ./fast/install/share/misc/magic.mgc @@   
```

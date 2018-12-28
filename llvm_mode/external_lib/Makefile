PREFIX  ?= ../../bin/
PROGS   = ${PREFIX}zlib-func.o
all: test_deps $(PROGS)

include ../Makefile.common

${PREFIX}zlib-func.o: zlib-func.c zlib_abilist.txt ../config.h | test_deps
	$(CC) $(CFLAGS) -fPIC -c $< -o $@
	cp zlib_abilist.txt ${PREFIX}.


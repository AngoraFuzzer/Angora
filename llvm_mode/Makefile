PREFIX     = ../bin/
PROGS      = ${PREFIX}angora-clang ${PREFIX}angora-llvm-pass.so ${PREFIX}unfold-branch-pass.so  ${PREFIX}io-func.o ${PREFIX}stdalloc.o ${PREFIX}pin_stub.o ${PREFIX}llvm-catch-dlclose.so

all: test_deps $(PROGS) rule_list ext_lib dfsan_lib

include Makefile.common

rule_list:
	cp ./custom/angora_abilist.txt ${PREFIX}.
	cp ./custom/exploitation_list.txt ${PREFIX}.

ext_lib:
	cd external_lib && make
	rm -rf ${PREFIX}/libcxx_dfsan
	mkdir ${PREFIX}/libcxx_dfsan
	cp libcxx_dfsan/libc++.a ${PREFIX}/libcxx_dfsan
	cp libcxx_dfsan/libc++abidfsan.a ${PREFIX}/libcxx_dfsan

dfsan_lib:
	cd dfsan && make

${PREFIX}angora-clang: angora-clang.c
	$(CC) $(CFLAGS) $< -o $@ $(LDFLAGS)
	ln -sf angora-clang ${PREFIX}angora-clang++

${PREFIX}angora-llvm-pass.so: angora-llvm-pass.so.cc abilist.h config.h
	$(CXX) $(CLANG_CFL) -shared $< -o $@ $(CLANG_LFL)

${PREFIX}unfold-branch-pass.so: unfold-branch-pass.so.cc config.h
	$(CXX) $(CLANG_CFL) -shared $< -o $@ $(CLANG_LFL)

${PREFIX}io-func.o: io-func.c config.h
	$(CC) $(CFLAGS) -fPIC -c $< -o $@

${PREFIX}stdalloc.o: stdalloc.c config.h
	$(CC) $(CFLAGS) -fPIC -c $< -o $@

${PREFIX}pin_stub.o: pin_stub.c
	$(CC) $(CFLAGS) -fPIC -c $< -o $@

${PREFIX}llvm-catch-dlclose.so: llvm-catch-dlclose.c
	$(CC) $(CFLAGS) -fPIC -shared $< -o $@

.NOTPARALLEL: clean

clean:
	rm -rf ${PREFIX}/*
	cd dfsan && make clean

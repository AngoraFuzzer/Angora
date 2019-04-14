PREFIX      ?= ../../bin/
CC         = clang
CXX        = clang++
AR         = ar
LLVM_CONFIG ?= llvm-config
CXXFLAGS    += -std=c++14 -Wall -fpic -O3 -g

CLANG_CFL    = `$(LLVM_CONFIG) --cxxflags` -funroll-loops -O3 -Wall -D_FORTIFY_SOURCE=2 -g -Wno-pointer-sign -Wno-variadic-macros -fno-rtti -fpic

CLANG_LFL    = `$(LLVM_CONFIG) --ldflags` $(LDFLAGS)

# User teor2345 reports that this is required to make things work on MacOS X.
ifeq "$(shell uname)" "Darwin"
  CLANG_LFL += -Wl,-flat_namespace -Wl,-undefined,suppress
endif

# TODO: fix rpc https://github.com/AngoraFuzzer/Angora/commit/b852891e86b57a9b7560a19f26857f55e5f2e907

#append_have_file_definition(rpc/xdr.h HAVE_RPC_XDR_H SANITIZER_COMMON_DEFINITIONS)
#append_have_file_definition(tirpc/rpc/xdr.h HAVE_TIRPC_RPC_XDR_H SANITIZER_COMMON_DEFINITIONS)


#SAN_SOURCES := $(wildcard ./sanitizer_common/*.cc)
SAN_SOURCES = ./sanitizer_common/sanitizer_allocator.cc ./sanitizer_common/sanitizer_common.cc ./sanitizer_common/sanitizer_deadlock_detector1.cc ./sanitizer_common/sanitizer_deadlock_detector2.cc ./sanitizer_common/sanitizer_flags.cc ./sanitizer_common/sanitizer_flag_parser.cc ./sanitizer_common/sanitizer_libc.cc ./sanitizer_common/sanitizer_libignore.cc ./sanitizer_common/sanitizer_linux.cc ./sanitizer_common/sanitizer_linux_s390.cc ./sanitizer_common/sanitizer_mac.cc ./sanitizer_common/sanitizer_persistent_allocator.cc ./sanitizer_common/sanitizer_platform_limits_linux.cc ./sanitizer_common/sanitizer_platform_limits_posix.cc ./sanitizer_common/sanitizer_posix.cc ./sanitizer_common/sanitizer_printf.cc ./sanitizer_common/sanitizer_procmaps_common.cc ./sanitizer_common/sanitizer_procmaps_freebsd.cc ./sanitizer_common/sanitizer_procmaps_linux.cc ./sanitizer_common/sanitizer_procmaps_mac.cc ./sanitizer_common/sanitizer_stackdepot.cc ./sanitizer_common/sanitizer_stacktrace.cc ./sanitizer_common/sanitizer_stacktrace_printer.cc ./sanitizer_common/sanitizer_suppressions.cc ./sanitizer_common/sanitizer_symbolizer.cc ./sanitizer_common/sanitizer_symbolizer_libbacktrace.cc ./sanitizer_common/sanitizer_symbolizer_mac.cc ./sanitizer_common/sanitizer_symbolizer_win.cc ./sanitizer_common/sanitizer_tls_get_addr.cc ./sanitizer_common/sanitizer_thread_registry.cc ./sanitizer_common/sanitizer_win.cc

# TERMINATION
SAN_SOURCES += ./sanitizer_common/sanitizer_termination.cc

#LIBC
SAN_SOURCES += ./sanitizer_common/sanitizer_common_libcdep.cc ./sanitizer_common/sanitizer_coverage_libcdep.cc ./sanitizer_common/sanitizer_coverage_mapping_libcdep.cc ./sanitizer_common/sanitizer_linux_libcdep.cc ./sanitizer_common/sanitizer_posix_libcdep.cc ./sanitizer_common/sanitizer_stacktrace_libcdep.cc ./sanitizer_common/sanitizer_stoptheworld_linux_libcdep.cc ./sanitizer_common/sanitizer_symbolizer_libcdep.cc ./sanitizer_common/sanitizer_symbolizer_posix_libcdep.cc ./sanitizer_common/sanitizer_unwind_linux_libcdep.cc

#NOLIBC
# SAN_SOURCES += sanitizer_common_nolibc.cc

SAN_OBJECTS := $(SAN_SOURCES:%.cc=%.o)

SAN_ASM_SOURCE = ./sanitizer_common/sanitizer_linux_x86_64.S
ifeq "$(shell uname)" "Darwin"
	SAN_ASM_SOURCE = ./sanitizer_common/sanitizer_linux_mips64.S
endif

SAN_ASM_OBJECT = $(SAN_ASM_SOURCE:%.S=%.o)

INT_SOURCES := $(wildcard ./interception/*.cc)
INT_OBJECTS := $(INT_SOURCES:%.cc=%.o)

RT_SOURCES := $(wildcard ./rt/*.cc)
RT_OBJECTS := $(RT_SOURCES:%.cc=%.o)

PROGS =  ${PREFIX}/DFSanPass.so ${PREFIX}/DFSanRT.a

all: ${PROGS} abilist

${PREFIX}/DFSanRT.a: ${RT_OBJECTS} ${SAN_OBJECTS} ${INT_OBJECTS} ${SAN_ASM_OBJECT}
	$(AR) r $@ ${RT_OBJECTS} ${SAN_OBJECTS} ${INT_OBJECTS} ${SAN_ASM_OBJECT} #../../tag_set/tag_set.o

./sanitizer_common/%.o : ./sanitizer_common/%.cc
	$(CXX) -c $(CXXFLAGS) $< -o $@

./interception/%.o : ./interception/%.cc
	$(CXX) -c $(CXXFLAGS) $< -o $@

./rt/%.o : ./rt/%.cc
	$(CXX) -c $(CXXFLAGS) $< -o $@

${SAN_ASM_OBJECT} : ${SAN_ASM_SOURCE}
	$(CXX) -c $(CXXFLAGS) $< -o $@

${PREFIX}/DFSanPass.so: ./pass/DFSanPass.cc
	$(CXX) $(CLANG_CFL) -shared $< -o $@ $(CLANG_LFL)

abilist: ./rt/done_abilist.txt ./rt/libc_ubuntu1404_abilist.txt libclang_rt.dfsan-x86_64.a.syms
	cat ./rt/done_abilist.txt ./rt/libc_ubuntu1404_abilist.txt > ${PREFIX}/dfsan_abilist.txt
	cp libclang_rt.dfsan-x86_64.a.syms  ${PREFIX}/DFSanRT.a.syms

clean:
	@rm -f ${RT_OBJECTS} ${SAN_OBJECTS} ${INT_OBJECTS} ${SAN_ASM_OBJECT} ${PROGS} 

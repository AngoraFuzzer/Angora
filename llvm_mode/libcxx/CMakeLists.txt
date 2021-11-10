# run compile.sh

install (FILES "build_fast/lib/libc++.a" DESTINATION "${ANGORA_LIB_DIR}/libcxx_fast" RENAME "libc++fast.a")
install (FILES "build_fast/lib/libc++abi.a" DESTINATION "${ANGORA_LIB_DIR}/libcxx_fast" RENAME "libc++abifast.a")
install (FILES "build_track/lib/libc++.a" DESTINATION "${ANGORA_LIB_DIR}/libcxx_track" RENAME "libc++track.a")
install (FILES "build_track/lib/libc++abi.a" DESTINATION "${ANGORA_LIB_DIR}/libcxx_track" RENAME "libc++abitrack.a")

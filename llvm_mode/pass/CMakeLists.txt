
find_package(LLVM REQUIRED CONFIG)

if (LLVM_FOUND)
    message(STATUS "LLVM_VERSION_MAJOR: ${LLVM_VERSION_MAJOR}")
    message(STATUS "LLVM_VERSION_MINOR: ${LLVM_VERSION_MINOR}")
    message(STATUS "LLVM_VERSION_PATCH: ${LLVM_VERSION_PATCH}")
    if (${LLVM_VERSION_MAJOR} GREATER 7 OR ${LLVM_VERSION_MAJOR} LESS 4)
        message(FATAL_ERROR "LLVM version should be 4.0.0 - 7.1.0 !")
    endif()
else()
    message(FATAL_ERROR "You haven't install LLVM !")
endif()

if (NOT TARGET LLVMPassConfig)

    EXECUTE_PROCESS(
        COMMAND cat ${CMAKE_CURRENT_SOURCE_DIR}/../../common/src/config.rs          
        COMMAND grep "pub const MAP_SIZE_POW2"
        COMMAND cut -f2 -d "="  
        COMMAND grep -o "[[:digit:]]*"
        COMMAND tr -d "\n" 
        OUTPUT_VARIABLE MAP_SIZE_POW2
    )
    message(STATUS "MAP_SIZE_POW2: ${MAP_SIZE_POW2}")
    
    add_library(LLVMPassConfig INTERFACE IMPORTED)
    # fix pass bug: https://github.com/sampsyo/llvm-pass-skeleton/issues/7#issuecomment-401834287
    set_target_properties(LLVMPassConfig PROPERTIES
        INTERFACE_COMPILE_OPTIONS "-fno-rtti" #-fpic
        INTERFACE_INCLUDE_DIRECTORIES "${LLVM_INCLUDE_DIRS}"
        INTERFACE_LINK_DIRECTORIES "${LLVM_LIBRARY_DIRS}"
        INTERFACE_LINK_OPTIONS "-Wl,-znodelete"
        INTERFACE_COMPILE_DEFINITIONS "LLVM_VERSION_MAJOR=${LLVM_VERSION_MAJOR};LLVM_VERSION_MINOR=${LLVM_VERSION_MINOR};MAP_SIZE_POW2=${MAP_SIZE_POW2}"
        # INTERFACE_COMPILE_FEATURES "cxx_std_11"
    )
    if(APPLE)
        set_target_properties(LLVMPassConfig PROPERTIES
            # User teor2345 reports that this is required to make things work on MacOS X.
            LINK_FLAGS "-Wl,-flat_namespace -Wl,-undefined,suppress"
    )
    endif(APPLE)
endif()

set(CMAKE_CXX_STANDARD 11)

add_library(UnfoldBranchPass MODULE UnfoldBranchPass.cc)
target_link_libraries(UnfoldBranchPass LLVMPassConfig)
install (TARGETS UnfoldBranchPass DESTINATION ${ANGORA_PASS_DIR})

add_library(AngoraPass MODULE AngoraPass.cc)
target_link_libraries(AngoraPass LLVMPassConfig)
install (TARGETS AngoraPass DESTINATION ${ANGORA_PASS_DIR})

add_library(DFSanPass MODULE DFSanPass.cc)
target_link_libraries(DFSanPass LLVMPassConfig)
install (TARGETS DFSanPass DESTINATION ${ANGORA_PASS_DIR})


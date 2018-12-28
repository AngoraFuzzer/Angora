# Evaluate coverage
- tool: [afl-cov](https://github.com/mrash/afl-cov)

# Install gcov and genhtml
```
sudo apt-get install lcov

```

# build
- make
```
CC=gcc CFLAGS="-fprofile-arcs -ftest-coverage -g -O0" LFLAGS="-lgcov --coverage" make
```

- autoconf
```
CFLAGS="-fprofile-arcs -ftest-coverage -g -O0" LIBS=-lgcov ../src/configure --prefix=`pwd`/install --disable-shared
```

- cmake
```
cmake -DENABLE_GCOV -DBUILD_SHARED_LIBS=OFF -DCMAKE_BUILD_TYPE=Debug ../src

option(ENABLE_GCOV "Enable gcov." Off)
if (ENABLE_GCOV)
   SET(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} -fprofile-arcs -ftest-coverage")
   SET(CMAKE_C_FLAGS_DEBUG "${CMAKE_C_FLAGS_DEBUG} -fprofile-arcs -ftest-coverage")
   SET(CMAKE_EXE_LINKER_FLAGS_DEBUG "${CMAKE_EXE_LINKER_FLAGS_DEBUG} -fprofile-arcs -ftest-coverage -lgcov")
   endif()
```

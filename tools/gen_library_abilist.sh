#!/bin/sh

if [ ! "$#" = "2" ]; then

    cat 1>&2 <<_EOF_
Usage
- Discard taints
$ ./gen_library_abilist.sh path-to-library.so > xxlib_abilist.txt discard
- Return value is the union of the label of its arguments.
$ ./gen_library_abilist.sh path-to-library.so > xxlib_abilist.txt functional
- Define a custom wrapper by yourself
$ ./gen_library_abilist.sh path-to-library.so > xxlib_abilist.txt custom
visit https://clang.llvm.org/docs/DataFlowSanitizer.html to see more.
_EOF_

    exit 1

fi

NM=`which nm 2>/dev/null`

if [ "$NM" = "" ]; then
    echo "[-] Error: can't find 'nm' in your \$PATH. please install binutils" 1>&2
    exit 1
fi

echo "# $1" | grep 'so[.0-9]*$'
if [ $? -eq 0 ]
then
    # echo "dynamic library.."
    nm -D --defined-only $1 | grep " T " | sed 's/^[0-9a-z]\+ T /fun:/g; s/$/=uninstrumented/g'
    nm -D --defined-only $1 | grep " T " | sed "s/^[0-9a-z]\+ T /fun:/g; s/$/=$2/g"
else
    # echo "static library.."
    nm --defined-only $1 | grep " T " | sed 's/^[0-9a-z]\+ T /fun:/g; s/$/=uninstrumented/g'
    nm --defined-only $1 | grep " T " | sed "s/^[0-9a-z]\+ T /fun:/g; s/$/=$2/g"
fi

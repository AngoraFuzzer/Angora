# LAVA-M `who` Dataset Performance Analysis

This fix log explains the numerous improvements that went into Angora and their
purposes in improving the performance on the LAVA-M dataset.

## Taint Propagation in Allocation Functions (Solved)

The `who` program calls `x2nrealloc()` while parsing the input `utmp` structs.
`x2nrealloc()` in turn calls the standard C library function `realloc()`, thus
requiring our taint propagation rules to conform to the semantics of `*alloc()`
functions. Since `realloc()` may change the base address of the heap chunk, an
efficient strategy would be to determine whether the base has changed and copy 
the corresponding labels to the destination address if required.

This strategy would require that our runtime libraries hook to standard library
allocation functions to correctly propagate the taint labels. Since we should 
preserve the relationships between each byte and their corresponding label, we
would have to save the allocation size of the heap chunk. An easy solution 
would be to record the base and bound of a chunk within the chunk itself. 
However, some `libc` functions allocate heap memory internally and statically
link to `*alloc()` functions, rendering this method impractical.

### Solution: Saving allocation metadata in a HashMap

The solution to this problem is to utilize a HashMap to record the 
base and bound information for allcoations. This only requires modifications to
the `__dfsw_*alloc()` functions. The HashMap insertion, deletion and querying 
operations are implemented in Rust. The HashMap uses base pointer values as the
key and bound values as the value. Minimal instrumentation is required for this 
approach. The source code can be found in the repository under the `llvm_mode`
directory. 

## Path Coverage (Unsolved)

There are also implicit dependencies within `who`. A simplified version would 
be:

```
time_t boottime = TYPE_MINIMUM (time_t);
...
while (n--)
  {
    if (utmp_buf->type == 7) {
        lava_1234 = boottime...
    }

    ...
    
    if (utmp_buf->type == 2) {
        boottime = utmp_buf->time...
    }
    utmp_buf++;
  }
```

The code requires that a previous element in the input array should have its 
`type` member set to 2 while the current element should have its `type` member
set to 7 so that input values are given to boottime, in turn allowing `lava_*` 
variables to receive the value. 

Solving this type of bugs would require recording path coverage, a problem
neither AFL nor Angora aims to solve.

## Synthesized Integer from String Bytes (Unsolved)

A few unsolved cases have their test values assembled from bytes taken from a 
string, like the example below:

```
int lava_1234 = 0;
char * host = ... ; // Get a string from input
lava_1234 |= ((uchar *) (host))[0] << (0*8);
lava_1234 |= ((uchar *) (host))[1] << (1*8);
...
```

The shape of these variables cannot be inferred reliably, due to a number of 
issues involving the accuracy and granularity of taint tracking. A simple 
solution would be to implement a strategy similar to that of REDQUEEN, but it
would be meaningful only to such cases.


# Troubleshoot

## Target program compilation errors 

- `dfs$***` undefined errors in compiling: See *Model external library* section.

- Can't find xlocal.h while compiling C++ with Angora.

```
ln -s /usr/include/locale.h /usr/include/xlocale.h
```

- `*scanf()` functions not modelled: Replace `*scanf()` functions.

## Runtime errors

- Failed to find any branches during dry run: Ensure the binary is instrumented and the input 
directory is populated. Otherwise no branches can be found.


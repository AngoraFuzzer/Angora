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

- Multiple inconsistent warnings. It caused by the fast and track programs has different behaviors. If most constraints are inconsistent, ensure they are compiled with the same environment. Otherwise, report us.

- Density is too large (> 10%). Please increase `MAP_SIZE_POW2` in `llvm_mode/config.h` and `MAP_LENGTH` in `common/src/config.rs`. Or disable function-call context by compiling with `ANGORA_DISABLE_CONTEXT=1` or `ANGORA_DIRECT_FN_CONTEXT=1` environment variable.

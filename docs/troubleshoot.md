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

- Density is too large (> 10%). Please increase `MAP_SIZE_POW2` in `common/src/config.rs`. Or disable function-call context(density > 50%) by compiling with `ANGORA_CUSTOM_FN_CONTEXT=k` (k is an integer and 0 <= k <= 32) environment variable. Angora disables context if k is 0.

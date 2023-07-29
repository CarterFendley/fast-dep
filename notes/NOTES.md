## Linker error 

**TODO:** Will probably need to enable this again for many_linux distribution. 

The `pyo3/extension-module` "feature" is apparently used to prevent linking against libpython on linux systems [documentation](https://pyo3.rs/v0.19.1/building_and_distribution#the-extension-module-feature). This can be added to both the `pyproject.toml` and `Cargo.toml`. As stated in the documentation it will cause tests and binaries to fail to build. See `linker_error.txt` in this directory for an example of this happing.

This seems like rust can find the headers but not the symbols in whatever package it finds. Interestingly with `conda` disabling this would cause another runtime issue complaining about not being able to find the system `libpython3.7m.dylib`. Sort of similar to the issue [here](https://github.com/PyO3/pyo3/issues/1800).

I have adopted `PYTHON_CONFIGURE_OPTS="--enable-shared" pyenv install X.Y.Z` as mentioned in that thread. This works fine with `extension-module` disabled. It has the same error as conda with the extension module enabled. This does confuse me as the `lib_dir` listed by `PYO3_PRINT_CONFIG=1 maturin build -b "bin"` does not point to a system directory in either `conda` or `pyenv` so not sure why it has a runtime error relating to the system libpython with `conda` where `pyenv` does not
#### Comparison of extension-module changes

In the below I don't see any difference in the `PYO3_ENVIRONMENT_SIGNATURE`, `PYO3_PYTHON`, and `PYTHON_SYS_EXECUTABLE` variables. This makes me wonder where the system libray changes is happening?

```bash
MACOSX_DEPLOYMENT_TARGET="10.7" PYO3_ENVIRONMENT_SIGNATURE="cpython-3.7-64bit" PYO3_PYTHON="/Users/carter/opt/anaconda3/envs/fast-dep/bin/python3" PYTHON_SYS_EXECUTABLE="/Users/carter/opt/anaconda3/envs/fast-dep/bin/python3" "cargo" "rustc" "--message-format" "json-render-diagnostics" "--manifest-path" "/Users/carter/src/fast-pydep/Cargo.toml" "--bin" "fast-dep"`
```

With `--features "extension-module"`

```bash
MACOSX_DEPLOYMENT_TARGET="10.7" PYO3_ENVIRONMENT_SIGNATURE="cpython-3.7-64bit" PYO3_PYTHON="/Users/carter/opt/anaconda3/envs/fast-dep/bin/python3" PYTHON_SYS_EXECUTABLE="/Users/carter/opt/anaconda3/envs/fast-dep/bin/python3" "cargo" "rustc" "--features" "extension-module" "--message-format" "json-render-diagnostics" "--manifest-path" "/Users/carter/src/fast-pydep/Cargo.toml" "--bin" "fast-dep"
```
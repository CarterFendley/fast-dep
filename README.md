# fast-dep

```
brew install pyenv
pyenv install 3.7.17
```

Maybe:

```
eval "$(pyenv init -)"
```

Build binary and install:

```
maturin build -b "bin" && pip install --force-reinstall target/wheels/*.whl
```

Build and install the package: 

```
maturin build && pip install --force-reinstall target/wheels/*.whl
```
# fast-dep

Fast python dependency tracing, from Rust.

```
pip install fast-dep
```

**NOTE:** There is currently some issue with running PyO3 code in MacOS conda environments, see more details [here]. We currently recommend using `pyenv` installs which work on that OS. See instruction below.

## Development

#### Building from source

Install rust from [rustup.rs](https://rustup.rs/) then run the following to build the python package for your current environment.

```
pip install -e ".[test]"
```

You can also build without installing by using the following method.

```
pip install maturin
maturin build
```

#### Running tests

```
python -m pytest pytests
```

## Installing PyEnv (For MacOS)

Install `pyenv`.

```
brew install pyenv
```

Place the following in your `.zshrc` or `.profile`.

```
eval "$(pyenv init -)"
```

Install a specific version of Python.

```
PYTHON_CONFIGURE_OPTS="--enable-shared" pyenv install X.Y.Z
```

Get a shell with this previously installed Python version.

```
pyenv shell X.Y.Z
```

Then install fast dep.

```
pip install fast-dep
```
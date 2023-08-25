# fast-dep

```
pip install fast-dep
```

**NOTE:** There is currently some issue with running PyO3 code in MacOS conda environments. We currently recommend using `pyenv` installs which work on that OS.


## Installing PyEnv (For MacOS)

```
brew install pyenv
```

Place the following in your `.zshrc` or `.profile`.

```
eval "$(pyenv init -)"
```

Install a specific version of Python and get a shell with that interpreter.

```
pyenv install <3.X>
pyenv shell <3.X>
```

Then install fast dep.

```
pip install fast-dep
```

## Building from source

Install rust from [rustup.rs](https://rustup.rs/) then run the following to build the python package for your current environment.

```
pip install -e .
```
References:
- https://docs.python.org/3.9/library/ast.html
- https://www.cs.princeton.edu/~appel/papers/asdl97.pdf

TODO: 
- Only need attributes for *sum_type*'s such as "arg"
- All of the boxes cause any performance hit? Any way around this?
- Change some of the `i32`s to `bool`s

Notable differences
- Some names in the python syntax distinguished based on case such as: `BoolOp` and `boolop`. To keep everything *rusty* these have been changed to `BoolOpExpr` and `BoolOp` where the `[...]Expr` suffix is applied to all expressions regardless of the name conflict.
- `ExceptionHandler` has `htype` because `type` is a keyword

## Notes on Enums

Looks like there is not currently Enum support of `FromPyObject` as noted [here](https://stackoverflow.com/questions/67412827/pyo3-deriving-frompyobject-for-enums). There is a PR which implements `#[pyclass]` support for Enums but this only supports the "Unit" variants. See [here](https://github.com/PyO3/pyo3/pull/2002/commits/b7419b5278e18ac9b99680ecb12fc109ddd56320)
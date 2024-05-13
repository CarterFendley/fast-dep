// // Reference: https://docs.python.org/3.9/library/ast.html
// // - https://www.cs.princeton.edu/~appel/papers/asdl97.pdf

pub struct Attributes {
    lineno: i32,
    col_offset: i32,
    end_lineno: Option<i32>,
    end_col_offset: Option<i32>
}

// --------------

pub enum Mod {
    Module {
        body: Vec<Stmt>,
        type_ignores: Vec<TypeIgnore>
    },
    Interactive {
        body: Vec<Stmt>
    },
    Expression {
        body: Expr,
    },
    FunctionType {
        argtypes: Vec<Expr>,
        returns: Expr
    }
}

// --------------

// ---- #[derive(FromPyObject)]
pub struct Stmt {
    // TODO: Not 100% sure why this Box is needed
    data: Box<StmtData>,
    attr: Attributes
}

// ---- #[derive(FromPyObject)]
pub enum StmtData {
    FunctionDef {
        name: String,
        args: Arguments,
        body: Vec<Stmt>,
        decorator_list: Vec<Expr>,
        returns: Option<Expr>,
        type_comment: String,
    },
    AsyncFunctionDef {
        name: String,
        args: Arguments,
        body: Vec<Stmt>,
        decorator_list: Vec<Expr>,
        returns: Option<Expr>,
        type_comment: String,
    },
    ClassDef {
        name: String,
        bases: Vec<Expr>,
        keywords: Vec<Keyword>,
        body: Vec<Stmt>,
        decorator_list: Vec<Expr>
    },
    Return {
        value: Option<Expr>
    },
    Delete {
        targets: Vec<Expr>,
    },
    Assign {
        targets: Vec<Expr>,
        value: Expr,
        type_comment: Option<String>
    },
    AugAssign {
        target: Expr,
        op: Operator,
        value: Expr
    },
    // 'simple' indicates that we annotate simple name without parens
    AnnAssign {
        target: Expr,
        annotation: Expr,
        value: Option<Expr>,
        simple: i32
    },
    // use 'orelse' because else is a keyword in target languages
    For {
        target: Expr,
        iter: Expr,
        body: Vec<Stmt>,
        orelse: Vec<Stmt>,
        type_comment: Option<String>
    },
    AsyncFor {
        target: Expr,
        iter: Expr,
        body: Vec<Stmt>,
        orelse: Vec<Stmt>,
        type_comment: Option<String>
    },
    While {
        test: Expr,
        body: Vec<Stmt>,
        orelse: Vec<Stmt>
    },
    If {
        test: Expr,
        body: Vec<Stmt>,
        orelse: Vec<Stmt>
    },
    With {
        test: Expr,
        body: Vec<Stmt>,
        type_comment: Option<String>
    },
    AsyncWith {
        test: Expr,
        body: Vec<Stmt>,
        type_comment: Option<String>
    },
    Raise {
        exc: Option<Expr>,
        cause: Option<Expr>
    },
    Try {
        body: Vec<Stmt>,
        handlers: Vec<ExceptionHandler>,
        orelse: Vec<Stmt>,
        finalbody: Vec<Stmt>
    },
    Assert {
        test: Expr,
        msg: Option<Expr>
    },
    Import {
        names: Vec<Alias>
    },
    ImportFrom {
        module: Option<String>,
        names: Vec<Alias>,
        level: Option<i32>
    },
    Global {
        names: Vec<String>,
    },
    Nonlocal {
        names: Vec<String>
    },
    Expr {
        value: Expr
    },
    Pass,
    Break,
    Continue
}

// --------------
pub struct Expr {
    // TODO: Not 100% sure why this Box is needed
    data: Box<ExprData>,
    attr: Attributes
}

pub enum ExprData {
    BoolOpExpr {
        boolop: BoolOp,
        values: Vec<Expr>
    },
}
    NamedExpr {
        target: Box<Expr>,
        value: Box<Expr>
    },
    BinOpExpr {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
    UnaryOpExpr {
        op: UnaryOp,
        operand: Box<Expr>
    },
    LambdaExpr {
        args: Args,
        body: Box<Expr>
    },
    IfExpr {
        test: Box<Expr>,
        body: Box<Expr>,
        orelse: Box<Expr>
    },
    DictExpr {
        keys: Vec<Expr>,
        values: Vec<Expr>
    },
    SetExpr {
        elts: Vec<Expr>
    },
    ListCompExpr {
        elt: Box<Expr>,
        generators: Vec<Comprehension>
    },
    SetCompExpr {
        elt: Box<Expr>,
        generators: Vec<Comprehension>
    },
    DictCompExpr {
        key: Box<Expr>,
        value: Box<Expr>,
        generators: Vec<Comprehension>
    },
    GeneratorExpr {
        elt: Box<Expr>,
        generators: Vec<Comprehension>
    },
    // the grammar constrains where yield expressions can occur
    AwaitExpr {
        value: Box<Expr>
    },
    YieldExpr {
        value: Option<Expr>
    },
    YieldFromExpr {
        value: Box<Expr>
    },
    // need sequences for compare to distinguish between
    // x < 4 < 3 and (x < 4) < 3
    CompareExpr {
        left: Box<Expr>,
        ops: Vec<CmpOp>,
        comparators: Vec<Expr>
    },
    CallExpr {
        func: Box<Expr>,
        args: Vec<Expr>,
        keywords: Vec<Keyword>
    },
    FormattedValueExpr {
        value: Box<Expr>,
        conversion: i32,
        format_spec: Box<Expr>
    },
    JoinedStrExpr {
        values: Vec<Expr>
    },
    ConstantExpr {
        value: String,
        kind: Option<String>
    },
    // the following expression can appear in assignment context
    AttributeExpr {
        value: Box<Expr>,
        attr: String,
        ctx: ExprContext
    },
    SubscriptExpr {
        value: Box<Expr>,
        slice: Box<Expr>,
        ctx: ExprContext
    },
    StarredExpr {
        value: Box<Expr>,
        ctx: ExprContext
    },
    NameExpr {
        id: String,
        ctx: ExprContext
    },
    ListExpr {
        elts: Vec<Expr>,
        ctx: ExprContext,
    },
    TupleExpr {
        elts: Vec<Expr>,
        ctx: ExprContext
    },
    // can appear only in Subscript
    SliceExpr {
        lower: Option<Expr>,
        upper: Option<Expr>,
        step: Option<Expr>
    }
}   

// -------------------

// ---- #[derive(FromPyObject)]
pub enum ExprContext {
    Load,
    Store,
    Del
}

pub enum BoolOp {
    And,
    Or
}

// ---- #[derive(FromPyObject)]
pub enum Operator {
    Add,
    Sub,
    Mult,
    MatMult,
    Div,
    Mod,
    Pow,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
    FloorDiv
}

// ---- #[derive(FromPyObject)]
pub enum UnaryOp {
    Invert,
    Not,
    UAdd,
    USub
}

// ---- #[derive(FromPyObject)]
pub enum CmpOp {
    Eq,
    NotEq,
    Lt,
    LtE,
    Gt,
    GtE,
    Is,
    IsNot,
    In,
    NotIn
}

// ---- #[derive(FromPyObject)]
pub struct Comprehension {
    target: Expr,
    iter: Expr,
    ifs: Vec<Expr>,
    is_async: i32,
}

// ---- #[derive(FromPyObject)]
pub struct ExceptionHandler {
    htype: Expr,
    name: String,
    body: Vec<Stmt>,
    attr: Attributes
}

// ---- #[derive(FromPyObject)]
pub struct Arguments {
    posonlyargs: Vec<Args>,
    args: Vec<Args>,
    vararg: Option<Args>,
    kwonlyargs: Vec<Args>,
    kw_defaults: Vec<Expr>,
    kwarg: Option<Args>,
    defaults: Vec<Expr>
}

// ---- #[derive(FromPyObject)]
pub struct Args {
    arg: String,
    annotation: Option<Expr>,
    type_comment: Option<String>,
    attr: Attributes
}

// keyword arguments supplied to call (NULL identifier for **kwargs)
// ---- #[derive(FromPyObject)]
pub struct Keyword {
    arg: Option<String>,
    value: Expr,
    annotation: Option<Expr>
}

// import name with optional 'as' alias.
// ---- #[derive(FromPyObject)]
pub struct Alias {
    name: String,
    asname: Option<String>
}

// ---- #[derive(FromPyObject)]
pub struct WithItem {
    context_expr: Expr,
    optional_vars: Option<Expr>
}

// ---- // ---- #[derive(FromPyObject)]
pub struct TypeIgnore {
    lineno: i32,
    tag: String
}

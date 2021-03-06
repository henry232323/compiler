use std::fmt::{Debug, Error, Formatter};

#[derive(Clone)]
pub enum Expr {
    Boolean(bool),
    Integer(i32),
    Float(f32),
    BinaryOp(Box<Expr>, Opcode, Box<Expr>),
    UnaryOp(Opcode, Box<Expr>),
    Error,
    Str(String),
    List(Vec<Box<Expr>>),
    Variable(String),
    FunctionCall(Box<Expr>, Vec<Box<Expr>>),
    ItemSubscription(Box<Expr>, Box<Expr>),
    AttrAccess(Box<Expr>, String),
    Function(String, Box<Vec<String>>, Vec<Box<Statement>>),
    None,
}

#[derive(Clone)]
pub enum Statement {
    Expression(Box<Expr>),
    If(Box<Expr>, Vec<Box<Statement>>, Option<Vec<Box<Statement>>>),
    Function(String, Box<Vec<String>>, Vec<Box<Statement>>),
    Assignment(String, Box<Expr>),
    Return(Box<Expr>),
    While(Box<Expr>, Vec<Box<Statement>>),
    For(String, Box<Expr>, Vec<Box<Statement>>),
    Import(Vec<Box<Name>>),
    Try(Vec<Box<Statement>>, Vec<Box<Catch>>),
}

#[derive(Clone)]
pub enum Catch {
    Catch(Option<String>, Option<Box<Expr>>, Vec<Box<Statement>>)
}

#[derive(Clone)]
pub enum Name {
    Name(String),
    Alias(String, String),
} 

#[derive(Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
    Pow,
}

pub struct Module {
    pub body: Vec<Box<Statement>>,
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match &*self {
            Integer(n) => write!(fmt, "Integer({:#?})", n),
            Float(n) => write!(fmt, "Float({:#?})", n),
            Boolean(b) => write!(fmt, "Boolean({:#?})", b),
            BinaryOp(ref l, op, ref r) => write!(fmt, "BinaryOp({:#?}, {:#?}, {:#?})", l, op, r),
            UnaryOp(op, ref r) => write!(fmt, "UnaryOp({:#?}, {:#?})", op, r),
            Str(s) => write!(fmt, "Str({:#?})", s),
            List(l) => write!(fmt, "List({:#?})", l),
            Variable(v) => write!(fmt, "Variable({:#})", v),
            FunctionCall(ref fun, args) => {
                write!(fmt, "FunctionCall({:#?}, args={:#?})", fun, args)
            }
            ItemSubscription(ref expr, ref index) => {
                write!(fmt, "ItemSubscription({:#?}, index={:#?})", expr, index)
            }
            AttrAccess(ref expr, attr) => write!(fmt, "AttrAccess({:#?}, attr={:#?})", expr, attr),
            Error => write!(fmt, "Error"),
            Function(name, args, body) => write!(
                fmt,
                "Function({:#?}, args={:#?}, body={:#?}, )",
                name, args, body
            ),
            None => write!(fmt, "None")
        }
    }
}

impl Debug for Statement {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Statement::*;
        match &*self {
            Expression(ref expr) => write!(fmt, "Expression({:#?})", expr),
            If(ref cond, body, elsebody) => write!(
                fmt,
                "If(cond=({:#?}), body={:#?}, elsebody={:#?})",
                cond, body, elsebody
            ),
            Function(ref name, args, body) => write!(
                fmt,
                "Function(name=({:#?}), args={:#?}, body={:#?})",
                name, args, body
            ),
            Assignment(ref name, value) => {
                write!(fmt, "Assignment(name=({:#?}), value={:#?})", name, value)
            }
            Return(ref expr) => write!(fmt, "Return({:#?})", expr),
            While(ref cond, body) => write!(fmt, "While(cond=({:#?}), body={:#?})", cond, body),
            For(ref name, value, body) => write!(
                fmt,
                "For(name=({:#?}), value={:#?}, body={:#?})",
                name, value, body
            ),

            Import(ref imports) => write!(
                fmt,
                "Import(imports=({:#?}))",
                imports
            ),
            Try(stmt, handlers) => write!(fmt, "Try({:#?}, {:#?})", stmt, handlers),
        }
    }
}

impl Debug for Catch {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Catch::*;
        match &*self {
            Catch(name, expr, stmts) => write!(fmt, "Catch({:#?}, {:#?}, {:#?})", name, expr, stmts),
        }
    }
}

impl Debug for Name {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Name::*;
        match &*self {
            Name(name) => write!(fmt, "Name({:#?})", name),
            Alias(name, alias) => write!(fmt, "Alias({:#?}, {:#?})", name, alias),
        } 
    }
}

impl Debug for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Opcode::*;
        match *self {
            Mul => write!(fmt, "Op::Mul"),
            Div => write!(fmt, "Op::Div"),
            Add => write!(fmt, "Op::Add"),
            Sub => write!(fmt, "Op::Sub"),
            Pow => write!(fmt, "Op::Pow"),
        }
    }
}




use core::libc::c_void;
use core::hashmap::linear;

use ast;
use interp::env;

pub struct Fun {
    pattern: @[@ast::Pat],
    body: @ast::Exp
}

pub enum Val<Interp> {
    // (),
    Unit,

    // a
    Symbol(@ast::Sym),

    // 1
    Integer(int),

    // 1.1
    Floating(float),

    // 'hello!'
    Bytes(@[char]),

    // "hello!"
    String(~str),

    // A(a, b, c, ...)
    Record(@ast::RecordDeclaration, @[@Val<Interp>]),

    // f(a, b, ...) = ...
    Function(@[@Fun]),

    // a.{...}
    Module(@mut linear::LinearMap<ast::Sym, @Val<Interp>>),

    // <routine>
    Routine(@fn (&Interp, &env::Env<Interp>) -> Val<Interp>),

    // <handle>
    Handle(c_void),
}

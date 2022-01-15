pub mod expression;
pub mod statement;
pub mod value;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum IdentifierKind {
    Constant,
    Variable,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identifier {
    pub ident: String,
    pub kind: IdentifierKind,
}

impl Identifier {
    pub fn new_const(ident: String) -> Self {
        Self {
            ident,
            kind: IdentifierKind::Constant,
        }
    }

    pub fn new_var(ident: String) -> Self {
        Self {
            ident,
            kind: IdentifierKind::Constant,
        }
    }

    pub fn is_const(&self) -> bool {
        self.kind == IdentifierKind::Constant
    }

    pub fn is_var(&self) -> bool {
        self.kind == IdentifierKind::Variable
    }
}

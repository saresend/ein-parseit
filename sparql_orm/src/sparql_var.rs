//!
//! WIP, ideal end state is to have
//!
//!
//! A marker trait that indicates that a type is a valid
//! variable, that is usable in a triple or any other
//! place where a valid variable or binding can be used

use crate::query_build::QueryFragment;

pub trait SPQLVar {}

use crate::identifier::*;

pub struct Literal<T: Identifier> {
    pub(crate) v: T,
}
pub struct Variable<T: Identifier> {
    pub(crate) v: T,
}

impl<T> SPQLVar for Literal<T> where T: Identifier {}
impl<T> SPQLVar for Variable<T> where T: Identifier {}

use crate::query_build::QueryBuilder;

impl<T> QueryFragment for Literal<T>
where
    T: Identifier + QueryFragment,
{
    fn generate_fragment(&self, builder: &mut QueryBuilder) {
        self.v.generate_fragment(builder);
    }
}

impl<T> QueryFragment for Variable<T>
where
    T: Identifier + QueryFragment,
{
    fn generate_fragment(&self, builder: &mut QueryBuilder) {
        builder.write_element("?");
        self.v.generate_fragment(builder);
    }
}

#[cfg(test)]
mod spql_var_tests {
    use crate::identifier::Ident;
    use crate::query_build::gen_fragment;
    use crate::sparql_var::{Literal, Variable};

    #[test]
    fn test_literal_render() {
        let lit = Literal {
            v: Ident(String::from("foo")),
        };
        let result = gen_fragment(lit);
        assert_eq!(result, "foo");
    }

    #[test]
    fn test_var_render() {
        let lit = Variable {
            v: Ident(String::from("foo")),
        };
        let result = gen_fragment(lit);
        assert_eq!(result, "?foo");
    }
}

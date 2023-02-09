//!
//! This library is a diesel inspired flavor, of a possible ORM for
//! sparql based queries. It implements a subset
//! of the w3c specification for both query and update calls
//!
#![feature(generic_const_exprs)]
pub mod delete_data_clause;
pub mod delete_statement;
mod graph_specifier;
mod identifier;
pub mod insert_data_clause;
pub mod insert_statement;
mod predicates;
pub mod prefix;
pub mod query_build;
mod sparql_var;
pub mod triple_pattern;
mod update_types;
mod where_clause;

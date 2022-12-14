//!
//! This library is a diesel inspired flavor, of a possible ORM for
//! sparql based queries. It implements a subset
//! of the w3c specification for both query and update calls

pub mod delete_data_clause;
pub mod delete_statement;
pub mod graph_specifier;
pub mod identifier;
pub mod insert_data_clause;
pub mod insert_statement;
pub mod predicates;
pub mod query_build;
pub mod sparql_var;
pub mod triple_pattern;
pub mod update_types;
pub mod where_clause;

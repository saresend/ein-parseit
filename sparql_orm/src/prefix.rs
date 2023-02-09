//!
//!
//! A module that contains logic for adding URI prefixes
//! to different statements
//!
use crate::query_build::QueryFragment;

/// Marker trait for any type that can render as a
/// prefix in a SPQARQL query
pub trait SPQLPrefixTrait: QueryFragment {}

/// A type denoting the empty set of prefixes
pub struct NullPrefixSet {}

impl SPQLPrefixTrait for NullPrefixSet {}

impl QueryFragment for NullPrefixSet {
    fn generate_fragment(&self, builder: &mut crate::query_build::QueryBuilder) {
        // no-op
    }
}

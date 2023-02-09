//!
//!
//! A module that contains logic for adding URI prefixes
//! to different statements
//!
use crate::query_build::QueryFragment;
use http::uri;

/// Marker trait for any type that can render as a
/// prefix in a SPQARQL query
pub trait SPQLPrefixTrait: QueryFragment {}

/// A type denoting the empty set of prefixes
pub struct NullPrefixSet {}


pub struct Prefix {
    name: String,
    uri: uri::Uri,
}

impl SPQLPrefixTrait for NullPrefixSet {}

impl QueryFragment for NullPrefixSet {
    fn generate_fragment(&self, builder: &mut crate::query_build::QueryBuilder) {
        // no-op
    }
}

impl QueryFragment for Prefix {
   fn generate_fragment(&self, builder: &mut crate::query_build::QueryBuilder) {
       let uri_str = self.uri.to_string();
       builder.write_element("PREFIX ");
       builder.write_element(&std::format!("{}: ", &self.name));
       builder.write_element(&std::format!("<{}>", &uri_str));
   } 
}


#[cfg(test)]
mod tests { 
    use super::*; 
    use crate::query_build::gen_fragment;
    #[test]
    fn test_basic_prefix_generation() {

        let test_prefix = Prefix { 
            name: "dc".to_string(),
             uri: uri::Uri::try_from("http://purl.org/dc/elements/1.1").unwrap()
        };
        let output = gen_fragment(test_prefix);
        assert_eq!("PREFIX dc: <http://purl.org/dc/elements/1.1>", output);
    }
}

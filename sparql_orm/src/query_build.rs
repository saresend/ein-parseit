//!
//! Contains constructs that handle, manage, and implement
//! actual sparql query generation
//!

pub struct QueryBuilder {
    curr_query: String,
}

impl QueryBuilder {
    fn new() -> Self {
        QueryBuilder {
            curr_query: String::new(),
        }
    }
    pub fn write_element(&mut self, frag: &str) {
        self.curr_query.push_str(&frag);
    }

    //TODO - we can probably do something smarter via references
    fn get_result(&self) -> String {
        self.curr_query.clone()
    }
}

///
/// The key trait that makes this all work - essentially all
/// types corresponding to a sparql type should implement this type
/// it'll allow us to run a full query build and view the output,
/// as if this were an AST pass
///
pub trait QueryFragment {
    //TODO: we need error handling, this should be fallible
    fn generate_fragment(&self, builder: &mut QueryBuilder);
}

///
/// A trait to mark "complete" queries, as opposed to fragmented types
pub trait SparqlQuery {}

pub fn run_sparql_generation<T: SparqlQuery + QueryFragment>(obj: T) -> String {
    // I don't want to expose
    // a public interface to any function that allows
    // folks to generate a non-complete sparql query,
    // however since TraitSparqlQuery is a subtype of QueryFragment,
    // we can just use this internally
    gen_fragment(obj)
}

pub(crate) fn gen_fragment<T: QueryFragment>(query_fragment: T) -> String {
    let mut query_builder = QueryBuilder::new();
    query_fragment.generate_fragment(&mut query_builder);
    query_builder.get_result()
}

#[cfg(test)]
mod fragment_tests {
    // TODO: add tests after we have types which implement QueryFragment
}

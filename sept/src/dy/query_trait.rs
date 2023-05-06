use crate::{dy, Result};

/// QueryTrait represents something that can be queried using a sequence of address tokens, ultimately
/// producing a QueryViewTrait (which can be evaluated to produce a value).
pub trait QueryTrait: dy::QueryViewTrait {
    /// This returns a view object of the addressed value.
    fn run_query<'a>(
        self: Box<Self>,
        address_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn dy::QueryViewTrait + 'a>>
    where
        Self: 'a;
}

/// Starting point for creating and running queries.
pub trait QueryableDynTrait {
    /// This simply creates a query object for self, which then can have run_query called on it.
    /// Or simply call make_and_run_query.  The make_query method is mainly used by Runtime.
    fn make_query<'a>(&'a self) -> Box<dyn dy::QueryTrait + 'a>;
    // fn make_query<'a>(&'a self) -> Box<dyn dy::QueryTrait + 'a> {
    //     GenericView::new(Box::new(self))
    // }

    /// Make a query object for self and run the query, returning the appropriate query view object.
    fn make_and_run_query<'a>(
        &'a self,
        address_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn dy::QueryViewTrait + 'a>>
    where
        Self: 'a,
    {
        self.make_query().run_query(address_i)
    }
}

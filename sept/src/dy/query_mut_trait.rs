use crate::{dy, Result};

pub trait QueryMutTrait: dy::QueryMutViewTrait {
    /// This returns a mut view object of the addressed value.
    fn run_query_mut<'a>(
        self: Box<Self>,
        address_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn dy::QueryMutViewTrait + 'a>>
    where
        Self: 'a;
}

/// Starting point for creating and running mut queries.
pub trait QueryableMutDynTrait {
    /// This simply creates a query object for self, which then can have run_query called on it.
    /// Or simply call make_and_run_query.  The make_query method is mainly used by Runtime.
    fn make_query_mut<'a>(&'a mut self) -> Box<dyn dy::QueryMutTrait + 'a>;
    // fn make_query_mut<'a>(&'a mut self) -> Box<dyn dy::QueryMutTrait + 'a> {
    //     GenericMutView::new(Box::new(self))
    // }

    /// Make a query mut object for self and run the query, returning the appropriate query view object.
    fn make_and_run_query_mut<'a>(
        &'a mut self,
        address_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn dy::QueryMutViewTrait + 'a>>
    where
        Self: 'a,
    {
        self.make_query_mut().run_query_mut(address_i)
    }
}

pub trait Editable {
    // TODO: Rename to something better, this is ridiculous.
    // TODO: I think that 's is not needed and can be elided.
    fn query_mut_and_apply_edit<'s, 'a>(
        &'s mut self,
        address_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
        edit: dy::Value,
    ) -> Result<()>
    where
        's: 'a;
}

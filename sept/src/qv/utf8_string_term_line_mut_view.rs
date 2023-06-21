use crate::{dy, qv, st, Error, Result};

#[derive(Debug)]
pub struct Utf8StringTermLineMutView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. a String view object, or Box<dyn Borrow<str>>.
    // Or actually it should be EvalTrait<'b> where 'a: 'b (i.e. 'b outlives 'a).
    // Eventually there could be st-module EvalTrait that has a specific type.
    pub string: &'a mut String,

    // Cached values
    pub line_count: usize,
}

impl<'a> Utf8StringTermLineMutView<'a> {
    pub fn new(string: &'a mut String) -> Self {
        let line_count = st::split_inclusive_allow_trailing_empty(string, '\n').count();
        Self { string, line_count }
    }
}

// impl<'b> qv::QueryMutAndApplyEditTrait for Utf8StringTermLineMutView<'b> {
//     fn query_mut_and_apply_edit<'s, 'a>(
//         &'s mut self,
//         address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
//         edit: dy::Value,
//     ) -> Result<()>
//     where
//         's: 'a,
//     {
//         if let Some(address_token) = address_token_i.next() {
//             // Re-borrow address_token with a shorter lifetime.
//             let address_token = &*address_token;
//             // Re-borrow the iterator items with a shorter lifetime.
//             let mut address_token_i = address_token_i.map(|x| &*x);
//             use qv::SingleQueryMut;
//             self.run_single_query_mut(address_token)?
//                 .query_mut_and_apply_edit(&mut address_token_i, edit)
//         } else {
//             use qv::ApplyEditTrait;
//             self.apply_edit(edit)
//         }
//     }
// }

impl<'a> qv::ApplyEditTrait for Utf8StringTermLineMutView<'a> {
    fn apply_edit(&mut self, _edit: dy::Value) -> anyhow::Result<()> {
        unimplemented!("blah");
    }
}

impl<'b> qv::SingleQueryMut<dy::Value> for Utf8StringTermLineMutView<'b> {
    type ReturnType<'a> = Utf8StringTermLineMutViewQuery<'a> where 'b: 'a;
    type Error = Error;
    fn run_single_query_mut<'a>(
        &'a mut self,
        address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        if let Some(line_index) = address_token.downcast_ref::<u32>() {
            Ok(qv::Utf8StringTermLineElemMutView::new_with_cached_values(
                self.string,
                *line_index as usize,
                self.line_count,
            )?
            .into())
        } else {
            use st::Stringifiable;
            anyhow::bail!(
                "Utf8StringTermLineMutView::run_single_query_mut; unrecognized address_token {}",
                address_token.stringify()
            );
        }
    }
}

#[derive(Debug, derive_more::From)]
pub enum Utf8StringTermLineMutViewQuery<'a> {
    Utf8StringTermLineElemMutView(qv::Utf8StringTermLineElemMutView<'a>),
}

// TODO: Derive this, because it just forwards to each variant.
impl<'a> qv::ApplyEditTrait for Utf8StringTermLineMutViewQuery<'a> {
    fn apply_edit(&mut self, edit: dy::Value) -> anyhow::Result<()> {
        match self {
            Self::Utf8StringTermLineElemMutView(v) => v.apply_edit(edit),
        }
    }
}

impl<'b> qv::QueryMutAndApplyEditTrait for Utf8StringTermLineMutViewQuery<'b> {
    fn query_mut_and_apply_edit<'s, 'a>(
        &'s mut self,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
        edit: dy::Value,
    ) -> Result<()>
    where
        's: 'a,
    {
        match self {
            Self::Utf8StringTermLineElemMutView(v) => {
                v.query_mut_and_apply_edit(address_token_i, edit)
            }
        }
    }
}

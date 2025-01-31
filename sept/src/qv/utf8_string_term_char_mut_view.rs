use crate::{dy, qv, st, Error, Result};

#[derive(Debug)]
pub struct UTF8StringTermCharMutView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. a String view object, or Box<dyn Borrow<str>>.
    // Or actually it should be EvalT<'b> where 'a: 'b (i.e. 'b outlives 'a).
    // Eventually there could be st-module EvalT that has a specific type.
    pub string: &'a mut String,

    // Cached values
    pub char_count: usize,
}

impl<'a> UTF8StringTermCharMutView<'a> {
    pub fn new(string: &'a mut String) -> Self {
        let char_count = string.chars().count();
        Self { string, char_count }
    }
    pub fn new_with_cached_values(string: &'a mut String, char_count: usize) -> Self {
        assert_eq!(
            char_count,
            string.chars().count(),
            "programmer error: given char_count did not match actual char_count"
        );
        Self { string, char_count }
    }
}

// impl<'b> qv::QueryMutAndApplyEditT for UTF8StringTermCharMutView<'b> {
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
//             use qv::SingleQueryMutT;
//             self.run_single_query_mut(address_token)?
//                 .query_mut_and_apply_edit(&mut address_token_i, edit)
//         } else {
//             use qv::ApplyEditT;
//             self.apply_edit(edit)
//         }
//     }
// }

impl<'a> qv::ApplyEditT for UTF8StringTermCharMutView<'a> {
    fn apply_edit(&mut self, _edit: dy::Value) -> anyhow::Result<()> {
        unimplemented!("blah");
    }
}

impl<'b> qv::SingleQueryMutT<dy::Value> for UTF8StringTermCharMutView<'b> {
    type ReturnType<'a> = UTF8StringTermCharMutViewQuery<'a> where 'b: 'a;
    type Error = Error;
    fn run_single_query_mut<'a>(
        &'a mut self,
        address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        if let Some(char_index) = address_token.downcast_ref::<u32>() {
            Ok(qv::UTF8StringTermCharElemMutView::new_with_cached_values(
                self.string,
                *char_index as usize,
                self.char_count,
            )?
            .into())
        } else {
            use st::StringifiableT;
            anyhow::bail!(
                "UTF8StringTermCharMutView::run_single_query_mut; unrecognized address_token {}",
                address_token.stringify()
            );
        }
    }
}

#[derive(Debug, derive_more::From)]
pub enum UTF8StringTermCharMutViewQuery<'a> {
    UTF8StringTermCharElemMutView(qv::UTF8StringTermCharElemMutView<'a>),
}

// TODO: Derive this, because it just forwards to each variant.
impl<'a> qv::ApplyEditT for UTF8StringTermCharMutViewQuery<'a> {
    fn apply_edit(&mut self, edit: dy::Value) -> anyhow::Result<()> {
        match self {
            Self::UTF8StringTermCharElemMutView(v) => v.apply_edit(edit),
        }
    }
}

impl<'b> qv::QueryMutAndApplyEditT for UTF8StringTermCharMutViewQuery<'b> {
    fn query_mut_and_apply_edit<'s, 'a>(
        &'s mut self,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
        edit: dy::Value,
    ) -> Result<()>
    where
        's: 'a,
    {
        match self {
            Self::UTF8StringTermCharElemMutView(v) => {
                v.query_mut_and_apply_edit(address_token_i, edit)
            }
        }
    }
}

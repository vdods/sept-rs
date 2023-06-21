use crate::{dy, qv, st, Error, Result};

#[derive(Debug)]
pub struct Utf8StringTermLineElemCharMutView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. a String view object, or Box<dyn Borrow<str>>.
    // Or actually it should be EvalTrait<'b> where 'a: 'b (i.e. 'b outlives 'a).
    // Eventually there could be st-module EvalTrait that has a specific type.
    pub string: &'a mut String,
    pub line_index: usize,

    // Cached values -- not sure how to cache the indexed line; maybe just cache the byte offsets?
    pub line_count: usize,
    pub line_char_count: usize,
}

impl<'a> Utf8StringTermLineElemCharMutView<'a> {
    pub fn new(string: &'a mut String, line_index: usize) -> Result<Self> {
        let line_count = st::split_inclusive_allow_trailing_empty(string.as_str(), '\n').count();
        anyhow::ensure!(
            line_index < line_count,
            "Utf8StringTermLineElemCharMutView line_index was out of bounds"
        );
        let line = st::split_inclusive_allow_trailing_empty(string.as_str(), '\n')
            .nth(line_index)
            .unwrap();
        let line_char_count = line.chars().count();
        Ok(Self {
            string,
            line_index,
            line_count,
            line_char_count,
        })
    }
    pub fn new_with_cached_values(
        string: &'a mut String,
        line_index: usize,
        line_count: usize,
        line_char_count: usize,
    ) -> Result<Self> {
        assert_eq!(
            line_count,
            st::split_inclusive_allow_trailing_empty(string.as_str(), '\n').count(),
            "programmer error: given line_count did not match actual line_count"
        );
        anyhow::ensure!(
            line_index < line_count,
            "Utf8StringTermLineElemCharMutView line_index was out of bounds"
        );
        let line = st::split_inclusive_allow_trailing_empty(string.as_str(), '\n')
            .nth(line_index)
            .unwrap();
        assert_eq!(
            line_char_count,
            line.chars().count(),
            "programmer error: given line_char_count did not match actual line_char_count"
        );
        Ok(Self {
            string,
            line_index,
            line_count,
            line_char_count,
        })
    }
}

// impl<'b> qv::QueryMutAndApplyEditTrait for Utf8StringTermLineElemCharMutView<'b> {
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

impl<'a> qv::ApplyEditTrait for Utf8StringTermLineElemCharMutView<'a> {
    fn apply_edit(&mut self, _edit: dy::Value) -> anyhow::Result<()> {
        unimplemented!("blah");
    }
}

impl<'b> qv::SingleQueryMut<dy::Value> for Utf8StringTermLineElemCharMutView<'b> {
    type ReturnType<'a> = qv::Utf8StringTermLineElemCharElemMutView<'a> where 'b: 'a;
    type Error = Error;
    fn run_single_query_mut<'a>(
        &'a mut self,
        address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        if let Some(char_index) = address_token.downcast_ref::<u32>() {
            qv::Utf8StringTermLineElemCharElemMutView::new_with_cached_values(
                self.string,
                self.line_index,
                *char_index as usize,
                self.line_count,
                self.line_char_count,
            )
        } else {
            use st::Stringifiable;
            anyhow::bail!(
                "Utf8StringTermLineElemCharMutView::run_single_query_mut; unrecognized address_token {}",
                address_token.stringify()
            );
        }
    }
}

#[derive(Debug, derive_more::From)]
pub enum Utf8StringTermLineElemCharMutViewQuery<'a> {
    Utf8StringTermCharElemMutView(qv::Utf8StringTermCharElemMutView<'a>),
}

// TODO: Derive this, because it just forwards to each variant.
impl<'a> qv::ApplyEditTrait for Utf8StringTermLineElemCharMutViewQuery<'a> {
    fn apply_edit(&mut self, edit: dy::Value) -> anyhow::Result<()> {
        match self {
            Self::Utf8StringTermCharElemMutView(v) => v.apply_edit(edit),
        }
    }
}

impl<'b> qv::QueryMutAndApplyEditTrait for Utf8StringTermLineElemCharMutViewQuery<'b> {
    fn query_mut_and_apply_edit<'s, 'a>(
        &'s mut self,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
        edit: dy::Value,
    ) -> Result<()>
    where
        's: 'a,
    {
        match self {
            Self::Utf8StringTermCharElemMutView(v) => {
                v.query_mut_and_apply_edit(address_token_i, edit)
            }
        }
    }
}

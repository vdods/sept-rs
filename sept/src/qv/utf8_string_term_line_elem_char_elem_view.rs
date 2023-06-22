use crate::{dy, qv, st, Error, Result};
use std::sync::{Arc, RwLock};

// TODO: This should just be a char view that takes a Utf8StringTermLineView.
#[allow(unused)] // TEMP HACK
#[derive(Clone, Debug)]
pub struct Utf8StringTermLineElemCharElemView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. a String view object, or Box<dyn Borrow<str>>.
    // Or actually it should be EvalTrait<'b> where 'a: 'b (i.e. 'b outlives 'a).
    // Eventually there could be st-module EvalTrait that has a specific type.
    pub string: &'a str,
    pub line_index: usize,
    pub char_index: usize,

    // cached values
    pub line_count: usize,
    pub line: &'a str,
    pub line_char_count: usize,
    // TODO: Consider storing the substring instead of the char
    pub char_o: Option<char>,
}

impl<'a> Utf8StringTermLineElemCharElemView<'a> {
    pub fn new(string: &'a str, line_index: usize, char_index: usize) -> Result<Self> {
        let mut line_i = st::split_inclusive_allow_trailing_empty(string, '\n');
        let line_count = line_i.clone().count();
        anyhow::ensure!(
            line_index < line_count,
            "Utf8StringTermLineElemCharElemView line_index out of bounds"
        );
        let line = line_i.nth(line_index).unwrap();
        // TODO: This does redundant computation with determining of c; fix that.
        let line_char_count = line.chars().count();
        // The out-of-bounds condition for char_index depends on if it's in the last line or not.
        if line_index + 1 == line_count {
            // If we're in the last line, then char_index is allowed to match line_char_count.
            anyhow::ensure!(
                char_index <= line_char_count,
                "Utf8StringTermLineElemCharElemView char_index ({}) out of bounds (last line of string), max allowable char_index value is {}",
                char_index,
                line_char_count,
            );
        } else {
            // Otherwise, char_index must be less than line_char_count.
            assert!(line_char_count > 0);
            anyhow::ensure!(char_index < line_char_count, "Utf8StringTermLineElemCharElemView char_index ({}) out of bounds (non-last line of string), max allowable char_index value is {}", char_index, line_char_count-1);
        }
        let char_o = line.chars().nth(char_index);
        Ok(Self {
            string,
            line_index,
            char_index,
            line_count,
            line,
            line_char_count,
            char_o,
        })
    }
    // pub fn new_with_cached_line(
    //     string: &'a str,
    //     line_index: usize,
    //     char_index: usize,
    //     line: &'a str,
    // ) -> Result<Self> {
    //     assert_eq!(
    //         st::split_inclusive_allow_trailing_empty(string, '\n').nth(line_index),
    //         Some(line),
    //         "programmer error: actual line does not match specified cached line"
    //     );
    //     let c = line.chars().nth(char_index).ok_or_else(|| {
    //         anyhow::anyhow!("Utf8StringTermLineElemCharElemView char_index out of bounds")
    //     })?;
    //     Ok(Self {
    //         string,
    //         line_index,
    //         char_index,
    //         line,
    //         c,
    //     })
    // }
    pub fn new_with_cached_line_and_char(
        string: &'a str,
        line_index: usize,
        char_index: usize,
        line_count: usize,
        line: &'a str,
        line_char_count: usize,
        char_o: Option<char>,
    ) -> Result<Self> {
        assert_eq!(
            st::split_inclusive_allow_trailing_empty(string, '\n').count(),
            line_count,
            "programmer error: actual line_count does not match specified line_count",
        );
        assert_eq!(
            st::split_inclusive_allow_trailing_empty(string, '\n').nth(line_index),
            Some(line),
            "programmer error: actual line does not match specified cached line",
        );
        assert_eq!(
            line_char_count,
            line.chars().count(),
            "programmer error: actual line_char_count does not match specified line_char_count"
        );
        // The out-of-bounds condition for char_index depends on if it's in the last line or not.
        if line_index + 1 == line_count {
            // If we're in the last line, then char_index is allowed to match line_char_count.
            anyhow::ensure!(
                char_index <= line_char_count,
                "Utf8StringTermLineElemCharElemView char_index ({}) out of bounds (last line of string), max allowable char_index is {}", char_index, line_char_count,
            );
        } else {
            // Otherwise, char_index must be less than line_char_count.
            assert!(line_char_count > 0);
            anyhow::ensure!(char_index < line_char_count, "Utf8StringTermLineElemCharElemView char_index ({}) out of bounds (non-last line of string), max allowable char_index is {}", char_index, line_char_count-1);
        }
        assert_eq!(
            line.chars().nth(char_index),
            char_o,
            "programmer error: actual char does not match specified cached char",
        );
        Ok(Self {
            string,
            line_index,
            char_index,
            line_count,
            line,
            line_char_count,
            char_o,
        })
    }
    pub fn increment_line_index_by(&mut self, line_index_delta: isize) {
        assert!(self.line_count > 0);
        let max_line_index = self.line_count - 1;
        self.line_index = self
            .line_index
            .saturating_add_signed(line_index_delta)
            .min(max_line_index);
        // Have to update the other cached values as well.
        self.line = st::split_inclusive_allow_trailing_empty(self.string, '\n')
            .nth(self.line_index)
            .unwrap();
        self.line_char_count = self.line.chars().count();
        self.char_index = self.char_index.min(self.max_line_char_index());
        self.char_o = self.line.chars().nth(self.char_index);
    }
    pub fn go_home(&mut self) {
        self.char_index = 0;
        self.char_o = self.line.chars().nth(self.char_index);
    }
    pub fn go_end(&mut self) {
        self.char_index = self.max_line_char_index();
        self.char_o = self.line.chars().nth(self.char_index);
    }
    pub fn increment_char_index_by(&mut self, mut char_index_delta: isize, allow_line_wrap: bool) {
        if allow_line_wrap {
            while char_index_delta != 0 {
                if char_index_delta < 0 {
                    // We're going backwards
                    if self.line_index == 0 {
                        // Since we're on the first line going backwards, this effectively uses up
                        // the whole delta.
                        self.char_index = self.char_index.saturating_add_signed(char_index_delta);
                        char_index_delta = 0;
                    } else {
                        let old_char_index = self.char_index;
                        self.char_index = self.char_index.saturating_add_signed(char_index_delta);
                        let actual_char_index_delta =
                            -((old_char_index - self.char_index) as isize);
                        char_index_delta -= actual_char_index_delta;
                        if char_index_delta < 0 {
                            // If there is still more delta to go, then we must wrap around to the
                            // the previous line.
                            self.increment_line_index_by(-1);
                            self.go_end();
                            // This uses up one char of the delta.
                            char_index_delta += 1;
                        }
                    }
                } else if char_index_delta > 0 {
                    // We're going forwards
                    if self.line_index + 1 == self.line_count {
                        // Since we're on the last line going forwards, this effectively uses up
                        // the whole delta.
                        self.char_index = self
                            .char_index
                            .saturating_add_signed(char_index_delta)
                            .min(self.max_line_char_index());
                        char_index_delta = 0;
                    } else {
                        let old_char_index = self.char_index;
                        self.char_index = self
                            .char_index
                            .saturating_add_signed(char_index_delta)
                            .min(self.max_line_char_index());
                        let actual_char_index_delta = (self.char_index - old_char_index) as isize;
                        char_index_delta -= actual_char_index_delta;
                        if char_index_delta > 0 {
                            // If there is still more delta to go, then we must wrap around to the
                            // the next line.
                            self.increment_line_index_by(1);
                            self.go_home();
                            // This uses up one char of the delta.
                            char_index_delta -= 1;
                        }
                    }
                } else {
                    // No delta, do nothing.
                }
            }
        } else {
            self.char_index = self
                .char_index
                .saturating_add_signed(char_index_delta)
                .min(self.max_line_char_index())
        }
        self.char_o = self.line.chars().nth(self.char_index);
    }
    /// The precise max on line_char_count depends on which line it is.  All but the last
    /// line ends with newline, which by definition can't have any chars after it.
    /// The last line can be indexed 1 past the last char.
    fn max_line_char_index(&self) -> usize {
        if self.line_index + 1 == self.line_count {
            // Last line
            self.line_char_count
        } else {
            // Non-last line; the line should end with '\n' and should therefore have at least 1 char.
            assert!(self.line.ends_with('\n'));
            assert!(self.line_char_count > 0);
            self.line_char_count - 1
        }
    }
    // TODO: Make methods which only return (line_index, char_index) pairs representing the
    // results of certain cursor movements that may correspond to string edits and therefore
    // the constraints checking can't be done until after the edit is made.
}

impl<'b> qv::QueryTrait for Utf8StringTermLineElemCharElemView<'b> {
    fn run_query<'a>(
        self: Box<Self>,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn qv::EvalTrait + 'a>>
    where
        'b: 'a,
    {
        let mut address_token_i = address_token_i.peekable();
        // If we're at the end of the address, then this is the value we're looking for.
        if address_token_i.peek().is_none() {
            return Ok(self);
        }
        let first_address = address_token_i.next().unwrap();
        // TODO: If char ever gets further queries (e.g. numeric unicode value), then pass them on here.
        use st::Stringifiable;
        anyhow::bail!(
            "Utf8StringTerm query doesn't support address: {}",
            first_address.stringify()
        );
    }
}

impl<'b> qv::EvalTrait for Utf8StringTermLineElemCharElemView<'b> {
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        use dy::IntoValue;
        // We already retrieved the char during construction, so just return the value.
        Ok(dy::MaybeDereferencedValue::make_value_la(Arc::new(
            RwLock::new(if let Some(c) = self.char_o {
                c.into_value()
            } else {
                // NOTE: Returning Void is a bit of a hack.  Maybe there should be some "End" NPTerm instead.
                st::Void.into_value()
            }),
        )))
    }
}

#[derive(Clone, Debug, derive_more::From)]
pub enum Utf8StringTermLineElemCharElemViewQuery {}

impl<'b> qv::SingleQuery<dy::Value> for Utf8StringTermLineElemCharElemView<'b> {
    type ReturnType<'a> = Utf8StringTermLineElemCharElemViewQuery where 'b: 'a;
    type Error = Error;
    fn run_single_query<'a>(
        &'a self,
        _address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        // TODO: Once char has queries, forward to that.
        anyhow::bail!(
            "Utf8StringTermLineElemCharElemView::run_single_query does not support any queries"
        );
    }
}

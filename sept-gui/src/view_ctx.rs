use crate::{
    ANSIColor, Edit, LayoutDiscriminant, LayoutMode, Model, ViewCtxNestingGuard,
    ViewCtxRenderAddressGuard, ViewCtxTAGuard, ViewOptions,
};
use std::{cmp::Ordering, collections::VecDeque};

/// Provides control over how things are rendered.
// TODO: Maybe this should be called UpdateCtx, and there could be a separate context for EventHandlerT.
// Though that also would be a problem because both need to be able to output commands.
pub struct ViewCtx<'a> {
    /// This is the sept::dy::Value that's being viewed.  It will be passed in from the thing
    /// that's rendering this view.
    pub model: &'a Model,
    /// This defines how the rendering is done.  It will be passed in from the thing that's rendering this view.
    pub view_options: &'a ViewOptions,
    /// This is the address of the current cursor.  It will be passed in from the thing that's rendering this view.
    pub cursor_address_o: Option<&'a mut sept::dy::TupleTerm>,
    /// This is the address of the currently-being-rendered term.
    // TODO: This should probably be renamed to "update address", although I don't really like "update" either.
    pub render_address: sept::dy::TupleTerm,
    /// Current nesting depth.
    pub current_nesting_depth: u32,
    /// If Some(_), will override self.view_options.show_type_annotations.
    pub override_show_type_annotations_o: Option<bool>,
    /// This is the queue of edits generated this frame that should be executed after
    /// the frame is done rendering.
    // TODO: Replace with Action?
    pub enqueued_edit_v: VecDeque<Edit>,
}

impl<'b> ViewCtx<'b> {
    pub fn new(
        model: &'b Model,
        view_options: &'b ViewOptions,
        cursor_address_o: Option<&'b mut sept::dy::TupleTerm>,
    ) -> Self {
        Self {
            model,
            view_options,
            cursor_address_o,
            render_address: sept::dy::TupleTerm::from(vec![]),
            current_nesting_depth: 0,
            override_show_type_annotations_o: None,
            enqueued_edit_v: VecDeque::new(),
        }
    }
    pub fn cursor_address(&self) -> &sept::dy::TupleTerm {
        self.cursor_address_o.as_deref().unwrap()
    }
    pub fn push_nesting_depth<'a>(&'a mut self) -> ViewCtxNestingGuard<'a, 'b>
    where
        'b: 'a,
    {
        ViewCtxNestingGuard::new(self)
    }
    pub fn push_show_type_annotations<'a>(
        &'a mut self,
        show_type_annotations: bool,
    ) -> ViewCtxTAGuard<'a, 'b>
    where
        'b: 'a,
    {
        ViewCtxTAGuard::new(self, show_type_annotations)
    }
    pub fn push_render_address_token<'a>(
        &'a mut self,
        address_token: sept::dy::Value,
    ) -> ViewCtxRenderAddressGuard<'a, 'b>
    where
        'b: 'a,
    {
        ViewCtxRenderAddressGuard::new(self, address_token)
    }
    pub fn should_show_type_annotations(&self) -> bool {
        self.override_show_type_annotations_o
            .unwrap_or(self.view_options.show_type_annotations)
    }
    pub fn enqueue_edit(&mut self, edit: impl Into<Edit>) {
        self.enqueued_edit_v.push_back(edit.into());
    }
    /// This returns (foreground_color, background_color) based on the given foreground_color and the
    /// current state of highlightedness based on the render_address compared to the cursor_address.
    // TODO: Consider making a struct for the return type.
    pub fn set_highlight_if_necessary(
        &self,
        foreground_color: egui::Color32,
    ) -> (egui::Color32, egui::Color32, egui::Stroke) {
        if self.render_address_is_subaddress_of_cursor_address() {
            (
                foreground_color,
                self.color_for_cursor_background(),
                egui::Stroke {
                    width: 1.0,
                    color: ANSIColor::BRIGHT_WHITE,
                },
            )
        } else {
            (
                foreground_color,
                egui::Color32::TRANSPARENT,
                egui::Stroke::NONE,
            )
        }
    }
    pub fn cursor_address_push(&mut self, address_token: sept::dy::Value) {
        if let Some(cursor_address) = self.cursor_address_o.as_deref_mut() {
            cursor_address.push(address_token);
        } else {
            panic!("No cursor_address to push to");
        }
    }
    pub fn cursor_address_pop(&mut self) -> sept::dy::Value {
        if let Some(cursor_address) = self.cursor_address_o.as_deref_mut() {
            cursor_address
                .pop()
                .expect("programmer error: can't pop from cursor_address because it was empty")
        } else {
            panic!("No cursor_address to pop from");
        }
    }
    /// Returns a reference to the nth-to-last token.
    pub fn cursor_address_nth_to_last_token(&self, n: usize) -> &sept::dy::Value {
        if let Some(cursor_address) = self.cursor_address_o.as_deref() {
            cursor_address
                .iter()
                .rev()
                .nth(n)
                .expect("programmer error: cursor_address had insufficient tokens")
        } else {
            panic!("cursor_address_o was None");
        }
    }
    /// Returns a mutable reference to the nth-to-last token.
    pub fn cursor_address_nth_to_last_token_mut(&mut self, n: usize) -> &mut sept::dy::Value {
        if let Some(cursor_address) = self.cursor_address_o.as_deref_mut() {
            cursor_address
                .iter_mut()
                .rev()
                .nth(n)
                .expect("programmer error: cursor_address had insufficient tokens")
        } else {
            panic!("cursor_address_o was None");
        }
    }
    /// This assumes that the last token in the cursor address is a u32.
    pub fn cursor_address_last_token_u32_set(&mut self, new_value: u32) {
        if let Some(cursor_address) = self.cursor_address_o.as_deref_mut() {
            let last_token_u32 = cursor_address.last_mut().expect("programmer error: can't set cursor_address last token because cursor_address was empty").downcast_mut::<u32>().expect("programmer error: can't set cursor_address last token because it isn't a u32");
            *last_token_u32 = new_value;
        } else {
            panic!("No cursor_address to set");
        }
    }
    // /// This assumes that the nth-to-last token in the cursor address is a u32.
    // pub fn cursor_address_nth_to_last_token_u32_set(&mut self, new_value: u32) {
    //     if let Some(cursor_address) = self.cursor_address_o.as_deref_mut() {
    //         let nth_to_last_token_u32 = cursor_address.iter_mut().rev().nth(n).expect("programmer error: can't set cursor_address nth-to-last token because cursor_address was empty").downcast_mut::<u32>().expect("programmer error: can't set cursor_address last token because it isn't a u32");
    //         *nth_to_last_token_u32 = new_value;
    //     } else {
    //         panic!("No cursor_address to set");
    //     }
    // }
    /// This assumes that the last token in the cursor address is a u32, and will use
    /// u32::saturating_add_signed to apply the increment, and will also saturate at the
    /// given max value.
    pub fn cursor_address_last_token_u32_increment_by(&mut self, increment: i32, max: u32) {
        if let Some(cursor_address) = self.cursor_address_o.as_deref_mut() {
            let last_token_u32 = cursor_address.last_mut().expect("programmer error: can't increment cursor_address last token because cursor_address was empty").downcast_mut::<u32>().expect("programmer error: can't increment cursor_address last token because it isn't a u32");
            *last_token_u32 = last_token_u32.saturating_add_signed(increment).min(max);
        } else {
            panic!("No cursor_address to increment");
        }
    }
    /// This assumes that the nth-to-last token in the cursor address is a u32, and will use
    /// u32::saturating_add_signed to apply the increment, and will also saturate at the
    /// given max value.
    pub fn cursor_address_nth_to_last_token_u32_increment_by(
        &mut self,
        n: usize,
        increment: i32,
        max: u32,
    ) {
        if let Some(cursor_address) = self.cursor_address_o.as_deref_mut() {
            let nth_to_last_token_u32 = cursor_address.iter_mut().rev().nth(n).expect("programmer error: can't increment cursor_address nth-to-last token because cursor_address had insufficient tokens").downcast_mut::<u32>().expect("programmer error: can't increment cursor_address nth-to-last token because it isn't a u32");
            *nth_to_last_token_u32 = nth_to_last_token_u32
                .saturating_add_signed(increment)
                .min(max);
        } else {
            panic!("No cursor_address to increment");
        }
    }
    pub fn render_address_is_cursor_address(&self) -> bool {
        if let Some(cursor_address) = self.cursor_address_o.as_deref() {
            self.render_address == *cursor_address
        } else {
            false
        }
    }
    pub fn render_address_is_parent_of_cursor_address(&self) -> bool {
        if let Some(cursor_address) = self.cursor_address_o.as_deref() {
            sept::dy::prefix_partial_cmp(&self.render_address, cursor_address)
                == Some(std::cmp::Ordering::Less)
                && self.render_address.len() + 1 == cursor_address.len()
        } else {
            false
        }
    }
    /// Returns the matching postfix address iff cursor_address is a subaddress of render_address and the
    /// postfix tokens in cursor_address beyond render_address inhabit the given subaddress_type_t.
    pub fn cursor_match_subaddress_of_render_address(
        &self,
        subaddress_type_t: &sept::dy::TupleTerm,
    ) -> Option<&[sept::dy::Value]> {
        if let Some(subaddress_token_v) = self.cursor_address_is_subaddress_of_render_address() {
            if subaddress_token_v.len() != subaddress_type_t.len() {
                return None;
            }
            for i in 0..subaddress_token_v.len() {
                use sept::st::InhabitsT;
                if !subaddress_token_v[i].inhabits(&subaddress_type_t[i]) {
                    return None;
                }
            }
            Some(&subaddress_token_v)
        } else {
            None
        }
    }
    fn cursor_address_is_subaddress_of_render_address(&self) -> Option<&[sept::dy::Value]> {
        if let Some(cursor_address) = self.cursor_address_o.as_deref() {
            if let Some(ordering) =
                sept::dy::prefix_partial_cmp(&self.render_address, cursor_address)
            {
                if ordering.is_lt() {
                    assert!(self.render_address.len() <= cursor_address.len());
                    // An address A is a "subaddress" of address B if B <= A, i.e. the specific data value that A addresses is
                    // "contained within" the data value that B addresses.
                    Some(&cursor_address.as_slice()[self.render_address.len()..])
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    fn render_address_is_subaddress_of_cursor_address(&self) -> bool {
        if let Some(cursor_address) = self.cursor_address_o.as_deref() {
            if let Some(ordering) =
                sept::dy::prefix_partial_cmp(&self.render_address, cursor_address)
            {
                // An address A is a "subaddress" of address B if B <= A, i.e. the specific data value that A addresses is
                // "contained within" the data value that B addresses.
                ordering.is_ge()
            } else {
                false
            }
        } else {
            false
        }
    }
    /// If cursor_address is a proper subaddress of render_address, then render_address can "follow"
    /// cursor_address.  This will return Some(token) with the next address token to push onto render_address
    /// if it's possible to "follow" (in this case, call that token the "guide token"), otherwise None.
    /// To give concrete examples, if render_address is (0, 2) and cursor_address is (0, 2, "line", 1),
    /// then the guide token is "line", whereas if render_address is (0, 2) and cursor_address is (0, 1)
    /// or (1, 2, "line", 1) or (0,), then the guide token isn't defined, and this method returns None.
    pub fn cursor_address_guide_token(&self) -> Option<&sept::dy::Value> {
        if let Some(cursor_address) = self.cursor_address_o.as_deref() {
            match sept::dy::prefix_partial_cmp(&self.render_address, cursor_address) {
                Some(std::cmp::Ordering::Less) => {
                    assert!(cursor_address.len() > self.render_address.len());
                    Some(&cursor_address[self.render_address.len()])
                }
                _ => None,
            }
        } else {
            None
        }
    }
    pub fn layout_mode(&self) -> LayoutMode {
        if self.current_nesting_depth < self.view_options.inline_at_nesting_depth {
            LayoutMode::Expanded
        } else {
            LayoutMode::Inline
        }
    }
    pub fn layout_discriminant(&self) -> LayoutDiscriminant {
        match self
            .current_nesting_depth
            .cmp(&self.view_options.inline_at_nesting_depth)
        {
            Ordering::Less => LayoutDiscriminant::Expanded,
            Ordering::Equal => LayoutDiscriminant::BoundaryLevelInline,
            Ordering::Greater => LayoutDiscriminant::InteriorLevelInline,
        }
    }
}

impl std::ops::Deref for ViewCtx<'_> {
    type Target = ViewOptions;
    fn deref(&self) -> &Self::Target {
        &self.view_options
    }
}

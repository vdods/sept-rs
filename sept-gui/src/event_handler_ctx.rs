use crate::{
    AddressedEdit, Command, EventHandlerCtxNestingGuard, LayoutDiscriminant, LayoutMode,
    ViewOptions,
};
use sept::dy::IntoValue;
use std::collections::VecDeque;

pub struct EventHandlerCtx<'a> {
    pub root_value: &'a sept::dy::Value,
    pub cursor_address: &'a sept::dy::TupleTerm,
    /// Current nesting depth.
    pub current_nesting_depth: u32,
    // This only partially belongs here.  It's used for page_up_down_delta and layout_mode,
    // but it might make sense to separate things out.
    view_options: &'a ViewOptions,
    pub remaining_event_v: &'a mut VecDeque<egui::Event>,
    enqueued_command_v: &'a mut VecDeque<Command>,
}

impl<'a> EventHandlerCtx<'a> {
    pub fn new(
        root_value: &'a sept::dy::Value,
        cursor_address: &'a sept::dy::TupleTerm,
        view_options: &'a ViewOptions,
        remaining_event_v: &'a mut VecDeque<egui::Event>,
        enqueued_command_v: &'a mut VecDeque<Command>,
    ) -> Self {
        Self {
            root_value,
            cursor_address,
            current_nesting_depth: 0,
            view_options,
            remaining_event_v,
            enqueued_command_v,
        }
    }
    pub fn page_up_down_delta(&self) -> u32 {
        self.view_options.page_up_down_delta
    }
    pub fn layout_mode(&self) -> LayoutMode {
        if self.current_nesting_depth < self.view_options.inline_at_nesting_depth {
            LayoutMode::Expanded
        } else {
            LayoutMode::Inline
        }
    }
    pub fn layout_discriminant(&self) -> LayoutDiscriminant {
        use std::cmp::Ordering;
        match self
            .current_nesting_depth
            .cmp(&self.view_options.inline_at_nesting_depth)
        {
            Ordering::Less => LayoutDiscriminant::Expanded,
            Ordering::Equal => LayoutDiscriminant::BoundaryLevelInline,
            Ordering::Greater => LayoutDiscriminant::InteriorLevelInline,
        }
    }
    pub fn enqueue_command(&mut self, command: Command) {
        self.enqueued_command_v.push_back(command);
    }
    /// Note that this can't be used twice in the same handle_event pass, since it has to know the
    /// cursor_address len in order to generate the CursorEdit commands.
    pub fn enqueue_command_cursor_address_push(
        &mut self,
        address_token_i: impl std::iter::Iterator<Item = sept::dy::Value>,
    ) {
        let mut cursor_len = self.cursor_address.len() as u32;
        for address_token in address_token_i {
            self.enqueue_command(Command::CursorEdit(AddressedEdit {
                address: vec![(cursor_len as u32).into_value()].into(),
                edit: sept::qv::InsertionTerm {
                    new_data: address_token,
                }
                .into(),
            }));
            cursor_len += 1;
        }
    }
    /// Note that this can't be used twice in the same handle_event pass, since it has to know the
    /// cursor_address len in order to generate the CursorEdit commands.
    pub fn enqueue_command_cursor_address_pop(&mut self, pop_count: usize) {
        if pop_count > self.cursor_address.len() {
            panic!("programmer error: cursor_address underflow");
        }
        let mut cursor_len = self.cursor_address.len() as u32;
        for address_token in self.cursor_address.iter().rev().take(pop_count) {
            self.enqueue_command(Command::CursorEdit(AddressedEdit {
                // TODO: This could use `-1` as the address once negative indexing is supported.
                address: vec![(cursor_len - 1).into_value()].into(),
                edit: sept::qv::DeletionTerm {
                    old_data: address_token.clone(),
                }
                .into(),
            }));
            cursor_len -= 1;
        }
    }
    pub fn push_nesting_depth<'b>(&'b mut self) -> EventHandlerCtxNestingGuard<'b, 'a>
    where
        'a: 'b,
    {
        EventHandlerCtxNestingGuard::new(self)
    }
}

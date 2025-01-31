use crate::{
    first_char_stripped_string, AddressedEdit, CursorEdit, EventHandlerCtx, EventHandlerT,
    RootValueEdit,
};
use anyhow::Result;
use sept::dy::IntoValueT;

impl EventHandlerT for sept::st::Utf8StringTerm {
    fn handle_event(
        &self,
        event: egui::Event,
        event_handler_ctx: &mut EventHandlerCtx<'_>,
        cursor_address_token_i: &mut dyn std::iter::Iterator<Item = &sept::dy::Value>,
    ) -> Result<Option<egui::Event>> {
        if let Some(cursor_address_token) = cursor_address_token_i.next() {
            let mut event_handler_ctx_g = event_handler_ctx.push_nesting_depth();
            let event_handler_ctx = &mut event_handler_ctx_g;
            use sept::qv::SingleQueryT;
            // TODO: Just make sept::qv::Utf8StringTermQuery impl EventHandlerT
            match self.run_single_query(cursor_address_token)? {
                sept::qv::Utf8StringTermQuery::Utf8StringTermCharView(v) => {
                    v.handle_event(event, event_handler_ctx, cursor_address_token_i)
                }
                sept::qv::Utf8StringTermQuery::Utf8StringTermLineView(v) => {
                    v.handle_event(event, event_handler_ctx, cursor_address_token_i)
                }
            }
        } else {
            // This is the value addressed by the cursor.
            // TODO: Probably put this into a method in EventHandlerT.
            match event {
                // egui::Event::Paste(string) => {
                // TODO: Create a replace or insert edit, depending on the mode.
                // }
                // egui::Event::Text(string) => {
                // TODO: Create a replace or insert edit, depending on the mode.
                // }
                egui::Event::Key {
                    key: egui::Key::Enter,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    // Unconditionally go into "line-char" mode at the end of the string.
                    enter_line_char_mode(self, event_handler_ctx, EnterLineCharModeAt::End);
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Text(string) if string.starts_with("\"") => {
                    // Unconditionally go into "line-char" mode at the beginning of the string.
                    enter_line_char_mode(self, event_handler_ctx, EnterLineCharModeAt::Beginning);
                    // Take the used char off the front of the string and push the rest back onto the remaining events,
                    // if there's anything left of the string after the first char.
                    if let Some(string) = first_char_stripped_string(string) {
                        event_handler_ctx
                            .remaining_event_v
                            .push_front(egui::Event::Text(string));
                    }
                    // We consumed the event.
                    Ok(None)
                }
                event => {
                    // We didn't consume the event, so return it.
                    Ok(Some(event))
                }
            }
        }
    }
}

impl<'a> EventHandlerT for sept::qv::Utf8StringTermCharView<'a> {
    fn handle_event(
        &self,
        event: egui::Event,
        event_handler_ctx: &mut EventHandlerCtx<'_>,
        cursor_address_token_i: &mut dyn std::iter::Iterator<Item = &sept::dy::Value>,
    ) -> Result<Option<egui::Event>> {
        if let Some(cursor_address_token) = cursor_address_token_i.next() {
            use sept::qv::SingleQueryT;
            match self.run_single_query(cursor_address_token)? {
                sept::qv::Utf8StringTermCharViewQuery::Utf8StringTermCharElemView(v) => {
                    v.handle_event(event, event_handler_ctx, cursor_address_token_i)
                }
            }
        } else {
            // This is the value addressed by the cursor.
            // TODO: Probably put this into a method in EventHandlerT.
            match event {
                event => {
                    // We didn't consume the event, so return it.
                    Ok(Some(event))
                }
            }
        }
    }
}

impl<'a> EventHandlerT for sept::qv::Utf8StringTermCharElemView<'a> {
    fn handle_event(
        &self,
        event: egui::Event,
        event_handler_ctx: &mut EventHandlerCtx<'_>,
        cursor_address_token_i: &mut dyn std::iter::Iterator<Item = &sept::dy::Value>,
    ) -> Result<Option<egui::Event>> {
        if let Some(cursor_address_token) = cursor_address_token_i.next() {
            use sept::qv::SingleQueryT;
            match self.run_single_query(cursor_address_token)? {}
        } else {
            // This is the value addressed by the cursor.
            // TODO: Probably put this into a method in EventHandlerT.
            match event {
                egui::Event::Key {
                    key: egui::Key::Enter,
                    pressed: true,
                    modifiers: egui::Modifiers::ALT,
                    ..
                }
                | egui::Event::Key {
                    key: egui::Key::Escape,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    // Escape this view by taking off the last two cursor tokens.
                    event_handler_ctx.enqueue_command_cursor_address_pop(2);
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Paste(string) | egui::Event::Text(string) => {
                    // TEMP HACK -- insert char by char for now, support insertion of strings later.
                    let cursor_len = event_handler_ctx.cursor_address.len() as u32;
                    assert!(cursor_len >= 2);

                    for c in string.chars() {
                        // Root value edit
                        event_handler_ctx.enqueue_edit(RootValueEdit::from(AddressedEdit {
                            address: event_handler_ctx.cursor_address.clone(),
                            edit: sept::qv::InsertionTerm { new_data: c.into() }.into(),
                        }));

                        enqueue_char_mode_cursor_edit(
                            self.char_index as u32,
                            (self.char_index as u32).saturating_add(1),
                            event_handler_ctx,
                        )
                    }
                    // We consumed the event.
                    Ok(None)
                }
                // TODO: Handle copy/cut
                egui::Event::Key {
                    key: egui::Key::ArrowLeft,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    // Cursor edit - Update the char_index
                    {
                        let old_char_index = self.char_index as u32;
                        let new_char_index = {
                            let mut v = self.clone();
                            v.increment_char_index_by(-1);
                            v.char_index as u32
                        };
                        enqueue_char_mode_cursor_edit(
                            old_char_index,
                            new_char_index,
                            event_handler_ctx,
                        );
                    }
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::ArrowRight,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    // Cursor edit - Update the char_index
                    {
                        let old_char_index = self.char_index as u32;
                        let new_char_index = {
                            let mut v = self.clone();
                            v.increment_char_index_by(1);
                            v.char_index as u32
                        };
                        enqueue_char_mode_cursor_edit(
                            old_char_index,
                            new_char_index,
                            event_handler_ctx,
                        );
                    }
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::Home,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    // Cursor edit - Update the char_index
                    {
                        let old_char_index = self.char_index as u32;
                        let new_char_index = {
                            let mut v = self.clone();
                            v.go_home();
                            v.char_index as u32
                        };
                        enqueue_char_mode_cursor_edit(
                            old_char_index,
                            new_char_index,
                            event_handler_ctx,
                        );
                    }
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::End,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    // Cursor edit - Update the char_index
                    {
                        let old_char_index = self.char_index as u32;
                        let new_char_index = {
                            let mut v = self.clone();
                            v.go_end();
                            v.char_index as u32
                        };
                        enqueue_char_mode_cursor_edit(
                            old_char_index,
                            new_char_index,
                            event_handler_ctx,
                        );
                    }
                    // We consumed the event.
                    Ok(None)
                }
                // egui::Event::Key {
                //     key: egui::Key::PageUp,
                //     pressed: true,
                //     modifiers: egui::Modifiers::NONE,
                //     ..
                // } => {
                //     view_ctx.cursor_address_last_token_u32_increment_by(-view_ctx.page_up_down_delta, self.char_count as u32);
                // // We consumed the event.
                // Ok(None)
                // }
                // egui::Event::Key {
                //     key: egui::Key::PageDown,
                //     pressed: true,
                //     modifiers: egui::Modifiers::NONE,
                //     ..
                // } => {
                //     view_ctx.cursor_address_last_token_u32_increment_by(view_ctx.page_up_down_delta, self.char_count as u32);
                // // We consumed the event.
                // Ok(None)
                // }
                egui::Event::Key {
                    key: egui::Key::Delete,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    let cursor_char_o = {
                        use sept::qv::QueryableDynT;
                        event_handler_ctx
                            .root_value
                            .make_and_run_query(&mut event_handler_ctx.cursor_address.iter())
                            .unwrap()
                            .eval()
                            .unwrap()
                            .read()
                            .downcast_ref::<char>()
                            .map(|c| *c)
                    };
                    if let Some(cursor_char) = cursor_char_o {
                        event_handler_ctx.enqueue_edit(RootValueEdit::from(AddressedEdit {
                            address: event_handler_ctx.cursor_address.clone(),
                            edit: sept::qv::DeletionTerm {
                                old_data: cursor_char.into(),
                            }
                            .into(),
                        }));
                    }
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::Backspace,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    // Only Backspace if we're not at the beginning of the line.
                    // TODO: Implement line wrap
                    if self.char_index > 0 {
                        let mut v = self.clone();
                        // TODO: Impl wrap and put it in ViewOptions
                        v.increment_char_index_by(-1);
                        let cursor_char_o = {
                            use sept::qv::EvalT;
                            let cursor_value_la = v.eval().unwrap();
                            // This view will produce a char if the cursor is in bounds.
                            let cursor_char_o =
                                cursor_value_la.read().downcast_ref::<char>().map(|c| *c);
                            cursor_char_o
                        };
                        if let Some(cursor_char) = cursor_char_o {
                            // Only if there was a char to Backspace through should we enqueue commands
                            // to edit the root value and update the cursor.

                            // Compute the updated cursor_address and make the Root value edit
                            {
                                let mut updated_cursor_address =
                                    event_handler_ctx.cursor_address.clone();
                                let cursor_len = updated_cursor_address.len();
                                assert!(cursor_len >= 2);
                                updated_cursor_address[cursor_len - 1] =
                                    (v.char_index as u32).into_value();

                                event_handler_ctx.enqueue_edit(RootValueEdit::from(
                                    AddressedEdit {
                                        address: updated_cursor_address.clone(),
                                        edit: sept::qv::DeletionTerm {
                                            old_data: cursor_char.into(),
                                        }
                                        .into(),
                                    },
                                ));
                            }

                            enqueue_char_mode_cursor_edit(
                                self.char_index as u32,
                                v.char_index as u32,
                                event_handler_ctx,
                            );
                        }
                    }
                    // We consumed the event.
                    Ok(None)
                }
                // TODO: Change this into egui::Event::Text
                egui::Event::Key {
                    key,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } if key == egui::Key::Enter || key == egui::Key::Tab => {
                    let c = if key == egui::Key::Enter { '\n' } else { '\t' };

                    let cursor_len = event_handler_ctx.cursor_address.len() as u32;
                    assert!(cursor_len >= 2);
                    let cursor_address = event_handler_ctx.cursor_address.clone();

                    // Root value edit
                    event_handler_ctx.enqueue_edit(RootValueEdit::from(AddressedEdit {
                        address: cursor_address.clone(),
                        edit: sept::qv::InsertionTerm { new_data: c.into() }.into(),
                    }));
                    enqueue_char_mode_cursor_edit(
                        self.char_index as u32,
                        (self.char_index as u32).saturating_add(1),
                        event_handler_ctx,
                    );

                    // We consumed the event.
                    Ok(None)
                }
                event => {
                    // We didn't consume the event, so return it.
                    Ok(Some(event))
                }
            }
        }
    }
}

impl<'a> EventHandlerT for sept::qv::Utf8StringTermLineView<'a> {
    fn handle_event(
        &self,
        event: egui::Event,
        event_handler_ctx: &mut EventHandlerCtx<'_>,
        cursor_address_token_i: &mut dyn std::iter::Iterator<Item = &sept::dy::Value>,
    ) -> Result<Option<egui::Event>> {
        if let Some(cursor_address_token) = cursor_address_token_i.next() {
            use sept::qv::SingleQueryT;
            match self.run_single_query(cursor_address_token)? {
                sept::qv::Utf8StringTermLineViewQuery::Utf8StringTermLineElemView(v) => {
                    v.handle_event(event, event_handler_ctx, cursor_address_token_i)
                }
            }
        } else {
            // This is the value addressed by the cursor.
            // TODO: Probably put this into a method in EventHandlerT.
            match event {
                event => {
                    // We didn't consume the event, so return it.
                    Ok(Some(event))
                }
            }
        }
    }
}

impl<'a> EventHandlerT for sept::qv::Utf8StringTermLineElemView<'a> {
    fn handle_event(
        &self,
        event: egui::Event,
        event_handler_ctx: &mut EventHandlerCtx<'_>,
        cursor_address_token_i: &mut dyn std::iter::Iterator<Item = &sept::dy::Value>,
    ) -> Result<Option<egui::Event>> {
        if let Some(cursor_address_token) = cursor_address_token_i.next() {
            let mut event_handler_ctx_g = event_handler_ctx.push_nesting_depth();
            let event_handler_ctx = &mut event_handler_ctx_g;
            use sept::qv::SingleQueryT;
            match self.run_single_query(cursor_address_token)? {
                sept::qv::Utf8StringTermLineElemViewQuery::Utf8StringTermLineElemCharView(v) => {
                    v.handle_event(event, event_handler_ctx, cursor_address_token_i)
                }
            }
        } else {
            // This is the value addressed by the cursor.
            // TODO: Probably put this into a method in EventHandlerT.
            match event {
                egui::Event::Key {
                    key: egui::Key::Enter,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    // Enter this Utf8StringTerm in "char" view at element 0.
                    event_handler_ctx.enqueue_command_cursor_address_push(
                        ["char".to_string().into(), 0u32.into()].into_iter(),
                    );
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::Enter,
                    pressed: true,
                    modifiers: egui::Modifiers::ALT,
                    ..
                }
                | egui::Event::Key {
                    key: egui::Key::Escape,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    // Escape this view by taking off the last two cursor tokens.
                    event_handler_ctx.enqueue_command_cursor_address_pop(2);
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Paste(_string) => {
                    // TODO: insert/replace -- have to figure out how to handle newlines
                    // Probably insert (or replace) the existing string with the given text and append a newline to the text if it doesn't have one already.
                    // unimplemented!("blah");
                    // We consumed the event.
                    Ok(None)
                }
                // egui::Event::Text(_string) => {
                //     // Not sure if this should do anything.
                //     // We consumed the event.
                //     Ok(None)
                // }
                // TODO: Handle copy/cut
                egui::Event::Key {
                    key: egui::Key::ArrowUp,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    // TODO: This should depend on LayoutMode
                    // Cursor edit - Update the line_index
                    {
                        let old_line_index = self.line_index as u32;
                        let new_line_index = {
                            let mut v = self.clone();
                            v.increment_line_index_by(-1);
                            v.line_index as u32
                        };
                        enqueue_line_mode_cursor_edit(
                            old_line_index,
                            new_line_index,
                            event_handler_ctx,
                        );
                    }
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::ArrowDown,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    // TODO: This should depend on LayoutMode
                    // Cursor edit - Update the line_index
                    {
                        let old_line_index = self.line_index as u32;
                        let new_line_index = {
                            let mut v = self.clone();
                            v.increment_line_index_by(1);
                            v.line_index as u32
                        };
                        enqueue_line_mode_cursor_edit(
                            old_line_index,
                            new_line_index,
                            event_handler_ctx,
                        );
                    }
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::ArrowLeft,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    // TODO: This should depend on LayoutMode
                    // Cursor edit - Update the line_index
                    {
                        let old_line_index = self.line_index as u32;
                        let new_line_index = {
                            let mut v = self.clone();
                            v.increment_line_index_by(-1);
                            v.line_index as u32
                        };
                        enqueue_line_mode_cursor_edit(
                            old_line_index,
                            new_line_index,
                            event_handler_ctx,
                        );
                    }
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::ArrowRight,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    // TODO: This should depend on LayoutMode
                    // Cursor edit - Update the line_index
                    {
                        let old_line_index = self.line_index as u32;
                        let new_line_index = {
                            let mut v = self.clone();
                            v.increment_line_index_by(1);
                            v.line_index as u32
                        };
                        enqueue_line_mode_cursor_edit(
                            old_line_index,
                            new_line_index,
                            event_handler_ctx,
                        );
                    }
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::Home,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    // Cursor edit - Update the line_index
                    {
                        let old_line_index = self.line_index as u32;
                        let new_line_index = {
                            let mut v = self.clone();
                            v.go_home();
                            v.line_index as u32
                        };
                        enqueue_line_mode_cursor_edit(
                            old_line_index,
                            new_line_index,
                            event_handler_ctx,
                        );
                    }
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::End,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    // Cursor edit - Update the line_index
                    {
                        let old_line_index = self.line_index as u32;
                        let new_line_index = {
                            let mut v = self.clone();
                            v.go_end();
                            v.line_index as u32
                        };
                        enqueue_line_mode_cursor_edit(
                            old_line_index,
                            new_line_index,
                            event_handler_ctx,
                        );
                    }
                    // We consumed the event.
                    Ok(None)
                }
                // egui::Event::Key {
                //     key: egui::Key::PageUp,
                //     pressed: true,
                //     modifiers: egui::Modifiers::NONE,
                //     ..
                // } => {
                // // We consumed the event.
                // Ok(None)
                // }
                // egui::Event::Key {
                //     key: egui::Key::PageDown,
                //     pressed: true,
                //     modifiers: egui::Modifiers::NONE,
                //     ..
                // } => {
                // // We consumed the event.
                // Ok(None)
                // }
                event => {
                    // Event not handled here.
                    // We didn't consume the event, so return it.
                    Ok(Some(event))
                }
            }
        }
    }
}

impl<'a> EventHandlerT for sept::qv::Utf8StringTermLineElemCharView<'a> {
    fn handle_event(
        &self,
        event: egui::Event,
        event_handler_ctx: &mut EventHandlerCtx<'_>,
        cursor_address_token_i: &mut dyn std::iter::Iterator<Item = &sept::dy::Value>,
    ) -> Result<Option<egui::Event>> {
        if let Some(cursor_address_token) = cursor_address_token_i.next() {
            use sept::qv::SingleQueryT;
            match self.run_single_query(cursor_address_token)? {
                sept::qv::Utf8StringTermLineElemCharViewQuery::Utf8StringTermLineElemCharElemView(v) => {
                    v.handle_event(event, event_handler_ctx, cursor_address_token_i)
                }
            }
        } else {
            // This is the value addressed by the cursor.
            // TODO: Probably put this into a method in EventHandlerT.
            match event {
                event => {
                    // We didn't consume the event, so return it.
                    Ok(Some(event))
                }
            }
        }
    }
}

impl<'a> EventHandlerT for sept::qv::Utf8StringTermLineElemCharElemView<'a> {
    fn handle_event(
        &self,
        event: egui::Event,
        event_handler_ctx: &mut EventHandlerCtx<'_>,
        cursor_address_token_i: &mut dyn std::iter::Iterator<Item = &sept::dy::Value>,
    ) -> Result<Option<egui::Event>> {
        let cursor_len = event_handler_ctx.cursor_address.len() as u32;
        if let Some(cursor_address_token) = cursor_address_token_i.next() {
            use sept::qv::SingleQueryT;
            match self.run_single_query(cursor_address_token)? {}
        } else {
            // This is the value addressed by the cursor.
            // TODO: Probably put this into a method in EventHandlerT.

            match event {
                egui::Event::Key {
                    key: egui::Key::Enter,
                    pressed: true,
                    modifiers: egui::Modifiers::ALT,
                    ..
                }
                | egui::Event::Key {
                    key: egui::Key::Escape,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    // Fully escape line-char mode by taking off the last four cursor tokens.
                    event_handler_ctx.enqueue_command_cursor_address_pop(4);
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::Enter,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    // Root value edit
                    event_handler_ctx.enqueue_edit(RootValueEdit::from(AddressedEdit {
                        address: event_handler_ctx.cursor_address.clone(),
                        edit: sept::qv::InsertionTerm {
                            new_data: '\n'.into_value(),
                        }
                        .into_value(),
                    }));
                    // Cursor edit
                    {
                        // Cursor edit - Update the line_index
                        {
                            let old_line_index = self.line_index as u32;
                            let new_line_index = old_line_index.saturating_add(1);
                            event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                                // TODO: This could use `-3` as the address once negative indexing is supported.
                                address: vec![(cursor_len - 3).into_value()].into(),
                                edit: sept::qv::ReplacementTerm {
                                    old_data: old_line_index.into(),
                                    new_data: new_line_index.into(),
                                }
                                .into(),
                            }));
                        }
                        // Cursor edit - Update the char_index
                        {
                            let old_char_index = self.char_index as u32;
                            let new_char_index = {
                                let mut v = self.clone();
                                v.go_home();
                                v.char_index as u32
                            };
                            event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                                // TODO: This could use `-1` as the address once negative indexing is supported.
                                address: vec![(cursor_len - 1).into_value()].into(),
                                edit: sept::qv::ReplacementTerm {
                                    old_data: old_char_index.into(),
                                    new_data: new_char_index.into(),
                                }
                                .into(),
                            }));
                        }
                    }
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::Tab,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    // Root value edit
                    event_handler_ctx.enqueue_edit(RootValueEdit::from(AddressedEdit {
                        address: event_handler_ctx.cursor_address.clone(),
                        edit: sept::qv::InsertionTerm {
                            new_data: '\t'.into_value(),
                        }
                        .into_value(),
                    }));
                    // Cursor edit - Update the char_index
                    {
                        let old_char_index = self.char_index as u32;
                        let new_char_index = old_char_index.saturating_add(1);
                        event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                            // TODO: This could use `-1` as the address once negative indexing is supported.
                            address: vec![(cursor_len - 1).into_value()].into(),
                            edit: sept::qv::ReplacementTerm {
                                old_data: old_char_index.into(),
                                new_data: new_char_index.into(),
                            }
                            .into(),
                        }));
                    }
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::Delete,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    let cursor_char_o = {
                        use sept::qv::EvalT;
                        let cursor_value_la = self.eval().unwrap();
                        // This view will produce a char if the cursor is in bounds.
                        let cursor_char_o =
                            cursor_value_la.read().downcast_ref::<char>().map(|c| *c);
                        cursor_char_o
                    };
                    if let Some(cursor_char) = cursor_char_o {
                        event_handler_ctx.enqueue_edit(RootValueEdit::from(AddressedEdit {
                            address: event_handler_ctx.cursor_address.clone(),
                            edit: sept::qv::DeletionTerm {
                                old_data: cursor_char.into(),
                            }
                            .into(),
                        }));
                    }
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::Backspace,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    // Only backspace if we're not at the beginning of the string.
                    if self.line_index > 0 || self.char_index > 0 {
                        let mut v = self.clone();
                        v.increment_char_index_by(-1, true);
                        let cursor_char_o = {
                            use sept::qv::EvalT;
                            let cursor_value_la = v.eval().unwrap();
                            // This view will produce a char if the cursor is in bounds.
                            let cursor_char_o =
                                cursor_value_la.read().downcast_ref::<char>().map(|c| *c);
                            cursor_char_o
                        };
                        if let Some(cursor_char) = cursor_char_o {
                            // Only if there was a char to Backspace through should we enqueue commands
                            // to edit the root value and update the cursor.

                            // Compute the updated cursor_address and make the Root value edit.
                            {
                                let mut updated_cursor_address =
                                    event_handler_ctx.cursor_address.clone();
                                let cursor_len = updated_cursor_address.len();
                                assert!(cursor_len >= 4);
                                updated_cursor_address[cursor_len - 3] =
                                    (v.line_index as u32).into_value();
                                updated_cursor_address[cursor_len - 1] =
                                    (v.char_index as u32).into_value();

                                // Root value edit
                                event_handler_ctx.enqueue_edit(RootValueEdit::from(
                                    AddressedEdit {
                                        address: updated_cursor_address.clone(),
                                        edit: sept::qv::DeletionTerm {
                                            old_data: cursor_char.into(),
                                        }
                                        .into(),
                                    },
                                ));
                            }

                            enqueue_line_char_mode_cursor_edit(
                                (self.line_index as u32, self.char_index as u32),
                                (v.line_index as u32, v.char_index as u32),
                                event_handler_ctx,
                            );
                        }
                    }
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Paste(string) | egui::Event::Text(string) => {
                    // TEMP HACK -- insert char by char for now, support insertion of strings later.
                    let cursor_len = event_handler_ctx.cursor_address.len() as u32;
                    assert!(cursor_len >= 4);
                    // let mut cursor_address = event_handler_ctx.cursor_address.clone();

                    for c in string.chars() {
                        // Root value edit
                        event_handler_ctx.enqueue_edit(RootValueEdit::from(AddressedEdit {
                            address: event_handler_ctx.cursor_address.clone(),
                            edit: sept::qv::InsertionTerm { new_data: c.into() }.into(),
                        }));

                        // Cursor edit
                        if c == '\n' {
                            enqueue_line_char_mode_cursor_edit(
                                (self.line_index as u32, self.char_index as u32),
                                ((self.line_index as u32).saturating_add(1), 0u32),
                                event_handler_ctx,
                            );
                        } else {
                            enqueue_line_char_mode_cursor_edit(
                                (self.line_index as u32, self.char_index as u32),
                                (
                                    self.line_index as u32,
                                    (self.char_index as u32).saturating_add(1),
                                ),
                                event_handler_ctx,
                            );
                        }
                    }
                    // We consumed the event.
                    Ok(None)
                }
                // TODO: Handle copy/cut
                egui::Event::Key {
                    key: egui::Key::ArrowUp,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    let mut v = self.clone();
                    v.increment_line_index_by(-1);
                    enqueue_line_char_mode_cursor_edit(
                        (self.line_index as u32, self.char_index as u32),
                        (v.line_index as u32, v.char_index as u32),
                        event_handler_ctx,
                    );
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::ArrowDown,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    let mut v = self.clone();
                    v.increment_line_index_by(1);
                    enqueue_line_char_mode_cursor_edit(
                        (self.line_index as u32, self.char_index as u32),
                        (v.line_index as u32, v.char_index as u32),
                        event_handler_ctx,
                    );
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::ArrowLeft,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    let mut v = self.clone();
                    v.increment_char_index_by(-1, true);
                    enqueue_line_char_mode_cursor_edit(
                        (self.line_index as u32, self.char_index as u32),
                        (v.line_index as u32, v.char_index as u32),
                        event_handler_ctx,
                    );
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::ArrowRight,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    let mut v = self.clone();
                    v.increment_char_index_by(1, true);
                    enqueue_line_char_mode_cursor_edit(
                        (self.line_index as u32, self.char_index as u32),
                        (v.line_index as u32, v.char_index as u32),
                        event_handler_ctx,
                    );
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::Home,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    let mut v = self.clone();
                    v.go_home();
                    enqueue_line_char_mode_cursor_edit(
                        (self.line_index as u32, self.char_index as u32),
                        (v.line_index as u32, v.char_index as u32),
                        event_handler_ctx,
                    );
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::End,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    let mut v = self.clone();
                    v.go_end();
                    enqueue_line_char_mode_cursor_edit(
                        (self.line_index as u32, self.char_index as u32),
                        (v.line_index as u32, v.char_index as u32),
                        event_handler_ctx,
                    );
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::PageUp,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    let mut v = self.clone();
                    v.increment_line_index_by(-(event_handler_ctx.page_up_down_delta() as isize));
                    enqueue_line_char_mode_cursor_edit(
                        (self.line_index as u32, self.char_index as u32),
                        (v.line_index as u32, v.char_index as u32),
                        event_handler_ctx,
                    );
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::PageDown,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    let mut v = self.clone();
                    v.increment_line_index_by(event_handler_ctx.page_up_down_delta() as isize);
                    enqueue_line_char_mode_cursor_edit(
                        (self.line_index as u32, self.char_index as u32),
                        (v.line_index as u32, v.char_index as u32),
                        event_handler_ctx,
                    );
                    // We consumed the event.
                    Ok(None)
                }
                event => {
                    // We didn't consume the event, so return it.
                    Ok(Some(event))
                }
            }
        }
    }
}

enum EnterLineCharModeAt {
    Beginning,
    End,
}

fn enter_line_char_mode(
    string: &String,
    event_handler_ctx: &mut EventHandlerCtx,
    enter_line_char_mode_at: EnterLineCharModeAt,
) {
    let (line_index, char_index) = match enter_line_char_mode_at {
        EnterLineCharModeAt::Beginning => (0u32, 0u32),
        EnterLineCharModeAt::End => {
            let line_i = sept::st::split_inclusive_allow_trailing_empty(string, '\n');
            let line_count = line_i.clone().count();
            assert!(line_count > 0);
            let last_line = line_i.last().unwrap();
            let last_line_char_count = last_line.chars().count();

            let line_index = (line_count - 1) as u32;
            let char_index = last_line_char_count as u32;
            (line_index, char_index)
        }
    };

    // Enter the respective elem view by adding cursor tokens.
    let mut cursor_len = event_handler_ctx.cursor_address.len() as u32;
    event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
        address: vec![cursor_len.into_value()].into(),
        edit: sept::qv::InsertionTerm {
            new_data: "line".to_string().into(),
        }
        .into(),
    }));
    cursor_len += 1;
    event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
        address: vec![cursor_len.into_value()].into(),
        edit: sept::qv::InsertionTerm {
            new_data: line_index.into(),
        }
        .into(),
    }));
    cursor_len += 1;
    event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
        address: vec![cursor_len.into_value()].into(),
        edit: sept::qv::InsertionTerm {
            new_data: "char".to_string().into(),
        }
        .into(),
    }));
    cursor_len += 1;
    event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
        address: vec![cursor_len.into_value()].into(),
        edit: sept::qv::InsertionTerm {
            new_data: char_index.into(),
        }
        .into(),
    }));
}

fn enqueue_char_mode_cursor_edit(
    old_char_index: u32,
    new_char_index: u32,
    event_handler_ctx: &mut EventHandlerCtx,
) {
    let cursor_len = event_handler_ctx.cursor_address.len() as u32;
    assert!(cursor_len >= 2);
    // Update the char_index
    if new_char_index != old_char_index {
        event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
            // TODO: This could use `-1` as the address once negative indexing is supported.
            address: vec![(cursor_len - 1).into_value()].into(),
            edit: sept::qv::ReplacementTerm {
                old_data: old_char_index.into(),
                new_data: new_char_index.into(),
            }
            .into(),
        }));
    }
}

fn enqueue_line_mode_cursor_edit(
    old_line_index: u32,
    new_line_index: u32,
    event_handler_ctx: &mut EventHandlerCtx,
) {
    let cursor_len = event_handler_ctx.cursor_address.len() as u32;
    assert!(cursor_len >= 2);
    // Update the line_index
    if new_line_index != old_line_index {
        event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
            // TODO: This could use `-1` as the address once negative indexing is supported.
            address: vec![(cursor_len - 1).into_value()].into(),
            edit: sept::qv::ReplacementTerm {
                old_data: old_line_index.into(),
                new_data: new_line_index.into(),
            }
            .into(),
        }));
    }
}

fn enqueue_line_char_mode_cursor_edit(
    (old_line_index, old_char_index): (u32, u32),
    (new_line_index, new_char_index): (u32, u32),
    event_handler_ctx: &mut EventHandlerCtx,
) {
    let cursor_len = event_handler_ctx.cursor_address.len() as u32;
    assert!(cursor_len >= 4);
    // Update the line_index
    if new_line_index != old_line_index {
        event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
            // TODO: This could use `-3` as the address once negative indexing is supported.
            address: vec![(cursor_len - 3).into_value()].into(),
            edit: sept::qv::ReplacementTerm {
                old_data: old_line_index.into(),
                new_data: new_line_index.into(),
            }
            .into(),
        }));
    }
    // Update the char_index
    if new_char_index != old_char_index {
        event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
            // TODO: This could use `-1` as the address once negative indexing is supported.
            address: vec![(cursor_len - 1).into_value()].into(),
            edit: sept::qv::ReplacementTerm {
                old_data: old_char_index.into(),
                new_data: new_char_index.into(),
            }
            .into(),
        }));
    }
}

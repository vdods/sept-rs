use crate::{
    first_char_stripped_string, is_mouse_event, AddressedEdit, CursorEdit, EventHandlerCtx,
    EventHandlerT, RootValueEdit,
};
use anyhow::Result;
use sept::{
    dy::IntoValueT,
    qv::{self, ReplacementTerm},
};

impl EventHandlerT for sept::st::UnicodeCharTerm {
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
            // TODO: Just make sept::qv::UnicodeCharTermQuery impl EventHandlerT
            match self.run_single_query(cursor_address_token)? {
                sept::qv::UnicodeCharTermQuery::UnicodeCharTermPlainView(v) => {
                    v.handle_event(event, event_handler_ctx, cursor_address_token_i)
                }
                sept::qv::UnicodeCharTermQuery::UnicodeCharTermEscCView(v) => {
                    v.handle_event(event, event_handler_ctx, cursor_address_token_i)
                }
            }
        } else {
            // This is the value addressed by the cursor.
            if !is_mouse_event(&event) {
                tracing::trace!(
                    "UnicodeCharTerm::handle_event; event: {:?}, cursor: {:?}",
                    event,
                    event_handler_ctx.cursor_address
                );
            }
            // TODO: Probably put this into a method in EventHandlerT.
            match event {
                // egui::Event::Paste(string) => {
                // TODO: Create a replace or insert edit, depending on the mode.
                // }
                egui::Event::Key {
                    key: egui::Key::Enter,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    // First, detect which class the char is, then use that view.
                    let (address_char, char_index) =
                        if qv::UnicodeCharTermView::is_plain_char(*self) {
                            assert!(sept::qv::UnicodeCharTermPlainView::new(self).is_ok());
                            (
                                'p',
                                sept::qv::UnicodeCharTermPlainView::indexed_char_count() as u32,
                            )
                        } else if qv::UnicodeCharTermView::is_single_char_escape(*self) {
                            assert!(sept::qv::UnicodeCharTermEscCView::new(self).is_ok());
                            (
                                'c',
                                sept::qv::UnicodeCharTermEscCView::indexed_char_count() as u32,
                            )
                        } else {
                            unimplemented!(
                            "Only \"plain\" and single-char escape chars are supported currently"
                        );
                        };
                    // Enter the elem view at the end of the char rep by adding a cursor token.
                    // TODO: Could use -1 as the address once negative indexing is supported.
                    let mut cursor_len = event_handler_ctx.cursor_address.len() as u32;
                    event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                        address: vec![cursor_len.into_value()].into(),
                        edit: sept::qv::InsertionTerm {
                            new_data: address_char.into(),
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
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Text(string) if string.starts_with("'") => {
                    // TODO: Could use -1 as the address once negative indexing is supported.
                    let mut cursor_len = event_handler_ctx.cursor_address.len() as u32;
                    // Enter the appropriate elem view at its beginning.
                    // First, detect which class the char is, then use that view.
                    if qv::UnicodeCharTermView::is_plain_char(*self) {
                        event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                            address: vec![cursor_len.into_value()].into(),
                            edit: sept::qv::InsertionTerm {
                                new_data: 'p'.into(),
                            }
                            .into(),
                        }));
                        cursor_len += 1;
                    } else if qv::UnicodeCharTermView::is_single_char_escape(*self) {
                        event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                            address: vec![cursor_len.into_value()].into(),
                            edit: sept::qv::InsertionTerm {
                                new_data: 'c'.into(),
                            }
                            .into(),
                        }));
                        cursor_len += 1;
                    } else {
                        unimplemented!(
                            "Only \"plain\" and single-char escape chars are supported currently"
                        );
                    }
                    event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                        address: vec![cursor_len.into_value()].into(),
                        edit: sept::qv::InsertionTerm {
                            new_data: 0u32.into(),
                        }
                        .into(),
                    }));

                    // Take the used char off the front of the string and push the rest back onto the remaining events,
                    // if there's anything left of the string after the first char.
                    if let Some(remaining_string) = first_char_stripped_string(string) {
                        event_handler_ctx
                            .remaining_event_v
                            .push_front(egui::Event::Text(remaining_string));
                    }
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Text(string) => {
                    assert!(!string.is_empty());
                    // Take the first char off the string and create an edit with it.
                    let first_char = string.chars().next().unwrap();
                    event_handler_ctx.enqueue_edit(RootValueEdit::from(AddressedEdit {
                        address: event_handler_ctx.cursor_address.clone(),
                        edit: sept::qv::ReplacementTerm {
                            old_data: self.clone().into_value(),
                            new_data: first_char.into_value(),
                        }
                        .into_value(),
                    }));
                    // TODO: Should we advance the cursor here?

                    // Take the used char off the front of the string and push the rest back onto the remaining events,
                    // if there's anything left of the string after the first char.
                    if let Some(remaining_string) = first_char_stripped_string(string) {
                        event_handler_ctx
                            .remaining_event_v
                            .push_front(egui::Event::Text(remaining_string));
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

impl<'a> EventHandlerT for sept::qv::UnicodeCharTermPlainView<'a> {
    fn handle_event(
        &self,
        event: egui::Event,
        event_handler_ctx: &mut EventHandlerCtx<'_>,
        cursor_address_token_i: &mut dyn std::iter::Iterator<Item = &sept::dy::Value>,
    ) -> Result<Option<egui::Event>> {
        if let Some(cursor_address_token) = cursor_address_token_i.next() {
            use sept::qv::SingleQueryT;
            match self.run_single_query(cursor_address_token)? {
                qv::UnicodeCharTermPlainViewQuery::UnicodeCharTermPlainViewElemView(v) => {
                    v.handle_event(event, event_handler_ctx, cursor_address_token_i)
                }
            }
        } else {
            // This is the value addressed by the cursor.
            if !is_mouse_event(&event) {
                tracing::trace!(
                    "UnicodeCharTermPlainView::handle_event; event: {:?}, cursor: {:?}",
                    event,
                    event_handler_ctx.cursor_address
                );
            }
            // No event handling for now.
            Ok(None)
        }
    }
}

impl<'a> EventHandlerT for sept::qv::UnicodeCharTermPlainElemView<'a> {
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
            if !is_mouse_event(&event) {
                tracing::trace!(
                    "UnicodeCharTermPlainElemView::handle_event; event: {:?}, cursor: {:?}",
                    event,
                    event_handler_ctx.cursor_address
                );
            }
            // TODO: Probably put this into a method in EventHandlerT.

            // The address should currently have the suffix ('p', n) for some u32 in {0, 1}
            assert!(
                event_handler_ctx.cursor_address.len() >= 2,
                "programmer error"
            );

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
                    // Fully escape this view by taking off the last two cursor tokens.
                    event_handler_ctx.enqueue_edit_cursor_address_pop(2);
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::Enter,
                    pressed: true,
                    modifiers,
                    ..
                } if modifiers.command => {
                    // Fully escape this view by taking off the last two cursor tokens, and then
                    // attempting to advance the cursor using egui::Key::ArrowRight, which is a uniform
                    // way to advance the cursor by one element in all views.
                    event_handler_ctx.enqueue_edit_cursor_address_pop(2);
                    // Attempt to advance the cursor by one element.
                    event_handler_ctx
                        .remaining_event_v
                        .push_front(egui::Event::Key {
                            key: egui::Key::ArrowRight,
                            physical_key: Some(egui::Key::ArrowRight),
                            pressed: true,
                            repeat: false,
                            modifiers: egui::Modifiers::NONE,
                        });
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::Quote,
                    pressed: true,
                    modifiers,
                    ..
                } if modifiers == egui::Modifiers::NONE || modifiers == egui::Modifiers::SHIFT => {
                    // Consume this event and wait for the egui::Event::Text that will show up with a `'` or `"` char.
                    Ok(None)
                }
                egui::Event::Key {
                    key,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } if key == egui::Key::Enter || key == egui::Key::Tab => {
                    let key_char = match key {
                        egui::Key::Enter => '\n',
                        egui::Key::Tab => '\t',
                        _ => unreachable!("programmer error: you missed a case!"),
                    };
                    if self.char_index == 0 {
                        // We're editing the char itself.
                        let mut address = event_handler_ctx.cursor_address.clone();
                        // Pop the ('p', 0) suffix to get the address of the char itself;
                        address.pop().unwrap();
                        address.pop().unwrap();
                        // Replace this char with a plain char.
                        event_handler_ctx.enqueue_edit(RootValueEdit::from(AddressedEdit {
                            address,
                            edit: sept::qv::ReplacementTerm {
                                old_data: (*self.c).into_value(),
                                new_data: key_char.into_value(),
                            }
                            .into_value(),
                        }));
                        // Alter the cursor, because the class of view changed; change suffix to ('c', 2).
                        let cursor_len = event_handler_ctx.cursor_address.len() as u32;
                        event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                            // TODO: This could use `-2` as the address once negative indexing is supported.
                            address: vec![(cursor_len - 2).into_value()].into(),
                            edit: sept::qv::ReplacementTerm {
                                old_data: 'p'.into(),
                                new_data: 'c'.into(),
                            }
                            .into(),
                        }));
                        event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                            // TODO: This could use `-1` as the address once negative indexing is supported.
                            address: vec![(cursor_len - 1).into_value()].into(),
                            edit: sept::qv::ReplacementTerm {
                                old_data: 0u32.into(),
                                new_data: 2u32.into(),
                            }
                            .into(),
                        }));

                        // We consumed the event.
                        Ok(None)
                    } else {
                        // Cursor is after the char.

                        // We didn't consume the event, so return it.
                        Ok(Some(event))
                    }
                }
                egui::Event::Key {
                    key: egui::Key::Backspace,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    if self.char_index == 0 {
                        // Do nothing for now.  Could potentially delete this term from whatever it's contained by.
                        // We didn't consume the event, so return it.
                        Ok(Some(event))
                    } else {
                        // Don't delete anything, just move the cursor backwards so the char can be overwritten.
                        let cursor_len = event_handler_ctx.cursor_address.len() as u32;
                        event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                            address: vec![(cursor_len - 1).into_value()].into(),
                            edit: ReplacementTerm {
                                old_data: 1u32.into_value(),
                                new_data: 0u32.into_value(),
                            }
                            .into_value(),
                        }));
                        // We consumed the event.
                        Ok(None)
                    }
                }
                egui::Event::Paste(string) | egui::Event::Text(string) => {
                    // Only process the first char of the string.
                    let first_char = string.chars().next().unwrap();
                    if let Some(remaining_string) = first_char_stripped_string(string) {
                        event_handler_ctx
                            .remaining_event_v
                            .push_front(egui::Event::Text(remaining_string));
                    }
                    if self.char_index == 0 {
                        // We're editing the char itself.
                        let mut address = event_handler_ctx.cursor_address.clone();
                        // Pop the ('p', 0) suffix to get the address of the char itself;
                        address.pop().unwrap();
                        address.pop().unwrap();
                        // Replace this char with a plain char.
                        event_handler_ctx.enqueue_edit(RootValueEdit::from(AddressedEdit {
                            address,
                            edit: sept::qv::ReplacementTerm {
                                old_data: (*self.c).into_value(),
                                new_data: first_char.into_value(),
                            }
                            .into_value(),
                        }));

                        // Depending on the class of first_char, we may need to alter the view.
                        let cursor_len = event_handler_ctx.cursor_address.len() as u32;
                        if sept::qv::UnicodeCharTermView::is_plain_char(first_char) {
                            // Move the cursor up by one.
                            event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                                // TODO: This could use `-1` as the address once negative indexing is supported.
                                address: vec![(cursor_len - 1).into_value()].into(),
                                edit: sept::qv::ReplacementTerm {
                                    old_data: 0u32.into(),
                                    new_data: 1u32.into(),
                                }
                                .into(),
                            }));
                        } else if sept::qv::UnicodeCharTermView::is_single_char_escape(first_char) {
                            // Alter the cursor, because the class of view changed; change suffix to ('c', 2).
                            event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                                // TODO: This could use `-2` as the address once negative indexing is supported.
                                address: vec![(cursor_len - 2).into_value()].into(),
                                edit: sept::qv::ReplacementTerm {
                                    old_data: 'p'.into(),
                                    new_data: 'c'.into(),
                                }
                                .into(),
                            }));
                            if first_char == '\\' {
                                // If we typed a `\` then we should advance to the escape code element.
                                event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                                    // TODO: This could use `-1` as the address once negative indexing is supported.
                                    address: vec![(cursor_len - 1).into_value()].into(),
                                    edit: sept::qv::ReplacementTerm {
                                        old_data: 0u32.into(),
                                        new_data: 1u32.into(),
                                    }
                                    .into(),
                                }));
                            } else {
                                // Otherwise we should advance to the end of the view.
                                event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                                    // TODO: This could use `-1` as the address once negative indexing is supported.
                                    address: vec![(cursor_len - 1).into_value()].into(),
                                    edit: sept::qv::ReplacementTerm {
                                        old_data: 0u32.into(),
                                        new_data: 2u32.into(),
                                    }
                                    .into(),
                                }));
                            }
                        } else {
                            unimplemented!("Only \"plain\" and single-char escape chars are supported currently");
                        }
                        // We consumed the event.
                        Ok(None)
                    } else {
                        // Cursor is after the char, and only certain keys are handled in that case.
                        if first_char == '\'' {
                            // Fully escape this view by taking off the last two cursor tokens, and then
                            // attempting to advance the cursor using egui::Key::ArrowRight, which is a uniform
                            // way to advance the cursor by one element in all views.
                            event_handler_ctx.enqueue_edit_cursor_address_pop(2);
                            // Attempt to advance the cursor by one element.
                            event_handler_ctx
                                .remaining_event_v
                                .push_front(egui::Event::Key {
                                    key: egui::Key::ArrowRight,
                                    physical_key: Some(egui::Key::ArrowRight),
                                    pressed: true,
                                    repeat: false,
                                    modifiers: egui::Modifiers::NONE,
                                });
                            // We consumed the event.
                            Ok(None)
                        } else {
                            // TODO: Should this return the event?  The reason this is hacked is because `event` is partially moved.
                            // // We didn't consume the event, so return it.
                            // Ok(Some(event))

                            // We consumed the event.
                            Ok(None)
                        }
                    }
                }
                // TODO: Handle copy/cut
                egui::Event::Key {
                    key: egui::Key::ArrowLeft,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    let mut v = self.clone();
                    v.increment_char_index_by(-1);
                    enqueue_char_mode_cursor_edit(
                        self.char_index as u32,
                        v.char_index as u32,
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
                    v.increment_char_index_by(1);
                    enqueue_char_mode_cursor_edit(
                        self.char_index as u32,
                        v.char_index as u32,
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
                    enqueue_char_mode_cursor_edit(
                        self.char_index as u32,
                        v.char_index as u32,
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
                    enqueue_char_mode_cursor_edit(
                        self.char_index as u32,
                        v.char_index as u32,
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

impl<'a> EventHandlerT for sept::qv::UnicodeCharTermEscCView<'a> {
    fn handle_event(
        &self,
        event: egui::Event,
        event_handler_ctx: &mut EventHandlerCtx<'_>,
        cursor_address_token_i: &mut dyn std::iter::Iterator<Item = &sept::dy::Value>,
    ) -> Result<Option<egui::Event>> {
        if let Some(cursor_address_token) = cursor_address_token_i.next() {
            use sept::qv::SingleQueryT;
            match self.run_single_query(cursor_address_token)? {
                qv::UnicodeCharTermEscCViewQuery::UnicodeCharTermEscCElemView(v) => {
                    v.handle_event(event, event_handler_ctx, cursor_address_token_i)
                }
            }
        } else {
            // This is the value addressed by the cursor.
            if !is_mouse_event(&event) {
                tracing::trace!(
                    "UnicodeCharTermEscCView::handle_event; event: {:?}, cursor: {:?}",
                    event,
                    event_handler_ctx.cursor_address
                );
            }
            // No event handling for now.
            Ok(None)
        }
    }
}

impl<'a> EventHandlerT for sept::qv::UnicodeCharTermEscCElemView<'a> {
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
            if !is_mouse_event(&event) {
                tracing::trace!(
                    "UnicodeCharTermEscCElemView::handle_event; event: {:?}, cursor: {:?}",
                    event,
                    event_handler_ctx.cursor_address
                );
            }
            // TODO: Probably put this into a method in EventHandlerT.

            // The address should currently have the suffix ('c', n) for some u32 in {0, 1, 2}
            assert!(
                event_handler_ctx.cursor_address.len() >= 2,
                "programmer error"
            );

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
                    // Fully escape this view by taking off the last two cursor tokens.
                    event_handler_ctx.enqueue_edit_cursor_address_pop(2);
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::Enter,
                    pressed: true,
                    modifiers,
                    ..
                } if modifiers.command => {
                    // Fully escape this view by taking off the last two cursor tokens, and then
                    // attempting to advance the cursor using egui::Key::ArrowRight, which is a uniform
                    // way to advance the cursor by one element in all views.
                    event_handler_ctx.enqueue_edit_cursor_address_pop(2);
                    // Attempt to advance the cursor by one element.
                    event_handler_ctx
                        .remaining_event_v
                        .push_front(egui::Event::Key {
                            key: egui::Key::ArrowRight,
                            physical_key: Some(egui::Key::ArrowRight),
                            pressed: true,
                            repeat: false,
                            modifiers: egui::Modifiers::NONE,
                        });
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::Quote,
                    pressed: true,
                    modifiers,
                    ..
                } if modifiers == egui::Modifiers::NONE || modifiers == egui::Modifiers::SHIFT => {
                    // Consume this event and wait for the egui::Event::Text that will show up with a `'` or `"` char.
                    Ok(None)
                }
                egui::Event::Key {
                    key,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } if key == egui::Key::Enter || key == egui::Key::Tab => {
                    let key_char = match key {
                        egui::Key::Enter => '\n',
                        egui::Key::Tab => '\t',
                        _ => unreachable!("programmer error: you missed a case!"),
                    };
                    if self.char_index == 0 {
                        // We're editing the char itself.
                        let mut address = event_handler_ctx.cursor_address.clone();
                        // Pop the ('p', 0) suffix to get the address of the char itself;
                        address.pop().unwrap();
                        address.pop().unwrap();
                        // Replace this char with key_char.
                        event_handler_ctx.enqueue_edit(RootValueEdit::from(AddressedEdit {
                            address,
                            edit: sept::qv::ReplacementTerm {
                                old_data: (*self.c).into_value(),
                                new_data: key_char.into_value(),
                            }
                            .into_value(),
                        }));

                        // Depending on the class of key_char, we may need to alter the view.
                        let cursor_len = event_handler_ctx.cursor_address.len() as u32;
                        if sept::qv::UnicodeCharTermView::is_plain_char(key_char) {
                            // Alter the cursor, because the class of view changed; change suffix to ('p', 1).
                            event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                                // TODO: This could use `-2` as the address once negative indexing is supported.
                                address: vec![(cursor_len - 2).into_value()].into(),
                                edit: sept::qv::ReplacementTerm {
                                    old_data: 'c'.into(),
                                    new_data: 'p'.into(),
                                }
                                .into(),
                            }));
                            event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                                // TODO: This could use `-1` as the address once negative indexing is supported.
                                address: vec![(cursor_len - 1).into_value()].into(),
                                edit: sept::qv::ReplacementTerm {
                                    old_data: (self.char_index as u32).into(),
                                    new_data: 1u32.into(),
                                }
                                .into(),
                            }));
                        } else if sept::qv::UnicodeCharTermView::is_single_char_escape(key_char) {
                            // Move the cursor up by one.  TODO: Make this use UnicodeCharTermEscCView::increment_char_index_by
                            event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                                // TODO: This could use `-1` as the address once negative indexing is supported.
                                address: vec![(cursor_len - 1).into_value()].into(),
                                edit: sept::qv::ReplacementTerm {
                                    old_data: (self.char_index as u32).into(),
                                    new_data: 2u32.into(),
                                }
                                .into(),
                            }));
                        } else {
                            unimplemented!("Only \"plain\" and single-char escape chars are supported currently");
                        }

                        // We consumed the event.
                        Ok(None)
                    } else {
                        // Cursor is after the char.

                        // We didn't consume the event, so return it.
                        Ok(Some(event))
                    }
                }
                egui::Event::Key {
                    key: egui::Key::Backspace,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    if self.char_index == 0 {
                        // Do nothing for now.  Could potentially delete this term from whatever it's contained by.
                        // We didn't consume the event, so return it.
                        Ok(Some(event))
                    } else {
                        // Don't delete anything, just move the cursor backwards so the char can be overwritten.
                        let old_char_index = self.char_index as u32;
                        let new_char_index = {
                            let mut v = self.clone();
                            v.increment_char_index_by(-1);
                            v.char_index as u32
                        };
                        let cursor_len = event_handler_ctx.cursor_address.len() as u32;
                        event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                            address: vec![(cursor_len - 1).into_value()].into(),
                            edit: ReplacementTerm {
                                old_data: old_char_index.into_value(),
                                new_data: new_char_index.into_value(),
                            }
                            .into_value(),
                        }));
                        // We consumed the event.
                        Ok(None)
                    }
                }
                egui::Event::Paste(string) | egui::Event::Text(string) => {
                    // Only process the first char of the string.
                    let first_char = string.chars().next().unwrap();
                    if let Some(remaining_string) = first_char_stripped_string(string) {
                        event_handler_ctx
                            .remaining_event_v
                            .push_front(egui::Event::Text(remaining_string));
                    }
                    let cursor_len = event_handler_ctx.cursor_address.len() as u32;
                    if self.char_index == 0 {
                        // The cursor is over the `\`.  Typing any non-`\` char should replace the UnicodeCharTerm fully.
                        if first_char != '\\' {
                            let mut address = event_handler_ctx.cursor_address.clone();
                            // Pop the ('c', 0) suffix to get the address of the char itself;
                            address.pop().unwrap();
                            address.pop().unwrap();
                            // Replace this char with a plain char.
                            event_handler_ctx.enqueue_edit(RootValueEdit::from(AddressedEdit {
                                address,
                                edit: sept::qv::ReplacementTerm {
                                    old_data: (*self.c).into_value(),
                                    new_data: first_char.into_value(),
                                }
                                .into_value(),
                            }));

                            // Depending on the class of first_char, we may need to alter the view.
                            if sept::qv::UnicodeCharTermView::is_plain_char(first_char) {
                                // Alter the cursor, because the class of view changed; change suffix to ('p', 1).
                                event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                                    // TODO: This could use `-2` as the address once negative indexing is supported.
                                    address: vec![(cursor_len - 2).into_value()].into(),
                                    edit: sept::qv::ReplacementTerm {
                                        old_data: 'c'.into(),
                                        new_data: 'p'.into(),
                                    }
                                    .into(),
                                }));
                                event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                                    // TODO: This could use `-1` as the address once negative indexing is supported.
                                    address: vec![(cursor_len - 1).into_value()].into(),
                                    edit: sept::qv::ReplacementTerm {
                                        old_data: (self.char_index as u32).into(),
                                        new_data: 1u32.into(),
                                    }
                                    .into(),
                                }));
                            } else if sept::qv::UnicodeCharTermView::is_single_char_escape(
                                first_char,
                            ) {
                                // Move the cursor up by one.  TODO: Make this use UnicodeCharTermEscCView::increment_char_index_by
                                event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                                    // TODO: This could use `-1` as the address once negative indexing is supported.
                                    address: vec![(cursor_len - 1).into_value()].into(),
                                    edit: sept::qv::ReplacementTerm {
                                        old_data: (self.char_index as u32).into(),
                                        new_data: 1u32.into(),
                                    }
                                    .into(),
                                }));
                            } else {
                                unimplemented!("Only \"plain\" and single-char escape chars are supported currently");
                            }
                        } else {
                            // The char was `\`, so just advance the cursor.
                            event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                                // TODO: This could use `-1` as the address once negative indexing is supported.
                                address: vec![(cursor_len - 1).into_value()].into(),
                                edit: sept::qv::ReplacementTerm {
                                    old_data: 0u32.into(),
                                    new_data: 1u32.into(),
                                }
                                .into(),
                            }));
                        }
                        // We consumed the event.
                        Ok(None)
                    } else if self.char_index == 1 {
                        // The cursor is over the escape code.  So only accept input that is a valid escape code.
                        if let Some(unescaped_char) =
                            sept::qv::UnicodeCharTermView::unescape_single_char(first_char)
                        {
                            if unescaped_char != *self.c {
                                let mut address = event_handler_ctx.cursor_address.clone();
                                // Pop the ('c', 1) suffix to get the address of the char itself;
                                address.pop().unwrap();
                                address.pop().unwrap();
                                // Replace this char with a plain char.
                                event_handler_ctx.enqueue_edit(RootValueEdit::from(
                                    AddressedEdit {
                                        address,
                                        edit: sept::qv::ReplacementTerm {
                                            old_data: (*self.c).into_value(),
                                            new_data: unescaped_char.into_value(),
                                        }
                                        .into_value(),
                                    },
                                ));
                            }
                            // Advance the cursor.
                            event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                                // TODO: This could use `-1` as the address once negative indexing is supported.
                                address: vec![(cursor_len - 1).into_value()].into(),
                                edit: sept::qv::ReplacementTerm {
                                    old_data: 1u32.into(),
                                    new_data: 2u32.into(),
                                }
                                .into(),
                            }));
                            // We consumed the event.
                            Ok(None)
                        } else {
                            // TODO: Should this return the event?  The reason this is hacked is because `event` is partially moved.
                            // // We didn't consume the event, so return it.
                            // Ok(Some(event))

                            // We consumed the event.
                            Ok(None)
                        }
                    } else {
                        // Cursor is after the escape code, and only certain keys are handled in that case.
                        if first_char == '\'' {
                            // Fully escape this view by taking off the last two cursor tokens, and then
                            // attempting to advance the cursor using egui::Key::ArrowRight, which is a uniform
                            // way to advance the cursor by one element in all views.
                            event_handler_ctx.enqueue_edit_cursor_address_pop(2);
                            // Attempt to advance the cursor by one element.
                            event_handler_ctx
                                .remaining_event_v
                                .push_front(egui::Event::Key {
                                    key: egui::Key::ArrowRight,
                                    physical_key: Some(egui::Key::ArrowRight),
                                    pressed: true,
                                    repeat: false,
                                    modifiers: egui::Modifiers::NONE,
                                });
                            // We consumed the event.
                            Ok(None)
                        } else {
                            // TODO: Should this return the event?  The reason this is hacked is because `event` is partially moved.
                            // // We didn't consume the event, so return it.
                            // Ok(Some(event))

                            // We consumed the event.
                            Ok(None)
                        }
                    }
                }
                // TODO: Handle copy/cut
                egui::Event::Key {
                    key: egui::Key::ArrowLeft,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    let mut v = self.clone();
                    v.increment_char_index_by(-1);
                    enqueue_char_mode_cursor_edit(
                        self.char_index as u32,
                        v.char_index as u32,
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
                    v.increment_char_index_by(1);
                    enqueue_char_mode_cursor_edit(
                        self.char_index as u32,
                        v.char_index as u32,
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
                    enqueue_char_mode_cursor_edit(
                        self.char_index as u32,
                        v.char_index as u32,
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
                    enqueue_char_mode_cursor_edit(
                        self.char_index as u32,
                        v.char_index as u32,
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

// Note: This is duplicated from a private function in utf8_string_term_event_handler.rs
// Potentially should deduplicate.
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

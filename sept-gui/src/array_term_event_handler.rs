use crate::{
    placeholder_event_handler_impl, AddressedEdit, CursorEdit, Edit, EventHandlerCtx,
    EventHandlerT, LayoutDiscriminant, RootValueEdit,
};
use anyhow::Result;
use sept::dy::IntoValueT;

impl EventHandlerT for sept::dy::ArrayTerm {
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
                sept::qv::ArrayTermQuery::ArrayTermElemView(v) => {
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
                    // Enter the elem view by adding a cursor token.
                    let cursor_len = event_handler_ctx.cursor_address.len() as u32;
                    event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                        address: vec![cursor_len.into_value()].into(),
                        edit: sept::qv::InsertionTerm {
                            new_data: 0u32.into(),
                        }
                        .into(),
                    }));
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

impl<'a> EventHandlerT for sept::qv::ArrayTermElemView<'a> {
    fn handle_event(
        &self,
        event: egui::Event,
        event_handler_ctx: &mut EventHandlerCtx<'_>,
        cursor_address_token_i: &mut dyn std::iter::Iterator<Item = &sept::dy::Value>,
    ) -> Result<Option<egui::Event>> {
        let mut cursor_address_token_i = cursor_address_token_i.peekable();
        if cursor_address_token_i.peek().is_some() {
            match self.elem_o.as_deref() {
                Some(elem) => {
                    // Forward to element Value
                    elem.handle_event(event, event_handler_ctx, &mut cursor_address_token_i)
                }
                None => {
                    anyhow::bail!("ArrayTermElemView::handle_event; elem_o is None, can't forward event to it.");
                }
            }
        } else {
            let cursor_len = event_handler_ctx.cursor_address.len() as u32;
            assert!(cursor_len >= 1);
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
                    modifiers: egui::Modifiers::ALT,
                    ..
                }
                | egui::Event::Key {
                    key: egui::Key::Escape,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    // Escape this view by taking off the last cursor token.
                    {
                        let old_data = event_handler_ctx.cursor_address.last().unwrap().clone();
                        event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                            // TODO: This could use `-1` as the address once negative indexing is supported.
                            address: vec![(cursor_len - 1).into_value()].into(),
                            edit: sept::qv::DeletionTerm { old_data }.into(),
                        }));
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
                    // Cursor edit - Update the elem_index
                    {
                        let old_elem_index = self.elem_index as u32;
                        let new_elem_index = {
                            let mut v = self.clone();
                            v.go_home();
                            v.elem_index as u32
                        };
                        event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                            // TODO: This could use `-1` as the address once negative indexing is supported.
                            address: vec![(cursor_len - 1).into_value()].into(),
                            edit: sept::qv::ReplacementTerm {
                                old_data: old_elem_index.into(),
                                new_data: new_elem_index.into(),
                            }
                            .into(),
                        }));
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
                    // Cursor edit - Update the elem_index
                    {
                        let old_elem_index = self.elem_index as u32;
                        let new_elem_index = {
                            let mut v = self.clone();
                            v.go_end();
                            v.elem_index as u32
                        };
                        event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                            // TODO: This could use `-1` as the address once negative indexing is supported.
                            address: vec![(cursor_len - 1).into_value()].into(),
                            edit: sept::qv::ReplacementTerm {
                                old_data: old_elem_index.into(),
                                new_data: new_elem_index.into(),
                            }
                            .into(),
                        }));
                    }

                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::ArrowUp,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    // if event_handler_ctx.layout_mode() == LayoutMode::Expanded {
                    if event_handler_ctx.layout_discriminant()
                        != LayoutDiscriminant::InteriorLevelInline
                    {
                        let mut v = self.clone();
                        let old_elem_index = v.elem_index as u32;
                        v.increment_elem_index_by(-1);
                        event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                            // TODO: This could use `-1` as the address once negative indexing is supported.
                            address: vec![(cursor_len - 1).into_value()].into(),
                            edit: sept::qv::ReplacementTerm {
                                old_data: old_elem_index.into(),
                                new_data: (v.elem_index as u32).into(),
                            }
                            .into(),
                        }));
                        // We consumed the event.
                        Ok(None)
                    } else {
                        // We didn't consume the event, so return it.
                        Ok(Some(event))
                    }
                }
                egui::Event::Key {
                    key: egui::Key::ArrowDown,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    // if event_handler_ctx.layout_mode() == LayoutMode::Expanded {
                    if event_handler_ctx.layout_discriminant()
                        != LayoutDiscriminant::InteriorLevelInline
                    {
                        let mut v = self.clone();
                        let old_elem_index = v.elem_index as u32;
                        v.increment_elem_index_by(1);
                        event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                            // TODO: This could use `-1` as the address once negative indexing is supported.
                            address: vec![(cursor_len - 1).into_value()].into(),
                            edit: sept::qv::ReplacementTerm {
                                old_data: old_elem_index.into(),
                                new_data: (v.elem_index as u32).into(),
                            }
                            .into(),
                        }));
                        // We consumed the event.
                        Ok(None)
                    } else {
                        // We didn't consume the event, so return it.
                        Ok(Some(event))
                    }
                }
                egui::Event::Key {
                    key: egui::Key::ArrowLeft,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    // if event_handler_ctx.layout_mode() == LayoutMode::Inline {
                    if event_handler_ctx.layout_discriminant()
                        == LayoutDiscriminant::InteriorLevelInline
                    {
                        let mut v = self.clone();
                        let old_elem_index = v.elem_index as u32;
                        v.increment_elem_index_by(-1);
                        event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                            // TODO: This could use `-1` as the address once negative indexing is supported.
                            address: vec![(cursor_len - 1).into_value()].into(),
                            edit: sept::qv::ReplacementTerm {
                                old_data: old_elem_index.into(),
                                new_data: (v.elem_index as u32).into(),
                            }
                            .into(),
                        }));
                        // We consumed the event.
                        Ok(None)
                    } else {
                        // We didn't consume the event, so return it.
                        Ok(Some(event))
                    }
                }
                egui::Event::Key {
                    key: egui::Key::ArrowRight,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    // if event_handler_ctx.layout_mode() == LayoutMode::Inline {
                    if event_handler_ctx.layout_discriminant()
                        == LayoutDiscriminant::InteriorLevelInline
                    {
                        let mut v = self.clone();
                        let old_elem_index = v.elem_index as u32;
                        v.increment_elem_index_by(1);
                        event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                            // TODO: This could use `-1` as the address once negative indexing is supported.
                            address: vec![(cursor_len - 1).into_value()].into(),
                            edit: sept::qv::ReplacementTerm {
                                old_data: old_elem_index.into(),
                                new_data: (v.elem_index as u32).into(),
                            }
                            .into(),
                        }));
                        // We consumed the event.
                        Ok(None)
                    } else {
                        // We didn't consume the event, so return it.
                        Ok(Some(event))
                    }
                }
                egui::Event::Key {
                    key: egui::Key::Delete,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    // Only delete if we're not at the end of the array.
                    if self.elem_index < self.array_term.len() {
                        let cursor_value = {
                            use sept::qv::QueryableDynT;
                            let query_b = event_handler_ctx
                                .root_value
                                .make_and_run_query(&mut event_handler_ctx.cursor_address.iter())
                                .unwrap();
                            query_b.eval().unwrap().to_owned()
                        };
                        event_handler_ctx.enqueue_edit(RootValueEdit::from(AddressedEdit {
                            address: event_handler_ctx.cursor_address.clone(),
                            edit: sept::qv::DeletionTerm {
                                old_data: cursor_value.into(),
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
                    // Only backspace if we're not at the beginning of the array.
                    if self.elem_index > 0 {
                        let mut v = self.clone();
                        v.increment_elem_index_by(-1);
                        let cursor_elem = {
                            use sept::qv::EvalT;
                            v.eval().unwrap().to_owned()
                        };

                        // Compute the updated cursor_address
                        let mut updated_cursor_address = event_handler_ctx.cursor_address.clone();
                        let cursor_len = updated_cursor_address.len();
                        assert!(cursor_len >= 1);
                        updated_cursor_address[cursor_len - 1] = (v.elem_index as u32).into_value();

                        // Root value edit
                        event_handler_ctx.enqueue_edit(RootValueEdit::from(AddressedEdit {
                            address: updated_cursor_address.clone(),
                            edit: sept::qv::DeletionTerm {
                                old_data: cursor_elem.into(),
                            }
                            .into(),
                        }));

                        // Cursor edit
                        {
                            event_handler_ctx.enqueue_edit(CursorEdit::from(AddressedEdit {
                                // TODO: This could use `-1` as the address once negative indexing is supported.
                                address: vec![((cursor_len - 1) as u32).into_value()].into(),
                                edit: sept::qv::ReplacementTerm {
                                    old_data: event_handler_ctx.cursor_address[cursor_len - 1]
                                        .clone(),
                                    new_data: updated_cursor_address[cursor_len - 1].clone(),
                                }
                                .into(),
                            }));
                        }
                    }
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::Insert,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                    ..
                } => {
                    // Insert a Placeholder at the current cursor position, but don't update the cursor.
                    event_handler_ctx.enqueue_edit(RootValueEdit::from(AddressedEdit {
                        address: event_handler_ctx.cursor_address.clone(),
                        edit: sept::qv::InsertionTerm {
                            new_data: sept::st::Placeholder.into(),
                        }
                        .into(),
                    }));
                    // We consumed the event.
                    Ok(None)
                }
                event => {
                    // We didn't use the event; forward it to the array element under the cursor.
                    match self.elem_o.as_deref() {
                        Some(elem) => {
                            elem.handle_event(event, event_handler_ctx, &mut cursor_address_token_i)
                        }
                        None => placeholder_event_handler_impl(
                            event,
                            event_handler_ctx,
                            |address: sept::dy::TupleTerm, new_data: sept::dy::Value| -> Edit {
                                RootValueEdit::from(AddressedEdit {
                                    address,
                                    edit: sept::qv::InsertionTerm { new_data }.into(),
                                })
                                .into()
                            },
                        ),
                    }
                }
            }
        }
    }
}

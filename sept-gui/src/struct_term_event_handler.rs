use crate::{
    placeholder_event_handler_impl, AddressedEdit, Command, CursorEdit, EventHandler,
    EventHandlerCtx, LayoutDiscriminant, RootValueEdit,
};
use anyhow::Result;
use sept::dy::IntoValue;

impl EventHandler for sept::dy::StructTerm {
    fn handle_event(
        &self,
        event: egui::Event,
        event_handler_ctx: &mut EventHandlerCtx<'_>,
        cursor_address_token_i: &mut dyn std::iter::Iterator<Item = &sept::dy::Value>,
    ) -> Result<Option<egui::Event>> {
        if let Some(cursor_address_token) = cursor_address_token_i.next() {
            let mut event_handler_ctx_g = event_handler_ctx.push_nesting_depth();
            let event_handler_ctx = &mut event_handler_ctx_g;
            use sept::qv::SingleQuery;
            match self.run_single_query(cursor_address_token)? {
                sept::qv::StructTermQuery::StructTermFieldElemView(v) => {
                    v.handle_event(event, event_handler_ctx, cursor_address_token_i)
                }
            }
        } else {
            // This is the value addressed by the cursor.
            // TODO: Probably put this into a method in the EventHandler trait.
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
                } => {
                    // Enter the field elem elem view at the name component of the last field
                    // by adding two cursor tokens.
                    let cursor_len = event_handler_ctx.cursor_address.len() as u32;
                    event_handler_ctx.enqueue_command(CursorEdit::from(AddressedEdit {
                        address: vec![cursor_len.into_value()].into(),
                        edit: sept::qv::InsertionTerm {
                            new_data: (self.len() as u32).into(),
                        }
                        .into(),
                    }));
                    event_handler_ctx.enqueue_command(CursorEdit::from(AddressedEdit {
                        address: vec![(cursor_len + 1).into_value()].into(),
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

impl<'a> EventHandler for sept::qv::StructTermFieldElemView<'a> {
    fn handle_event(
        &self,
        event: egui::Event,
        event_handler_ctx: &mut EventHandlerCtx<'_>,
        cursor_address_token_i: &mut dyn std::iter::Iterator<Item = &sept::dy::Value>,
    ) -> Result<Option<egui::Event>> {
        if let Some(cursor_address_token) = cursor_address_token_i.next() {
            use sept::qv::SingleQuery;
            self.run_single_query(cursor_address_token)?.handle_event(
                event,
                event_handler_ctx,
                cursor_address_token_i,
            )
        } else {
            // This is the value addressed by the cursor.
            // TODO: Probably put this into a method in the EventHandler trait.
            let cursor_len = event_handler_ctx.cursor_address.len() as u32;
            assert!(cursor_len >= 1);
            match event {
                egui::Event::Key {
                    key: egui::Key::Enter,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                } => {
                    // Enter this StructTerm field at element 0.
                    event_handler_ctx
                        .enqueue_command_cursor_address_push([0u32.into()].into_iter());
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::Enter,
                    pressed: true,
                    modifiers: egui::Modifiers::ALT,
                }
                | egui::Event::Key {
                    key: egui::Key::Escape,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                } => {
                    // Escape this view by taking off the last cursor token.
                    event_handler_ctx.enqueue_command_cursor_address_pop(1);
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::Delete,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                } => {
                    // Only delete if we're not at the end of the StructTerm.
                    if self.field_index < self.struct_term.len() {
                        let cursor_value = {
                            use sept::qv::QueryableDynTrait;
                            let query_b = event_handler_ctx
                                .root_value
                                .make_and_run_query(&mut event_handler_ctx.cursor_address.iter())
                                .unwrap();
                            query_b.eval().unwrap().to_owned()
                        };
                        event_handler_ctx.enqueue_command(RootValueEdit::from(AddressedEdit {
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
                } => {
                    // Only backspace if we're not at the beginning of the array.
                    if self.field_index > 0 {
                        let mut v = self.clone();
                        v.increment_field_index_by(-1);
                        let cursor_field = {
                            use sept::qv::EvalTrait;
                            v.eval().unwrap().to_owned()
                        };

                        // Compute the updated cursor_address
                        let mut updated_cursor_address = event_handler_ctx.cursor_address.clone();
                        let cursor_len = updated_cursor_address.len();
                        assert!(cursor_len >= 1);
                        updated_cursor_address[cursor_len - 1] =
                            (v.field_index as u32).into_value();

                        // Root value edit
                        event_handler_ctx.enqueue_command(RootValueEdit::from(AddressedEdit {
                            address: updated_cursor_address.clone(),
                            edit: sept::qv::DeletionTerm {
                                old_data: cursor_field.into(),
                            }
                            .into(),
                        }));

                        // Cursor edit
                        {
                            event_handler_ctx.enqueue_command(CursorEdit::from(AddressedEdit {
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
                    key: egui::Key::Home,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                } => {
                    // Cursor edit - Update the elem_index
                    {
                        let mut v = self.clone();
                        v.go_home();
                        event_handler_ctx.enqueue_command(CursorEdit::from(AddressedEdit {
                            // TODO: This could use `-1` as the address once negative indexing is supported.
                            address: vec![(cursor_len - 1).into_value()].into(),
                            edit: sept::qv::ReplacementTerm {
                                old_data: (self.field_index as u32).into(),
                                new_data: (v.field_index as u32).into(),
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
                } => {
                    // Cursor edit - Update the field_index
                    {
                        let mut v = self.clone();
                        v.go_end();
                        event_handler_ctx.enqueue_command(CursorEdit::from(AddressedEdit {
                            // TODO: This could use `-1` as the address once negative indexing is supported.
                            address: vec![(cursor_len - 1).into_value()].into(),
                            edit: sept::qv::ReplacementTerm {
                                old_data: (self.field_index as u32).into(),
                                new_data: (v.field_index as u32).into(),
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
                } => {
                    // if event_handler_ctx.layout_mode() == LayoutMode::Expanded {
                    if event_handler_ctx.layout_discriminant()
                        != LayoutDiscriminant::InteriorLevelInline
                    {
                        let mut v = self.clone();
                        v.increment_field_index_by(-1);
                        event_handler_ctx.enqueue_command(CursorEdit::from(AddressedEdit {
                            // TODO: This could use `-1` as the address once negative indexing is supported.
                            address: vec![(cursor_len - 1).into_value()].into(),
                            edit: sept::qv::ReplacementTerm {
                                old_data: (self.field_index as u32).into(),
                                new_data: (v.field_index as u32).into(),
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
                } => {
                    // if event_handler_ctx.layout_mode() == LayoutMode::Expanded {
                    if event_handler_ctx.layout_discriminant()
                        != LayoutDiscriminant::InteriorLevelInline
                    {
                        let mut v = self.clone();
                        v.increment_field_index_by(1);
                        event_handler_ctx.enqueue_command(CursorEdit::from(AddressedEdit {
                            // TODO: This could use `-1` as the address once negative indexing is supported.
                            address: vec![(cursor_len - 1).into_value()].into(),
                            edit: sept::qv::ReplacementTerm {
                                old_data: (self.field_index as u32).into(),
                                new_data: (v.field_index as u32).into(),
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
                } => {
                    // if event_handler_ctx.layout_mode() == LayoutMode::Inline {
                    if event_handler_ctx.layout_discriminant()
                        == LayoutDiscriminant::InteriorLevelInline
                    {
                        let mut v = self.clone();
                        v.increment_field_index_by(-1);
                        event_handler_ctx.enqueue_command(CursorEdit::from(AddressedEdit {
                            // TODO: This could use `-1` as the address once negative indexing is supported.
                            address: vec![(cursor_len - 1).into_value()].into(),
                            edit: sept::qv::ReplacementTerm {
                                old_data: (self.field_index as u32).into(),
                                new_data: (v.field_index as u32).into(),
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
                } => {
                    // if event_handler_ctx.layout_mode() == LayoutMode::Inline {
                    if event_handler_ctx.layout_discriminant()
                        == LayoutDiscriminant::InteriorLevelInline
                    {
                        let mut v = self.clone();
                        v.increment_field_index_by(1);
                        event_handler_ctx.enqueue_command(CursorEdit::from(AddressedEdit {
                            // TODO: This could use `-1` as the address once negative indexing is supported.
                            address: vec![(cursor_len - 1).into_value()].into(),
                            edit: sept::qv::ReplacementTerm {
                                old_data: (self.field_index as u32).into(),
                                new_data: (v.field_index as u32).into(),
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
                event => {
                    // We didn't consume the event, so return it.
                    Ok(Some(event))
                }
            }
        }
    }
}

impl<'a> EventHandler for sept::qv::StructTermFieldElemElemView<'a> {
    fn handle_event(
        &self,
        event: egui::Event,
        event_handler_ctx: &mut EventHandlerCtx<'_>,
        cursor_address_token_i: &mut dyn std::iter::Iterator<Item = &sept::dy::Value>,
    ) -> Result<Option<egui::Event>> {
        let mut cursor_address_token_i = cursor_address_token_i.peekable();
        if cursor_address_token_i.peek().is_some() {
            assert!(self.sub_index <= 1);
            if !self.is_at_end_field_placeholder() {
                if self.sub_index == 0 {
                    self.struct_term
                        .get_field_name(self.field_index)
                        .unwrap()
                        .handle_event(event, event_handler_ctx, &mut cursor_address_token_i)
                } else {
                    self.struct_term
                        .get_field_type(self.field_index)
                        .unwrap()
                        .handle_event(event, event_handler_ctx, &mut cursor_address_token_i)
                }
            } else {
                anyhow::bail!("StructTermFieldElemElemView doesn't support further queries after the last field.")
            }
        } else {
            // This is the value addressed by the cursor.
            // TODO: Probably put this into a method in the EventHandler trait.
            let cursor_len = event_handler_ctx.cursor_address.len() as u32;
            assert!(cursor_len >= 2);
            match event {
                egui::Event::Key {
                    key: egui::Key::Enter,
                    pressed: true,
                    modifiers: egui::Modifiers::ALT,
                }
                | egui::Event::Key {
                    key: egui::Key::Escape,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                } => {
                    // Escape this view by taking off the last cursor token.
                    event_handler_ctx.enqueue_command_cursor_address_pop(1);
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::Home,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                } => {
                    // Cursor edit - Update the sub_index
                    {
                        let mut v = self.clone();
                        v.go_home();
                        event_handler_ctx.enqueue_command(CursorEdit::from(AddressedEdit {
                            // TODO: This could use `-1` as the address once negative indexing is supported.
                            address: vec![(cursor_len - 1).into_value()].into(),
                            edit: sept::qv::ReplacementTerm {
                                old_data: (self.sub_index as u32).into(),
                                new_data: (v.sub_index as u32).into(),
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
                } => {
                    // Cursor edit - Update the sub_index
                    {
                        let mut v = self.clone();
                        v.go_end();
                        event_handler_ctx.enqueue_command(CursorEdit::from(AddressedEdit {
                            // TODO: This could use `-1` as the address once negative indexing is supported.
                            address: vec![(cursor_len - 1).into_value()].into(),
                            edit: sept::qv::ReplacementTerm {
                                old_data: (self.sub_index as u32).into(),
                                new_data: (v.sub_index as u32).into(),
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
                } => {
                    let mut v = self.clone();
                    v.increment_field_index_by(-1);
                    event_handler_ctx.enqueue_command(CursorEdit::from(AddressedEdit {
                        // TODO: This could use `-2` as the address once negative indexing is supported.
                        address: vec![(cursor_len - 2).into_value()].into(),
                        edit: sept::qv::ReplacementTerm {
                            old_data: (self.field_index as u32).into(),
                            new_data: (v.field_index as u32).into(),
                        }
                        .into(),
                    }));
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::ArrowDown,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                } => {
                    let mut v = self.clone();
                    // let old_field_index = v.field_index as u32;
                    v.increment_field_index_by(1);
                    event_handler_ctx.enqueue_command(CursorEdit::from(AddressedEdit {
                        // TODO: This could use `-2` as the address once negative indexing is supported.
                        address: vec![(cursor_len - 2).into_value()].into(),
                        edit: sept::qv::ReplacementTerm {
                            old_data: (self.field_index as u32).into(),
                            new_data: (v.field_index as u32).into(),
                        }
                        .into(),
                    }));
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::ArrowLeft,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                } => {
                    let mut v = self.clone();
                    v.increment_sub_index_by(-1, true);
                    event_handler_ctx.enqueue_command(CursorEdit::from(AddressedEdit {
                        // TODO: This could use `-2` as the address once negative indexing is supported.
                        address: vec![(cursor_len - 2).into_value()].into(),
                        edit: sept::qv::ReplacementTerm {
                            old_data: (self.field_index as u32).into(),
                            new_data: (v.field_index as u32).into(),
                        }
                        .into(),
                    }));
                    event_handler_ctx.enqueue_command(CursorEdit::from(AddressedEdit {
                        // TODO: This could use `-1` as the address once negative indexing is supported.
                        address: vec![(cursor_len - 1).into_value()].into(),
                        edit: sept::qv::ReplacementTerm {
                            old_data: (self.sub_index as u32).into(),
                            new_data: (v.sub_index as u32).into(),
                        }
                        .into(),
                    }));
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::ArrowRight,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                } => {
                    let mut v = self.clone();
                    v.increment_sub_index_by(1, true);
                    event_handler_ctx.enqueue_command(CursorEdit::from(AddressedEdit {
                        // TODO: This could use `-2` as the address once negative indexing is supported.
                        address: vec![(cursor_len - 2).into_value()].into(),
                        edit: sept::qv::ReplacementTerm {
                            old_data: (self.field_index as u32).into(),
                            new_data: (v.field_index as u32).into(),
                        }
                        .into(),
                    }));
                    event_handler_ctx.enqueue_command(CursorEdit::from(AddressedEdit {
                        // TODO: This could use `-1` as the address once negative indexing is supported.
                        address: vec![(cursor_len - 1).into_value()].into(),
                        edit: sept::qv::ReplacementTerm {
                            old_data: (self.sub_index as u32).into(),
                            new_data: (v.sub_index as u32).into(),
                        }
                        .into(),
                    }));
                    // We consumed the event.
                    Ok(None)
                }
                event => {
                    assert!(self.sub_index <= 1);
                    if !self.is_at_end_field_placeholder() {
                        // We didn't use the event; forward it to the value under the cursor.
                        if self.sub_index == 0 {
                            self.struct_term
                                .get_field_name(self.field_index)
                                .unwrap()
                                .handle_event(event, event_handler_ctx, &mut cursor_address_token_i)
                        } else {
                            self.struct_term
                                .get_field_type(self.field_index)
                                .unwrap()
                                .handle_event(event, event_handler_ctx, &mut cursor_address_token_i)
                        }
                    } else {
                        // We're at the placeholder after the last field.  Handle creation of new fields.
                        if self.sub_index == 0 {
                            placeholder_event_handler_impl(
                                event,
                                event_handler_ctx,
                                |mut address: sept::dy::TupleTerm,
                                 new_data: sept::dy::Value|
                                 -> Command {
                                    // Insert the struct field `X: Term` at the end of the struct,
                                    // where X is the new_data determined by placeholder_event_handler_impl.
                                    // The address is of the field elem elem, so we have to pop one
                                    // to get the field elem.  `Term` is used because that is the least-
                                    // constrained possible type (no constraints).
                                    assert!(address.len() >= 2);
                                    address.pop().unwrap();
                                    RootValueEdit::from(AddressedEdit {
                                        address,
                                        edit: sept::qv::InsertionTerm {
                                            new_data: sept::dy::TupleTerm::from(vec![
                                                new_data,
                                                sept::st::Term.into(),
                                            ])
                                            .into(),
                                        }
                                        .into(),
                                    })
                                    .into()
                                },
                            )
                        } else {
                            // unimplemented!("todo");

                            // placeholder_event_handler_impl(
                            //     event,
                            //     event_handler_ctx,
                            //     |address: sept::dy::TupleTerm,
                            //      new_data: sept::dy::Value|
                            //      -> Command {
                            //         // Insert the struct field `X: Placeholder` at the end of the struct,
                            //         // where X is the new_data determined by placeholder_event_handler_impl.
                            //         // The address is of the field elem elem, so we have to pop one
                            //         // to get the field elem.
                            //         assert!(address.len() >= 2);
                            //         address.pop().unwrap();
                            //         RootValueEdit::from(AddressedEdit {
                            //             address,
                            //             edit: sept::qv::InsertionTerm {
                            //                 new_data: sept::dy::TupleTerm::from(vec![
                            //                     new_data,
                            //                     sept::st::Placeholder.into(),
                            //                 ])
                            //                 .into(),
                            //             },
                            //         })
                            //     },
                            // )

                            // We didn't consume the event, so return it.
                            Ok(Some(event))
                        }
                    }
                }
            }
        }
    }
}

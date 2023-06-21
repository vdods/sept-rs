use crate::{AddressedEdit, Command, EventHandler, EventHandlerCtx};
use anyhow::Result;
use sept::dy::IntoValue;

impl EventHandler for sept::st::Utf8StringTerm {
    fn handle_event(
        &self,
        event: egui::Event,
        event_handler_ctx: &mut EventHandlerCtx<'_>,
        cursor_address_token_i: &mut dyn std::iter::Iterator<Item = &sept::dy::Value>,
    ) -> Result<Option<egui::Event>> {
        // tracing::debug!("Utf8StringTerm::handle_event; event: {:?}", event);
        if let Some(cursor_address_token) = cursor_address_token_i.next() {
            let mut event_handler_ctx_g = event_handler_ctx.push_nesting_depth();
            let event_handler_ctx = &mut event_handler_ctx_g;
            use sept::qv::SingleQuery;
            // TODO: Just make sept::qv::Utf8StringTermQuery impl EventHandler
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
                    // // Which mode ("line" vs "char") to enter depends on which LayoutMode we're in.
                    // let mode = match event_handler_ctx.layout_mode() {
                    //     LayoutMode::Expanded => "line",
                    //     LayoutMode::Inline => "char",
                    // };
                    // // Enter the respective elem view by adding cursor tokens.
                    // let mut cursor_len = event_handler_ctx.cursor_address.len() as u32;
                    // event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
                    //     address: vec![cursor_len.into_value()].into(),
                    //     edit: sept::qv::InsertionTerm {
                    //         new_data: mode.to_string().into(),
                    //     }
                    //     .into(),
                    // }));
                    // cursor_len += 1;
                    // event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
                    //     address: vec![cursor_len.into_value()].into(),
                    //     edit: sept::qv::InsertionTerm {
                    //         new_data: 0u32.into(),
                    //     }
                    //     .into(),
                    // }));
                    // // We consumed the event.
                    // Ok(None)

                    // Unconditionally go into "line-char" mode.

                    // Enter the respective elem view by adding cursor tokens.
                    let mut cursor_len = event_handler_ctx.cursor_address.len() as u32;
                    event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
                        address: vec![cursor_len.into_value()].into(),
                        edit: sept::qv::InsertionTerm {
                            new_data: "line".to_string().into(),
                        }
                        .into(),
                    }));
                    cursor_len += 1;
                    event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
                        address: vec![cursor_len.into_value()].into(),
                        edit: sept::qv::InsertionTerm {
                            new_data: 0u32.into(),
                        }
                        .into(),
                    }));
                    cursor_len += 1;
                    event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
                        address: vec![cursor_len.into_value()].into(),
                        edit: sept::qv::InsertionTerm {
                            new_data: "char".to_string().into(),
                        }
                        .into(),
                    }));
                    cursor_len += 1;
                    event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
                        address: vec![cursor_len.into_value()].into(),
                        edit: sept::qv::InsertionTerm {
                            new_data: 0u32.into(),
                        }
                        .into(),
                    }));
                    // We consumed the event.
                    Ok(None)
                } //     egui::Event::Text(mut string) if string.as_str().starts_with('c') => {
                //         // This block is handling the messiness of egui's event system.
                //         {
                //             // Filter out egui::Event::Key C (since it will be redundant with this Text event)
                //             event_handler_ctx.remaining_event_v.retain(|event| {
                //                 !matches!(
                //                     event,
                //                     egui::Event::Key {
                //                         key: egui::Key::C,
                //                         pressed: true,
                //                         modifiers: egui::Modifiers::NONE
                //                     }
                //                 )
                //             });
                //             // Take off the 'c' char and push the remaining Text string.
                //             string.remove(0);
                //             event_handler_ctx
                //                 .remaining_event_v
                //                 .push_front(egui::Event::Text(string));
                //         }

                //         // Enter the "char" elem view by adding cursor tokens.
                //         let mut cursor_len = event_handler_ctx.cursor_address.len() as u32;
                //         event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
                //             address: vec![cursor_len.into_value()].into(),
                //             edit: sept::qv::InsertionTerm {
                //                 new_data: "char".to_string().into(),
                //             }
                //             .into(),
                //         }));
                //         cursor_len += 1;
                //         event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
                //             address: vec![cursor_len.into_value()].into(),
                //             edit: sept::qv::InsertionTerm {
                //                 new_data: 0u32.into(),
                //             }
                //             .into(),
                //         }));
                //         // We consumed the event.
                //         Ok(None)
                //     }
                //     egui::Event::Text(mut string) if string.as_str().starts_with('l') => {
                //         // This block is handling the messiness of egui's event system.
                //         {
                //             // Filter out egui::Event::Key L (since it will be redundant with this Text event)
                //             event_handler_ctx.remaining_event_v.retain(|event| {
                //                 !matches!(
                //                     event,
                //                     egui::Event::Key {
                //                         key: egui::Key::L,
                //                         pressed: true,
                //                         modifiers: egui::Modifiers::NONE
                //                     }
                //                 )
                //             });
                //             // Take off the 'l' char and push the remaining Text string.
                //             string.remove(0);
                //             event_handler_ctx
                //                 .remaining_event_v
                //                 .push_front(egui::Event::Text(string));
                //         }

                //         // Enter the "line" elem view by adding cursor tokens.
                //         let mut cursor_len = event_handler_ctx.cursor_address.len() as u32;
                //         event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
                //             address: vec![cursor_len.into_value()].into(),
                //             edit: sept::qv::InsertionTerm {
                //                 new_data: "line".to_string().into(),
                //             }
                //             .into(),
                //         }));
                //         cursor_len += 1;
                //         event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
                //             address: vec![cursor_len.into_value()].into(),
                //             edit: sept::qv::InsertionTerm {
                //                 new_data: 0u32.into(),
                //             }
                //             .into(),
                //         }));
                //         // We consumed the event.
                //         Ok(None)
                //     }
                event => {
                    // We didn't consume the event, so return it.
                    Ok(Some(event))
                }
            }
        }
    }
}

impl<'a> EventHandler for sept::qv::Utf8StringTermCharView<'a> {
    fn handle_event(
        &self,
        event: egui::Event,
        event_handler_ctx: &mut EventHandlerCtx<'_>,
        cursor_address_token_i: &mut dyn std::iter::Iterator<Item = &sept::dy::Value>,
    ) -> Result<Option<egui::Event>> {
        // tracing::debug!("Utf8StringTermCharView::handle_event; event: {:?}", event);
        if let Some(cursor_address_token) = cursor_address_token_i.next() {
            use sept::qv::SingleQuery;
            match self.run_single_query(cursor_address_token)? {
                sept::qv::Utf8StringTermCharViewQuery::Utf8StringTermCharElemView(v) => {
                    v.handle_event(event, event_handler_ctx, cursor_address_token_i)
                }
            }
        } else {
            // This is the value addressed by the cursor.
            // TODO: Probably put this into a method in the EventHandler trait.
            match event {
                event => {
                    // We didn't consume the event, so return it.
                    Ok(Some(event))
                }
            }
        }
    }
}

impl<'a> EventHandler for sept::qv::Utf8StringTermCharElemView<'a> {
    fn handle_event(
        &self,
        event: egui::Event,
        event_handler_ctx: &mut EventHandlerCtx<'_>,
        cursor_address_token_i: &mut dyn std::iter::Iterator<Item = &sept::dy::Value>,
    ) -> Result<Option<egui::Event>> {
        // tracing::debug!(
        //     "Utf8StringTermCharElemView::handle_event; event: {:?}",
        //     event
        // );
        let cursor_len = event_handler_ctx.cursor_address.len() as u32;
        if let Some(cursor_address_token) = cursor_address_token_i.next() {
            use sept::qv::SingleQuery;
            match self.run_single_query(cursor_address_token)? {}
            // return Ok(false);
        } else {
            // This is the value addressed by the cursor.
            // TODO: Probably put this into a method in the EventHandler trait.
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
                        event_handler_ctx.enqueue_command(Command::RootValueEdit(AddressedEdit {
                            address: event_handler_ctx.cursor_address.clone(),
                            edit: sept::qv::InsertionTerm { new_data: c.into() }.into(),
                        }));

                        // Cursor edit - Update the char_index
                        {
                            let old_char_index = self.char_index as u32;
                            let new_char_index = old_char_index.saturating_add(1);
                            event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
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
                // TODO: Handle copy/cut
                egui::Event::Key {
                    key: egui::Key::ArrowLeft,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                } => {
                    // Cursor edit - Update the char_index
                    {
                        let old_char_index = self.char_index as u32;
                        let new_char_index = {
                            let mut v = self.clone();
                            v.increment_char_index_by(-1);
                            v.char_index as u32
                        };
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
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
                    key: egui::Key::ArrowRight,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                } => {
                    // Cursor edit - Update the char_index
                    {
                        let old_char_index = self.char_index as u32;
                        let new_char_index = {
                            let mut v = self.clone();
                            v.increment_char_index_by(1);
                            v.char_index as u32
                        };
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
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
                    key: egui::Key::Home,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                } => {
                    // Cursor edit - Update the char_index
                    {
                        let old_char_index = self.char_index as u32;
                        let new_char_index = {
                            let mut v = self.clone();
                            v.go_home();
                            v.char_index as u32
                        };
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
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
                    key: egui::Key::End,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                } => {
                    // Cursor edit - Update the char_index
                    {
                        let old_char_index = self.char_index as u32;
                        let new_char_index = {
                            let mut v = self.clone();
                            v.go_end();
                            v.char_index as u32
                        };
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
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
                // egui::Event::Key {
                //     key: egui::Key::PageUp,
                //     pressed: true,
                //     modifiers: egui::Modifiers::NONE,
                // } => {
                //     view_ctx.cursor_address_last_token_u32_increment_by(-view_ctx.page_up_down_delta, self.char_count as u32);
                // // We consumed the event.
                // Ok(None)
                // }
                // egui::Event::Key {
                //     key: egui::Key::PageDown,
                //     pressed: true,
                //     modifiers: egui::Modifiers::NONE,
                // } => {
                //     view_ctx.cursor_address_last_token_u32_increment_by(view_ctx.page_up_down_delta, self.char_count as u32);
                // // We consumed the event.
                // Ok(None)
                // }
                egui::Event::Key {
                    key: egui::Key::Delete,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                } => {
                    let cursor_char_o = {
                        use sept::qv::QueryableDynTrait;
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
                        event_handler_ctx.enqueue_command(Command::RootValueEdit(AddressedEdit {
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
                } => {
                    // Only Backspace if we're not at the beginning of the line.
                    // TODO: Implement line wrap
                    if self.char_index > 0 {
                        let mut v = self.clone();
                        // TODO: Impl wrap and put it in ViewOptions
                        v.increment_char_index_by(-1);
                        let cursor_char_o = {
                            use sept::qv::EvalTrait;
                            let cursor_value_la = v.eval().unwrap();
                            // This view will produce a char if the cursor is in bounds.
                            let cursor_char_o =
                                cursor_value_la.read().downcast_ref::<char>().map(|c| *c);
                            cursor_char_o
                        };
                        if let Some(cursor_char) = cursor_char_o {
                            // Only if there was a char to Backspace through should we enqueue commands
                            // to edit the root value and update the cursor.

                            // Compute the updated cursor_address
                            let mut updated_cursor_address =
                                event_handler_ctx.cursor_address.clone();
                            let cursor_len = updated_cursor_address.len();
                            assert!(cursor_len >= 2);
                            updated_cursor_address[cursor_len - 1] =
                                (v.char_index as u32).into_value();

                            // Root edit
                            event_handler_ctx.enqueue_command(Command::RootValueEdit(
                                AddressedEdit {
                                    address: updated_cursor_address.clone(),
                                    edit: sept::qv::DeletionTerm {
                                        old_data: cursor_char.into(),
                                    }
                                    .into(),
                                },
                            ));

                            // Cursor edit
                            {
                                // Char index
                                {
                                    event_handler_ctx.enqueue_command(Command::CursorEdit(
                                        AddressedEdit {
                                            // TODO: This could use `-1` as the address once negative indexing is supported.
                                            address: vec![((cursor_len - 1) as u32).into_value()]
                                                .into(),
                                            edit: sept::qv::ReplacementTerm {
                                                old_data: event_handler_ctx.cursor_address
                                                    [cursor_len - 1]
                                                    .clone(),
                                                new_data: updated_cursor_address[cursor_len - 1]
                                                    .clone(),
                                            }
                                            .into(),
                                        },
                                    ));
                                }
                            }
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
                } if key == egui::Key::Enter || key == egui::Key::Tab => {
                    let c = if key == egui::Key::Enter { '\n' } else { '\t' };

                    let cursor_len = event_handler_ctx.cursor_address.len() as u32;
                    assert!(cursor_len >= 2);
                    let mut cursor_address = event_handler_ctx.cursor_address.clone();

                    // Root value edit
                    event_handler_ctx.enqueue_command(Command::RootValueEdit(AddressedEdit {
                        address: cursor_address.clone(),
                        edit: sept::qv::InsertionTerm { new_data: c.into() }.into(),
                    }));

                    // Cursor edit - Update the char_index
                    {
                        let cursor_address_char_index = cursor_address
                            .last_mut()
                            .unwrap()
                            .downcast_mut::<u32>()
                            .unwrap();
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
                            // TODO: This could use `-1` as the address once negative indexing is supported.
                            address: vec![(cursor_len - 1).into_value()].into(),
                            edit: sept::qv::ReplacementTerm {
                                old_data: (*cursor_address_char_index).into(),
                                new_data: (*cursor_address_char_index + 1).into(),
                            }
                            .into(),
                        }));
                        *cursor_address_char_index += 1;
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

impl<'a> EventHandler for sept::qv::Utf8StringTermLineView<'a> {
    fn handle_event(
        &self,
        event: egui::Event,
        event_handler_ctx: &mut EventHandlerCtx<'_>,
        cursor_address_token_i: &mut dyn std::iter::Iterator<Item = &sept::dy::Value>,
    ) -> Result<Option<egui::Event>> {
        // tracing::debug!("Utf8StringTermLineView::handle_event; event: {:?}", event);
        if let Some(cursor_address_token) = cursor_address_token_i.next() {
            use sept::qv::SingleQuery;
            match self.run_single_query(cursor_address_token)? {
                sept::qv::Utf8StringTermLineViewQuery::Utf8StringTermLineElemView(v) => {
                    v.handle_event(event, event_handler_ctx, cursor_address_token_i)
                }
            }
        } else {
            // This is the value addressed by the cursor.
            // TODO: Probably put this into a method in the EventHandler trait.
            match event {
                event => {
                    // We didn't consume the event, so return it.
                    Ok(Some(event))
                }
            }
        }
    }
}

impl<'a> EventHandler for sept::qv::Utf8StringTermLineElemView<'a> {
    fn handle_event(
        &self,
        event: egui::Event,
        event_handler_ctx: &mut EventHandlerCtx<'_>,
        cursor_address_token_i: &mut dyn std::iter::Iterator<Item = &sept::dy::Value>,
    ) -> Result<Option<egui::Event>> {
        // tracing::debug!(
        //     "Utf8StringTermLineElemView::handle_event; event: {:?}",
        //     event
        // );
        let cursor_len = event_handler_ctx.cursor_address.len() as u32;
        if let Some(cursor_address_token) = cursor_address_token_i.next() {
            let mut event_handler_ctx_g = event_handler_ctx.push_nesting_depth();
            let event_handler_ctx = &mut event_handler_ctx_g;
            use sept::qv::SingleQuery;
            match self.run_single_query(cursor_address_token)? {
                sept::qv::Utf8StringTermLineElemViewQuery::Utf8StringTermLineElemCharView(v) => {
                    v.handle_event(event, event_handler_ctx, cursor_address_token_i)
                }
            }
        } else {
            // This is the value addressed by the cursor.
            // TODO: Probably put this into a method in the EventHandler trait.
            match event {
                egui::Event::Key {
                    key: egui::Key::Enter,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
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
                }
                | egui::Event::Key {
                    key: egui::Key::Escape,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
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
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
                            // TODO: This could use `-1` as the address once negative indexing is supported.
                            address: vec![(cursor_len - 1).into_value()].into(),
                            edit: sept::qv::ReplacementTerm {
                                old_data: old_line_index.into(),
                                new_data: new_line_index.into(),
                            }
                            .into(),
                        }));
                    }
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::ArrowDown,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
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
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
                            // TODO: This could use `-1` as the address once negative indexing is supported.
                            address: vec![(cursor_len - 1).into_value()].into(),
                            edit: sept::qv::ReplacementTerm {
                                old_data: old_line_index.into(),
                                new_data: new_line_index.into(),
                            }
                            .into(),
                        }));
                    }
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::ArrowLeft,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
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
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
                            // TODO: This could use `-1` as the address once negative indexing is supported.
                            address: vec![(cursor_len - 1).into_value()].into(),
                            edit: sept::qv::ReplacementTerm {
                                old_data: old_line_index.into(),
                                new_data: new_line_index.into(),
                            }
                            .into(),
                        }));
                    }
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::ArrowRight,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
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
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
                            // TODO: This could use `-1` as the address once negative indexing is supported.
                            address: vec![(cursor_len - 1).into_value()].into(),
                            edit: sept::qv::ReplacementTerm {
                                old_data: old_line_index.into(),
                                new_data: new_line_index.into(),
                            }
                            .into(),
                        }));
                    }
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::Home,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                } => {
                    // Cursor edit - Update the line_index
                    {
                        let old_line_index = self.line_index as u32;
                        let new_line_index = {
                            let mut v = self.clone();
                            v.go_home();
                            v.line_index as u32
                        };
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
                            // TODO: This could use `-1` as the address once negative indexing is supported.
                            address: vec![(cursor_len - 1).into_value()].into(),
                            edit: sept::qv::ReplacementTerm {
                                old_data: old_line_index.into(),
                                new_data: new_line_index.into(),
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
                    // Cursor edit - Update the line_index
                    {
                        let old_line_index = self.line_index as u32;
                        let new_line_index = {
                            let mut v = self.clone();
                            v.go_end();
                            v.line_index as u32
                        };
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
                            // TODO: This could use `-1` as the address once negative indexing is supported.
                            address: vec![(cursor_len - 1).into_value()].into(),
                            edit: sept::qv::ReplacementTerm {
                                old_data: old_line_index.into(),
                                new_data: new_line_index.into(),
                            }
                            .into(),
                        }));
                    }
                    // We consumed the event.
                    Ok(None)
                }
                // egui::Event::Key {
                //     key: egui::Key::PageUp,
                //     pressed: true,
                //     modifiers: egui::Modifiers::NONE,
                // } => {
                // // We consumed the event.
                // Ok(None)
                // }
                // egui::Event::Key {
                //     key: egui::Key::PageDown,
                //     pressed: true,
                //     modifiers: egui::Modifiers::NONE,
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

impl<'a> EventHandler for sept::qv::Utf8StringTermLineElemCharView<'a> {
    fn handle_event(
        &self,
        event: egui::Event,
        event_handler_ctx: &mut EventHandlerCtx<'_>,
        cursor_address_token_i: &mut dyn std::iter::Iterator<Item = &sept::dy::Value>,
    ) -> Result<Option<egui::Event>> {
        // tracing::debug!(
        //     "Utf8StringTermLineElemCharView::handle_event; event: {:?}",
        //     event
        // );
        if let Some(cursor_address_token) = cursor_address_token_i.next() {
            use sept::qv::SingleQuery;
            match self.run_single_query(cursor_address_token)? {
                sept::qv::Utf8StringTermLineElemCharViewQuery::Utf8StringTermLineElemCharElemView(v) => {
                    v.handle_event(event, event_handler_ctx, cursor_address_token_i)
                }
            }
        } else {
            // This is the value addressed by the cursor.
            // TODO: Probably put this into a method in the EventHandler trait.
            match event {
                event => {
                    // We didn't consume the event, so return it.
                    Ok(Some(event))
                }
            }
        }
    }
}

impl<'a> EventHandler for sept::qv::Utf8StringTermLineElemCharElemView<'a> {
    fn handle_event(
        &self,
        event: egui::Event,
        event_handler_ctx: &mut EventHandlerCtx<'_>,
        cursor_address_token_i: &mut dyn std::iter::Iterator<Item = &sept::dy::Value>,
    ) -> Result<Option<egui::Event>> {
        // tracing::debug!(
        //     "Utf8StringTermLineElemCharElemView::handle_event; event: {:?}",
        //     event
        // );
        let cursor_len = event_handler_ctx.cursor_address.len() as u32;
        if let Some(cursor_address_token) = cursor_address_token_i.next() {
            use sept::qv::SingleQuery;
            match self.run_single_query(cursor_address_token)? {}
            // // We didn't consume the event, so return it.
            // Ok(Some(event))
        } else {
            // This is the value addressed by the cursor.
            // TODO: Probably put this into a method in the EventHandler trait.

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
                    // // Escape this view by taking off the last two cursor tokens.
                    // event_handler_ctx.enqueue_command_cursor_address_pop(2);
                    // // We consumed the event.
                    // Ok(None)

                    // Fully escape line-char mode by taking off the last four cursor tokens.
                    event_handler_ctx.enqueue_command_cursor_address_pop(4);
                    // We consumed the event.
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::Enter,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                } => {
                    // Root value edit
                    event_handler_ctx.enqueue_command(Command::RootValueEdit(AddressedEdit {
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
                            // let new_line_index = {
                            //     let mut v = self.clone();
                            //     v.increment_line_index_by(1);
                            //     v.line_index as u32
                            // };
                            event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
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
                            event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
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
                } => {
                    // Root value edit
                    event_handler_ctx.enqueue_command(Command::RootValueEdit(AddressedEdit {
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
                        // let new_char_index = {
                        //     let mut v = self.clone();
                        //     // NOTE: this won't necessarily work because v.string isn't changed so the bounds
                        //     // clamping will interfere if we're at the end of the line.
                        //     v.increment_char_index_by(1, false);
                        //     v.char_index as u32
                        // };
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
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
                } => {
                    // let cursor_char_o = {
                    //     let root_value_g = event_handler_ctx.model.root_value_la.read().unwrap();
                    //     use sept::qv::QueryableDynTrait;
                    //     let query_b = root_value_g
                    //         .make_and_run_query(&mut event_handler_ctx.cursor_address.iter())
                    //         .unwrap();
                    //     let cursor_value_la = query_b.eval().unwrap();
                    //     // This view will produce a char if the cursor is in bounds.
                    //     let cursor_char_o =
                    //         cursor_value_la.read().downcast_ref::<char>().map(|c| *c);
                    //     cursor_char_o
                    // };
                    let cursor_char_o = {
                        use sept::qv::EvalTrait;
                        let cursor_value_la = self.eval().unwrap();
                        // This view will produce a char if the cursor is in bounds.
                        let cursor_char_o =
                            cursor_value_la.read().downcast_ref::<char>().map(|c| *c);
                        cursor_char_o
                    };
                    if let Some(cursor_char) = cursor_char_o {
                        event_handler_ctx.enqueue_command(Command::RootValueEdit(AddressedEdit {
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
                } => {
                    // Only backspace if we're not at the beginning of the string.
                    if self.line_index > 0 || self.char_index > 0 {
                        let mut v = self.clone();
                        // TODO: Impl wrap and put it in ViewOptions
                        v.increment_char_index_by(-1, true);
                        let cursor_char_o = {
                            use sept::qv::EvalTrait;
                            let cursor_value_la = v.eval().unwrap();
                            // This view will produce a char if the cursor is in bounds.
                            let cursor_char_o =
                                cursor_value_la.read().downcast_ref::<char>().map(|c| *c);
                            cursor_char_o
                        };
                        if let Some(cursor_char) = cursor_char_o {
                            // Only if there was a char to Backspace through should we enqueue commands
                            // to edit the root value and update the cursor.

                            // Compute the updated cursor_address
                            let mut updated_cursor_address =
                                event_handler_ctx.cursor_address.clone();
                            let cursor_len = updated_cursor_address.len();
                            assert!(cursor_len >= 4);
                            updated_cursor_address[cursor_len - 3] =
                                (v.line_index as u32).into_value();
                            updated_cursor_address[cursor_len - 1] =
                                (v.char_index as u32).into_value();

                            // Root edit
                            event_handler_ctx.enqueue_command(Command::RootValueEdit(
                                AddressedEdit {
                                    address: updated_cursor_address.clone(),
                                    edit: sept::qv::DeletionTerm {
                                        old_data: cursor_char.into(),
                                    }
                                    .into(),
                                },
                            ));

                            // Cursor edit
                            {
                                // Line index
                                {
                                    event_handler_ctx.enqueue_command(Command::CursorEdit(
                                        AddressedEdit {
                                            // TODO: This could use `-3` as the address once negative indexing is supported.
                                            address: vec![((cursor_len - 3) as u32).into_value()]
                                                .into(),
                                            edit: sept::qv::ReplacementTerm {
                                                old_data: event_handler_ctx.cursor_address
                                                    [cursor_len - 3]
                                                    .clone(),
                                                new_data: updated_cursor_address[cursor_len - 3]
                                                    .clone(),
                                            }
                                            .into(),
                                        },
                                    ));
                                }
                                // Char index
                                {
                                    event_handler_ctx.enqueue_command(Command::CursorEdit(
                                        AddressedEdit {
                                            // TODO: This could use `-1` as the address once negative indexing is supported.
                                            address: vec![((cursor_len - 1) as u32).into_value()]
                                                .into(),
                                            edit: sept::qv::ReplacementTerm {
                                                old_data: event_handler_ctx.cursor_address
                                                    [cursor_len - 1]
                                                    .clone(),
                                                new_data: updated_cursor_address[cursor_len - 1]
                                                    .clone(),
                                            }
                                            .into(),
                                        },
                                    ));
                                }
                            }
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
                        event_handler_ctx.enqueue_command(Command::RootValueEdit(AddressedEdit {
                            address: event_handler_ctx.cursor_address.clone(),
                            edit: sept::qv::InsertionTerm { new_data: c.into() }.into(),
                        }));

                        // Cursor edit
                        if c == '\n' {
                            // Cursor edit - Update the line_index
                            {
                                let old_line_index = self.line_index as u32;
                                let new_line_index = old_line_index.saturating_add(1);
                                // let new_line_index = {
                                //     let mut v = self.clone();
                                //     v.increment_line_index_by(1);
                                //     v.line_index as u32
                                // };
                                event_handler_ctx.enqueue_command(Command::CursorEdit(
                                    AddressedEdit {
                                        // TODO: This could use `-3` as the address once negative indexing is supported.
                                        address: vec![(cursor_len - 3).into_value()].into(),
                                        edit: sept::qv::ReplacementTerm {
                                            old_data: old_line_index.into(),
                                            new_data: new_line_index.into(),
                                        }
                                        .into(),
                                    },
                                ));
                            }
                            // Cursor edit - Update the char_index
                            {
                                let old_char_index = self.char_index as u32;
                                let new_char_index = {
                                    let mut v = self.clone();
                                    v.go_home();
                                    v.char_index as u32
                                };
                                event_handler_ctx.enqueue_command(Command::CursorEdit(
                                    AddressedEdit {
                                        // TODO: This could use `-1` as the address once negative indexing is supported.
                                        address: vec![(cursor_len - 1).into_value()].into(),
                                        edit: sept::qv::ReplacementTerm {
                                            old_data: old_char_index.into(),
                                            new_data: new_char_index.into(),
                                        }
                                        .into(),
                                    },
                                ));
                            }
                            // // Update the line_index
                            // {
                            //     let cursor_address_line_index = cursor_address
                            //         .get_mut((cursor_len - 3) as usize)
                            //         .unwrap()
                            //         .downcast_mut::<u32>()
                            //         .unwrap();
                            //     event_handler_ctx.enqueue_command(Command::CursorEdit(
                            //         AddressedEdit {
                            //             // TODO: This could use `-3` as the address once negative indexing is supported.
                            //             address: vec![(cursor_len - 3).into_value()].into(),
                            //             edit: sept::qv::ReplacementTerm {
                            //                 old_data: (*cursor_address_line_index).into(),
                            //                 new_data: (*cursor_address_line_index + 1).into(),
                            //             }
                            //             .into(),
                            //         },
                            //     ));
                            //     *cursor_address_line_index += 1;
                            // }

                            // // Update the char_index
                            // {
                            //     let cursor_address_char_index = cursor_address
                            //         .get_mut((cursor_len - 1) as usize)
                            //         .unwrap()
                            //         .downcast_mut::<u32>()
                            //         .unwrap();
                            //     event_handler_ctx.enqueue_command(Command::CursorEdit(
                            //         AddressedEdit {
                            //             // TODO: This could use `-1` as the address once negative indexing is supported.
                            //             address: vec![(cursor_len - 1).into_value()].into(),
                            //             edit: sept::qv::ReplacementTerm {
                            //                 old_data: (*cursor_address_char_index).into(),
                            //                 new_data: 0u32.into(),
                            //             }
                            //             .into(),
                            //         },
                            //     ));
                            //     *cursor_address_char_index = 0;
                            // }
                        } else {
                            // Cursor edit - Update the char_index
                            {
                                let old_char_index = self.char_index as u32;
                                let new_char_index = old_char_index.saturating_add(1);
                                // let new_char_index = {
                                //     let mut v = self.clone();
                                //     v.increment_char_index_by(1, false);
                                //     v.char_index as u32
                                // };
                                event_handler_ctx.enqueue_command(Command::CursorEdit(
                                    AddressedEdit {
                                        // TODO: This could use `-1` as the address once negative indexing is supported.
                                        address: vec![(cursor_len - 1).into_value()].into(),
                                        edit: sept::qv::ReplacementTerm {
                                            old_data: old_char_index.into(),
                                            new_data: new_char_index.into(),
                                        }
                                        .into(),
                                    },
                                ));
                            }

                            // // Update the char_index
                            // let cursor_address_char_index = cursor_address
                            //     .get_mut((cursor_len - 1) as usize)
                            //     .unwrap()
                            //     .downcast_mut::<u32>()
                            //     .unwrap();
                            // event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
                            //     // TODO: This could use `-1` as the address once negative indexing is supported.
                            //     address: vec![(cursor_len - 1).into_value()].into(),
                            //     edit: sept::qv::ReplacementTerm {
                            //         old_data: (*cursor_address_char_index).into(),
                            //         new_data: (*cursor_address_char_index + 1).into(),
                            //     }
                            //     .into(),
                            // }));
                            // *cursor_address_char_index += 1;
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
                } => {
                    // let mut v = self.clone();
                    // v.increment_line_index_by(-1);
                    // {
                    //     let cursor_line_index = view_ctx
                    //         .cursor_address_nth_to_last_token_mut(2)
                    //         .downcast_mut::<u32>()
                    //         .unwrap();
                    //     *cursor_line_index = v.line_index as u32;
                    // }
                    // {
                    //     let cursor_char_index = view_ctx
                    //         .cursor_address_nth_to_last_token_mut(0)
                    //         .downcast_mut::<u32>()
                    //         .unwrap();
                    //     *cursor_char_index = v.char_index as u32;
                    // }
                    let mut v = self.clone();
                    v.increment_line_index_by(-1);
                    // Cursor edit - Update the line_index
                    {
                        let old_line_index = self.line_index as u32;
                        let new_line_index = v.line_index as u32;
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
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
                        let new_char_index = v.char_index as u32;
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
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
                    key: egui::Key::ArrowDown,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                } => {
                    let mut v = self.clone();
                    v.increment_line_index_by(1);
                    // Cursor edit - Update the line_index
                    {
                        let old_line_index = self.line_index as u32;
                        let new_line_index = v.line_index as u32;
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
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
                        let new_char_index = v.char_index as u32;
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
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
                    key: egui::Key::ArrowLeft,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                } => {
                    let mut v = self.clone();
                    v.increment_char_index_by(-1, true);
                    // Cursor edit - Update the line_index
                    {
                        let old_line_index = self.line_index as u32;
                        let new_line_index = v.line_index as u32;
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
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
                        let new_char_index = v.char_index as u32;
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
                            // TODO: This could use `-1` as the address once negative indexing is supported.
                            address: vec![(cursor_len - 1).into_value()].into(),
                            edit: sept::qv::ReplacementTerm {
                                old_data: old_char_index.into(),
                                new_data: new_char_index.into(),
                            }
                            .into(),
                        }));
                    }
                    Ok(None)
                }
                egui::Event::Key {
                    key: egui::Key::ArrowRight,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                } => {
                    let mut v = self.clone();
                    v.increment_char_index_by(1, true);
                    // Cursor edit - Update the line_index
                    {
                        let old_line_index = self.line_index as u32;
                        let new_line_index = v.line_index as u32;
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
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
                        let new_char_index = v.char_index as u32;
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
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
                    key: egui::Key::Home,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                } => {
                    let mut v = self.clone();
                    v.go_home();
                    // Cursor edit - Update the line_index
                    {
                        let old_line_index = self.line_index as u32;
                        let new_line_index = v.line_index as u32;
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
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
                        let new_char_index = v.char_index as u32;
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
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
                    key: egui::Key::End,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                } => {
                    let mut v = self.clone();
                    v.go_end();
                    // Cursor edit - Update the line_index
                    {
                        let old_line_index = self.line_index as u32;
                        let new_line_index = v.line_index as u32;
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
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
                        let new_char_index = v.char_index as u32;
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
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
                    key: egui::Key::PageUp,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                } => {
                    let mut v = self.clone();
                    v.increment_line_index_by(-(event_handler_ctx.page_up_down_delta() as isize));
                    // Cursor edit - Update the line_index
                    {
                        let old_line_index = self.line_index as u32;
                        let new_line_index = v.line_index as u32;
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
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
                        let new_char_index = v.char_index as u32;
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
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
                    key: egui::Key::PageDown,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                } => {
                    let mut v = self.clone();
                    v.increment_line_index_by(event_handler_ctx.page_up_down_delta() as isize);
                    // Cursor edit - Update the line_index
                    {
                        let old_line_index = self.line_index as u32;
                        let new_line_index = v.line_index as u32;
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
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
                        let new_char_index = v.char_index as u32;
                        event_handler_ctx.enqueue_command(Command::CursorEdit(AddressedEdit {
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
                event => {
                    // We didn't consume the event, so return it.
                    Ok(Some(event))
                }
            }
        }
    }
}

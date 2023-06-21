// TEMP HACK
#![allow(unused)]

use crate::{
    extract_text_prefix_from_front_text, Command, EventHandlerCtx, Model, ValueUI, ViewCtx,
    ViewOptions,
};
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    // TODO: Re-enable serde
    #[serde(skip)]
    model: Model,
    #[serde(skip)]
    cursor_address: sept::dy::TupleTerm,
    #[serde(skip)]
    view_options: ViewOptions,
    #[serde(skip)]
    local_symbol_table_la: Arc<RwLock<sept::dy::SymbolTable>>,
}

impl Default for App {
    fn default() -> Self {
        let a1 = sept::dy::ArrayTerm::from(vec![
            true.into(),
            false.into(),
            sept::dy::ArrayTerm::from(vec![]).into(),
            123i8.into(),
            200u8.into(),
            12345i16.into(),
            45678u16.into(),
            1234567i32.into(),
            4567890u32.into(),
            1000000000000i64.into(),
            9223372036854775808u64.into(),
            10101.202f32.into(),
            1.01020304050607f64.into(),
            sept::st::Void.into(),
            sept::st::True.into(),
            sept::st::False.into(),
            sept::st::EmptyType.into(),
            sept::st::Void.into(),
            sept::st::Void.into(),
            sept::st::Void.into(),
            sept::st::Void.into(),
            sept::st::Void.into(),
            sept::st::Void.into(),
            sept::st::Void.into(),
            sept::st::Void.into(),
            sept::st::Void.into(),
            sept::st::Void.into(),
            sept::st::Void.into(),
            sept::st::Void.into(),
        ]);
        let a2 =
            sept::dy::ArrayTerm::from(vec![true.into(), 606i32.into(), a1.into(), false.into()]);

        let m0 = sept::dy::OrderedMapTerm::from(maplit::btreemap! {});
        let m1 = sept::dy::OrderedMapTerm::from(
            maplit::btreemap! { 3i32.into() => "blah".to_string().into(), 5.5f32.into() => sept::st::Void.into() },
        );
        let m2 = sept::dy::OrderedMapTerm::from(maplit::btreemap! {
            sept::dy::OrderedMapTerm::from(maplit::btreemap! { false.into() => 123u32.into() }).into() => 505.606f64.into(),
            true.into() => sept::st::Void.into(),
            sept::st::Bool.into() => sept::dy::OrderedMapTerm::from(
                maplit::btreemap! { 3i32.into() => "blah".to_string().into(), 5.5f32.into() => sept::st::Void.into() },
            ).into()
        });

        let t1 = sept::dy::TupleTerm::from(vec![
            true.into(),
            false.into(),
            123i8.into(),
            sept::dy::TupleTerm::from(vec![
                200u8.into(),
                sept::dy::TupleTerm::from(vec![
                    12345i16.into(),
                    45678u16.into(),
                    1234567i32.into(),
                ])
                .into(),
                4567890u32.into(),
                1000000000000i64.into(),
            ])
            .into(),
            9223372036854775808u64.into(),
            10101.202f32.into(),
            1.01020304050607f64.into(),
        ]);

        let t2 = sept::dy::TupleTerm::from(vec![
            sept::st::Void.into(),
            sept::st::VoidType.into(),
            sept::st::Bool.into(),
            sept::st::BoolType.into(),
            "blah\nthing\tWAAAA\tXyz\n\n!!!\rx".to_string().into(),
        ]);
        let t3 = sept::dy::TupleTerm::from(vec![
            sept::st::Sint32.into(),
            sept::st::Utf8String.into(),
            sept::st::Array.into(),
        ]);

        let st1 = sept::dy::StructTerm::new(
            vec![
                ("age".into(), sept::st::Uint8.into()),
                ("gravity".into(), sept::st::Float64.into()),
                ("thingy".into(), t3.into()),
            ]
            .into(),
        )
        .unwrap();

        let st0 = sept::dy::StructTerm::new(vec![].into()).unwrap();

        use sept::dy::Constructor;

        // Make an empty StructTermTerm
        let stt0 = st0.construct(sept::dy::TupleTerm::from(vec![])).unwrap();

        let stt1 = st1
            .construct(sept::dy::TupleTerm::from(vec![
                28u8.into(),
                4035.56f64.into(),
                sept::dy::TupleTerm::from(vec![
                    445566i32.into(),
                    "OSTRICH".to_string().into(),
                    sept::dy::ArrayTerm::from(vec![]).into(),
                ])
                .into(),
            ]))
            .unwrap();

        let s0 = String::new();
        let s1 = "+++ one day, a big hippo came along and wrecked\teverything.\nyes, i mean absolutely everything!\nthere was nothing left.\n\n\tnothing left but hippos.".to_string();
        // Very long string.
        let s2 = "blah blah blahhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh blah blah blahhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh blah blah blahhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh blah blah blahhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh blah blah blahhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh blah blah blahhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh blah blah blahhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh blah blah blahhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh".to_string();

        let gsr0 = sept::dy::GlobalSymRefTerm::new_unchecked("Hippo".to_string());
        // NOTE: This uses the dereferenced StructTerm as the type and not the GlobalSymRefTerm as intended.
        // TODO: Figure out how to get it to construct correctly.
        let stt2 = gsr0
            .construct(sept::dy::TupleTerm::from(vec![
                28u8.into(),
                4035.56f64.into(),
            ]))
            .unwrap();
        // Manually construct the StructTermTerm so that the GlobalSymRefTerm is used as its type.
        let stt3 = sept::dy::StructTermTerm::new_checked(
            gsr0.clone().into(),
            sept::dy::TupleTerm::from(vec![28u8.into(), 4035.56f64.into()]),
        )
        .unwrap();

        let local_symbol_table_la = Arc::new(RwLock::new(
            sept::dy::SymbolTable::new_without_parent("fancy".to_string()).expect("test"),
        ));
        local_symbol_table_la
            .write()
            .unwrap()
            .define_symbol("blah", sept::dy::Value::from(1230321i32))
            .expect("test");
        tracing::debug!(
            "local_symbol_table_la: {:#?}",
            local_symbol_table_la.read().unwrap()
        );

        let lsr0 =
            sept::dy::LocalSymRefTerm::new_checked(local_symbol_table_la.clone(), "blah".into())
                .expect("test");

        let gsr1 =
            sept::dy::GlobalSymRefTerm::new_unchecked("this one doesn't resolve".to_string());
        let lsr1 = sept::dy::LocalSymRefTerm::new_unchecked(
            local_symbol_table_la.clone(),
            "also doesn't resolve".into(),
        );

        // TEMP HACK
        // let value: sept::dy::Value = sept::dy::ArrayTerm::from(vec![s1.into()]).into();
        // let value: sept::dy::Value = s1.into();

        // let root_value: sept::dy::Value = sept::dy::ArrayTerm::from(vec![
        //     a2.into(),
        //     sept::dy::ArrayTerm::from(vec![]).into(),
        //     m0.into(),
        //     m1.into(),
        //     m2.into(),
        //     t1.into(),
        //     t2.into(),
        //     sept::dy::TupleTerm::from(vec![]).into(),
        //     st0.into(),
        //     st1.into(),
        //     stt0.into(),
        //     stt1.into(),
        //     s0.into(),
        //     s1.into(),
        //     s2.into(),
        //     gsr0.into(),
        //     stt2.into(),
        //     stt3.into(),
        //     lsr0.into(),
        //     gsr1.into(),
        //     lsr1.into(),
        // ])
        // .into();
        use sept::dy::IntoValue;
        // let root_value = s1.into_value();
        let root_value = sept::dy::ArrayTerm::from(vec![
            "".to_string().into(),
            "a".to_string().into(),
            "\n".to_string().into(),
            "xy\npq\n".to_string().into(),
            "hippos\nare\tabsolutely\nthe\nbest".to_string().into(),
            sept::dy::ArrayTerm::from(vec![
                sept::dy::ArrayTerm::from(vec![]).into(),
                "thingy".to_string().into(),
                "other\nthingy".to_string().into(),
            ])
            .into(),
        ])
        .into_value();
        let root_value_la = Arc::new(RwLock::new(root_value));
        let model = Model {
            root_value_la,
            executed_command_v: VecDeque::new(),
        };
        // Start with the cursor on the root value.
        let cursor_address = sept::dy::TupleTerm::from(vec![]);
        let view_options = ViewOptions::default();

        Self {
            model,
            cursor_address,
            view_options,
            local_symbol_table_la,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    // No File > Quit on web pages.
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        if ui.button("Quit").clicked() {
                            frame.close();
                        }
                    }
                    #[cfg(target_arch = "wasm32")]
                    {
                        let _ = frame;
                        let _ = ui;
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            // Render the cursor address.  Unfortunately because this has to be rendered before
            // the CentralPanel, this gets updated with a slight delay after the events that change
            // the cursor address.
            ui.horizontal(|ui| {
                ui.label("Cursor Address:");

                // Create a model for the cursor address.
                let model = Model {
                    root_value_la: Arc::new(RwLock::new(self.cursor_address.clone().into())),
                    executed_command_v: VecDeque::new(),
                };
                // Set the rendering options specific for rendering the cursor address.  These options
                // are to make it very compact.
                // TODO: Make it super compact by eliminating spaces.
                let view_options = ViewOptions {
                    inline_at_nesting_depth: 0,
                    show_type_annotations: false,
                    show_struct_field_name_hints: false,
                    ..Default::default()
                };
                // Create a ViewCtx to be used for rendering the cursor address.  It itself does not
                // have a cursor address, since the user is not interacting with it.
                let mut view_ctx = ViewCtx::new(&model, &view_options, None);

                let old_item_spacing = ui.spacing().item_spacing;
                ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
                // ui.spacing_mut().item_spacing.x = 0.0;

                ui.vertical(|ui| {
                    let layout_job = self.cursor_address.run_ui(ui, &mut view_ctx, None);
                    ui.label(layout_job);
                });

                ui.spacing_mut().item_spacing = old_item_spacing;
            });

            ui.horizontal_wrapped(|ui| {
                ui.label("Inlining Depth:");
                ui.add(
                    egui::DragValue::new(&mut self.view_options.inline_at_nesting_depth)
                        .speed(0.0625),
                );

                ui.checkbox(
                    &mut self.view_options.show_expanded_item_indicator,
                    "Expanded Item Indicator",
                );

                ui.checkbox(
                    &mut self.view_options.show_type_annotations,
                    "Type Annotations",
                );

                ui.checkbox(
                    &mut self.view_options.show_struct_field_name_hints,
                    "Struct Field Name Hints",
                );

                ui.label("Font:");
                ui.add(
                    egui::DragValue::new(&mut self.view_options.font_id.size)
                        .clamp_range(6.0..=30.0)
                        .max_decimals(0)
                        .suffix("pt")
                        .speed(0.0625),
                );
                egui::ComboBox::from_id_source("font family combobox")
                    .selected_text(format!("{:?}", &mut self.view_options.font_id.family))
                    .show_ui(ui, |ui| {
                        // ui.style_mut().wrap = Some(false);
                        ui.set_min_width(60.0);
                        ui.selectable_value(
                            &mut self.view_options.font_id.family,
                            egui::FontFamily::Monospace,
                            "Monospace",
                        );
                        ui.selectable_value(
                            &mut self.view_options.font_id.family,
                            egui::FontFamily::Proportional,
                            "Proportional",
                        );
                    });

                egui::warn_if_debug_build(ui);
            });
        });

        // Note that the CentralPanel must be added after side panels.
        egui::CentralPanel::default().show(ctx, |ui| {
            // tracing::trace!("App::update; events:");
            // for event in ui.input().events.iter() {
            //     tracing::trace!("    {:?}", event);
            // }

            // TODO: Handle all keyboard/text/paste (and possibly other) events for the cursor before
            // starting the render, as these can change the cursor and therefore what should be rendered.
            // Technically mouse events can also change the cursor, but I'm not sure how that can
            // possibly be decoupled using an immediate mode GUI, unless maybe you can guarantee that no
            // more than one cursor-changing mouse event is received at a time.
            {
                let mut input_g = ui.input_mut();
                let mut event_v = std::mem::take(&mut input_g.events)
                    .into_iter()
                    .collect::<VecDeque<_>>();
                if !event_v.is_empty() {
                    tracing::trace!("App::update; {} events:", event_v.len());
                    for event in event_v.iter() {
                        tracing::trace!("    {:?}", event);
                    }
                }
                let mut unhandled_event_v = Vec::new();
                while !event_v.is_empty() {
                    let event = event_v.pop_front().unwrap();
                    let (unhandled_event_o, enqueued_command_v) = {
                        let mut enqueued_command_v = VecDeque::new();

                        // Give the app a chance to handle top-level events.
                        let mut unhandled_event_o =
                            self.handle_top_level_event(event, &mut event_v);
                        // Fall through to the root value if not handled.
                        unhandled_event_o = if let Some(unhandled_event) = unhandled_event_o {
                            let root_value_g = self.model.root_value_la.read().unwrap();
                            let mut event_handler_ctx = EventHandlerCtx::new(
                                &root_value_g,
                                &self.cursor_address,
                                &self.view_options,
                                &mut event_v,
                                &mut enqueued_command_v,
                            );

                            use crate::EventHandler;
                            // If top level didn't handle it, pass it on to the root value.
                            root_value_g
                                .handle_event(
                                    unhandled_event,
                                    &mut event_handler_ctx,
                                    &mut self.cursor_address.iter(),
                                )
                                .unwrap()
                        } else {
                            None
                        };
                        (unhandled_event_o, enqueued_command_v)
                    };

                    // Apply each enqueued command
                    {
                        let mut root_value_g = self.model.root_value_la.write().unwrap();
                        for command in enqueued_command_v.into_iter() {
                            tracing::trace!("App::update; executing command {:?}", command);
                            use sept::qv::QueryMutAndApplyEditTrait;
                            match &command {
                                Command::CursorEdit(addressed_edit) => {
                                    self.cursor_address
                                        .query_mut_and_apply_edit(
                                            &mut addressed_edit.address.iter(),
                                            addressed_edit.edit.clone(),
                                        )
                                        .expect("TODO: handle error");
                                }
                                Command::RootValueEdit(addressed_edit) => {
                                    root_value_g
                                        .query_mut_and_apply_edit(
                                            &mut addressed_edit.address.iter(),
                                            addressed_edit.edit.clone(),
                                        )
                                        .expect("TODO: handle error");
                                }
                            }
                            // Store each edit in the undo queue.
                            self.model.executed_command_v.push_back(command);
                        }
                    }

                    if let Some(unhandled_event) = unhandled_event_o {
                        // Pass the event on to remaining_event_v.
                        unhandled_event_v.push(unhandled_event);
                    }
                }
                // Pass all unhandled events through to the UI's InputState, so that they can be
                // handled on the UI pass.
                input_g.events = unhandled_event_v;
            }

            egui::ScrollArea::vertical()
                .always_show_scroll(true)
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    let old_item_spacing = ui.spacing().item_spacing;
                    ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
                    // ui.spacing_mut().item_spacing = egui::vec2(-1.0, 1.0);
                    // ui.spacing_mut().item_spacing.x = 0.0;

                    let mut view_ctx = ViewCtx::new(
                        &self.model,
                        &self.view_options,
                        Some(&mut self.cursor_address),
                    );

                    ui.vertical(|ui| {
                        let layout_job = self.model.root_value_la.read().unwrap().run_ui(
                            ui,
                            &mut view_ctx,
                            None,
                        );
                        ui.label(layout_job);
                    });

                    ui.spacing_mut().item_spacing = old_item_spacing;
                });
        });
    }
}

impl App {
    /// Handles top-level events, returning any unhandled events as Some(event), or
    /// None if the event was handled.
    pub fn handle_top_level_event(
        &mut self,
        event: egui::Event,
        remaining_event_v: &mut VecDeque<egui::Event>,
    ) -> Option<egui::Event> {
        match event {
            egui::Event::Key {
                key: egui::Key::PlusEquals,
                pressed: true,
                modifiers: egui::Modifiers::ALT,
            } => {
                self.view_options.inline_at_nesting_depth = self
                    .view_options
                    .inline_at_nesting_depth
                    .saturating_add_signed(1);
                // There will be a Text event that starts with "=", so remove that portion.
                extract_text_prefix_from_front_text("=", remaining_event_v);
                // We consumed the event.
                None
            }
            egui::Event::Key {
                key: egui::Key::Minus,
                pressed: true,
                modifiers: egui::Modifiers::ALT,
            } => {
                self.view_options.inline_at_nesting_depth = self
                    .view_options
                    .inline_at_nesting_depth
                    .saturating_add_signed(-1);
                // There will be a Text event that starts with "-", so remove that portion.
                extract_text_prefix_from_front_text("-", remaining_event_v);
                // We consumed the event.
                None
            }
            event => {
                // We didn't consume the event, so return it.
                Some(event)
            }
        }
    }
}

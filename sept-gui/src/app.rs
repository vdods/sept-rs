// TEMP HACK
#![allow(unused)]

use egui::Modifiers;
use sept::st::{DeserializableT, SerializableT};

use crate::{
    edit, extract_text_prefix_from_front_text, AddressedEdit, CursorEdit, Edit, EventHandlerCtx,
    Model, SaveBehavior, ValueUIT, ViewCtx, ViewOptions,
};
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

const OFFER_SAVE_IF_UNSAVED_TITLE: &str = "File Has Unsaved Changes";

/// Result of offer_save_if_unsaved, which is functionality common to multiple methods regarding file operations.
#[derive(Clone, Copy, Debug)]
pub enum OfferSaveResult {
    /// This means that the user chose to cancel the operation, but no changes were actually made.
    Cancel,
    /// This means that the user chose to discard the changes, but no changes were actually made.
    Discard,
    /// This means that the file was saved to disk (either using a pre-existing path or a path the user just chose).
    Saved,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    // TODO: Re-enable serde
    #[serde(skip)]
    model: Model,
    #[serde(skip)]
    view_options: ViewOptions,
}

impl App {
    /// Called once before the first frame.
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        open_file_path_o: Option<std::path::PathBuf>,
    ) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // // Load previous app state (if any).
        // // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        // use sept::dy::IntoValueT;
        // // let root_value = s1.into_value();
        // let root_value = sept::dy::ArrayTerm::from(vec![
        //     sept::dy::StructTerm::new(vec![
        //         ("name".to_string(), sept::st::Utf8String.into()),
        //         ("age".to_string(), sept::st::Uint8.into()),
        //     ])
        //     .unwrap()
        //     .into(),
        //     sept::dy::StructTerm::new(vec![("name".to_string(), sept::st::Utf8String.into())])
        //         .unwrap()
        //         .into(),
        //     sept::dy::StructTerm::new(vec![]).unwrap().into(),
        //     "".to_string().into(),
        //     "a".to_string().into(),
        //     "\n".to_string().into(),
        //     "xy\npq\n".to_string().into(),
        //     "hippos\nare\tabsolutely\nthe\nbest".to_string().into(),
        //     sept::dy::ArrayTerm::from(vec![
        //         sept::dy::ArrayTerm::from(vec![]).into(),
        //         "thingy".to_string().into(),
        //         "other\nthingy".to_string().into(),
        //     ])
        //     .into(),
        // ])
        // .into_value();

        // use sept::dy::IntoValueT;
        // let root_value = if let Some(open_file_path) = open_file_path_o.as_deref() {
        //     let mut file = std::fs::OpenOptions::new()
        //         .read(true)
        //         .open(open_file_path)
        //         .expect("TODO: handle this");
        //     use sept::st::DeserializableT;
        //     let root_value = sept::dy::Value::deserialize(&mut file).expect("TODO: handle this");
        //     cc.egui_ctx
        //         .send_viewport_cmd(egui::ViewportCommand::Title(format!(
        //             "SEPT - {}",
        //             open_file_path.display()
        //         )));
        //     root_value
        // } else {
        //     // Default is an empty array.
        //     sept::dy::ArrayTerm::from(vec![]).into_value()
        // };
        // let root_value_la = Arc::new(RwLock::new(root_value));
        // let model = Model {
        //     root_value_la,
        //     executed_command_v: VecDeque::new(),
        // };
        // // Start with the cursor on the root value.
        // let cursor_address = sept::dy::TupleTerm::from(vec![]);
        // let view_options = ViewOptions::default();

        // let local_symbol_table_la = Arc::new(RwLock::new(
        //     sept::dy::SymbolTable::new_without_parent("boring".to_string()).expect("test"),
        // ));

        // Self {
        //     model,
        //     cursor_address,
        //     view_options,
        //     local_symbol_table_la,
        //     open_file_path_o,
        // }

        let mut app = Self::default();
        if let Some(open_file_path) = open_file_path_o {
            app.file_open(Some(open_file_path), &cc.egui_ctx);
        }
        app
    }
    pub fn update_title(&self, ctx: &egui::Context) {
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(self.model.title()));
    }
    pub fn file_new(&mut self, ctx: &egui::Context) {
        #[cfg(target_arch = "wasm32")]
        {
            panic!("not implemented for wasm32 yet");
        }

        #[cfg(not(target_arch = "wasm32"))]
        match self.offer_save_if_unsaved(
            OFFER_SAVE_IF_UNSAVED_TITLE,
            "Save before creating new document?",
            ctx,
        ) {
            OfferSaveResult::Saved | OfferSaveResult::Discard => {
                self.model.clear();
                self.update_title(ctx);
            }
            OfferSaveResult::Cancel => {
                // Don't do anything.
            }
        }
    }
    pub fn file_open(&mut self, path_o: Option<std::path::PathBuf>, ctx: &egui::Context) {
        #[cfg(target_arch = "wasm32")]
        {
            panic!("not implemented for wasm32 yet");
        }

        #[cfg(not(target_arch = "wasm32"))]
        match self.offer_save_if_unsaved(
            OFFER_SAVE_IF_UNSAVED_TITLE,
            "Save before opening different document?",
            ctx,
        ) {
            OfferSaveResult::Saved | OfferSaveResult::Discard => {
                if let Some(path) = path_o {
                    self.model.open(path);
                    self.update_title(ctx);
                } else if let Some(path) = rfd::FileDialog::new()
                    .set_title("Open")
                    .add_filter("SEPT Files (*.sept)", &["sept"])
                    .pick_file()
                {
                    // This Model::clear is to ensure there are no unsaved changes, since Model::open
                    // will panic if there are any.
                    self.model.clear();
                    self.model.open(path);
                    self.update_title(ctx);
                }
            }
            OfferSaveResult::Cancel => {
                // Don't do anything.
            }
        }
    }
    pub fn file_save(&mut self, ctx: &egui::Context) {
        #[cfg(target_arch = "wasm32")]
        {
            panic!("not implemented for wasm32 yet");
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            if self.model.open_file_path_o().is_none() {
                if let Some(path) = rfd::FileDialog::new()
                    .set_title("Save As")
                    .add_filter("SEPT Files (*.sept)", &["sept"])
                    .save_file()
                {
                    self.model.save_as(path);
                }
            } else {
                self.model.save(SaveBehavior::OnlyIfChanged);
            }
            self.update_title(ctx);
        }
    }
    pub fn file_save_as(&mut self, ctx: &egui::Context) {
        #[cfg(target_arch = "wasm32")]
        {
            panic!("not implemented for wasm32 yet");
        }

        #[cfg(not(target_arch = "wasm32"))]
        if let Some(path) = rfd::FileDialog::new()
            .set_title("Save As")
            .add_filter("SEPT Files (*.sept)", &["sept"])
            .save_file()
        {
            self.model.save_as(path);
            self.update_title(ctx);
        }
    }
    pub fn file_quit(&mut self, ctx: &egui::Context) {
        #[cfg(target_arch = "wasm32")]
        {
            panic!("not implemented for wasm32 yet");
        }

        #[cfg(not(target_arch = "wasm32"))]
        match self.offer_save_if_unsaved(OFFER_SAVE_IF_UNSAVED_TITLE, "Save before quitting?", ctx)
        {
            OfferSaveResult::Saved | OfferSaveResult::Discard => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            OfferSaveResult::Cancel => {
                // Don't quit.
            }
        }
    }
    fn offer_save_if_unsaved(
        &mut self,
        title: &str,
        description: &str,
        ctx: &egui::Context,
    ) -> OfferSaveResult {
        if self.model.has_unsaved_changes() {
            tracing::debug!("App::offer_save_if_unsaved -- Model has unsaved changes");
            let message_dialog_result = rfd::MessageDialog::new()
                .set_title(title)
                .set_description(description)
                .set_buttons(rfd::MessageButtons::YesNoCancelCustom(
                    "Save".to_string(),
                    "Discard".to_string(),
                    "Cancel".to_string(),
                ))
                .show();
            tracing::debug!(
                "App::offer_save_if_unsaved -- MessageDialog result: {:?}",
                message_dialog_result
            );
            match message_dialog_result {
                rfd::MessageDialogResult::Custom(message) => {
                    match message.as_str() {
                        "Save" => {
                            // Save before quitting.
                            #[cfg(not(target_arch = "wasm32"))]
                            if self.model.open_file_path_o().is_none() {
                                if let Some(path) = rfd::FileDialog::new()
                                    .set_title("Save As")
                                    .add_filter("SEPT Files (*.sept)", &["sept"])
                                    .save_file()
                                {
                                    self.model.save_as(path);
                                    OfferSaveResult::Saved
                                } else {
                                    OfferSaveResult::Cancel
                                }
                            } else {
                                self.model.save(SaveBehavior::OnlyIfChanged);
                                OfferSaveResult::Saved
                            }
                        }
                        "Discard" => OfferSaveResult::Discard,
                        "Cancel" => OfferSaveResult::Cancel,
                        _ => {
                            panic!("programmer error: this should not happen");
                        }
                    }
                }
                _ => {
                    panic!("programmer error: this should not happen");
                }
            }
        } else {
            OfferSaveResult::Saved
        }
    }
    /// Handles top-level events, returning any unhandled events as Some(event), or
    /// None if the event was handled.
    pub fn handle_top_level_event(
        &mut self,
        event: egui::Event,
        remaining_event_v: &mut VecDeque<egui::Event>,
    ) -> Option<egui::Event> {
        // tracing::trace!("App::handle_top_level_event; event: {:?}", event);
        match event {
            egui::Event::Key {
                key: egui::Key::Equals,
                pressed: true,
                modifiers: egui::Modifiers::ALT,
                ..
            }
            | egui::Event::Key {
                key: egui::Key::Plus,
                pressed: true,
                modifiers: egui::Modifiers::ALT,
                ..
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
                ..
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
    /// Returns ordered sequence of unhandled events.
    fn handle_input_events(&mut self, mut event_v: VecDeque<egui::Event>) -> Vec<egui::Event> {
        let mut unhandled_event_v = Vec::new();
        while !event_v.is_empty() {
            let event = event_v.pop_front().unwrap();
            if matches!(event, egui::Event::Key { .. }) {
                tracing::debug!("App::handle_input_events; event: {:?}", event);
            } else {
                tracing::trace!("App::handle_input_events; event: {:?}", event);
            }

            // Handle undo/redo actions first.
            let event = match event {
                egui::Event::Key {
                    key: egui::Key::Z,
                    pressed: true,
                    modifiers,
                    ..
                } if modifiers.command => {
                    // TODO: Handle error, e.g. a sound or a status bar message indicating that
                    // there are no actions to undo/redo.
                    if modifiers.shift {
                        self.model.redo_action();
                    } else {
                        self.model.undo_action();
                    }
                    // We handled the event.
                    continue;
                }
                _ => event,
            };

            // Give the app a chance to handle top-level events.
            // TODO: handle_top_level_event is really only handling events that affect the ViewOptions, so rename this method accordingly.
            let event =
                if let Some(unhandled_event) = self.handle_top_level_event(event, &mut event_v) {
                    unhandled_event
                } else {
                    continue;
                };

            // Finally, give the model a chance to handle the event.
            let unhandled_event_o = self
                .model
                .handle_event(event, &mut event_v, &self.view_options)
                .expect("TODO: handle error");

            // Store any unhandled events for the next pass.
            if let Some(unhandled_event) = unhandled_event_o {
                // Pass the event on to unhandled_event_v.
                unhandled_event_v.push(unhandled_event);
            }
        }
        unhandled_event_v
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);

        // TODO: Save all unsaved documents to their backup files before quitting.
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            let mut file_new = false;
            let mut file_open = false;
            let mut file_save = false;
            let mut file_save_as = false;
            let mut file_quit = false;

            // Check keyboard shortcuts.
            ui.input_mut(|input_state| {
                if input_state.consume_key(Modifiers::CTRL, egui::Key::N) {
                    file_new = true;
                }
                if input_state.consume_key(Modifiers::CTRL, egui::Key::O) {
                    file_open = true;
                }
                // NOTE: This has to be done before the check for Ctrl+S due to some caveat in consume_key (see its docs).
                if input_state.consume_key(Modifiers::CTRL | Modifiers::SHIFT, egui::Key::S) {
                    file_save_as = true;
                }
                if input_state.consume_key(Modifiers::CTRL, egui::Key::S) {
                    file_save = true;
                }
                if input_state.consume_key(Modifiers::CTRL, egui::Key::Q) {
                    file_quit = true;
                }
            });

            tracing::trace!(
                "App::update; new: {}, open: {}, save: {}, save_as: {}, quit: {}",
                file_new,
                file_open,
                file_save,
                file_save_as,
                file_quit,
            );

            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    // TODO: Impl open/save for wasm.
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        if ui.button("New").clicked() || file_new {
                            file_new = true;
                            ui.close_menu();
                        }
                        if ui.button("Open").clicked() || file_open {
                            file_open = true;
                            ui.close_menu();
                        }
                        if ui.button("Save").clicked() || file_save {
                            file_save = true;
                            ui.close_menu();
                        }
                        if ui.button("Save As").clicked() || file_save_as {
                            file_save_as = true;
                            ui.close_menu();
                        }
                        if ui.button("Quit").clicked() || file_quit {
                            file_quit = true;
                            ui.close_menu();
                        }
                    }

                    #[cfg(target_arch = "wasm32")]
                    {
                        let _ = frame;
                        let _ = ui;
                    }
                });
            });

            if file_new {
                self.file_new(ctx);
            }
            if file_open {
                self.file_open(None, ctx);
            }
            if file_save {
                self.file_save(ctx);
            }
            if file_save_as {
                self.file_save_as(ctx);
            }
            if file_quit {
                self.file_quit(ctx);
            }
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            // Render the cursor address.  Unfortunately because this has to be rendered before
            // the CentralPanel, this gets updated with a slight delay after the events that change
            // the cursor address.
            ui.horizontal(|ui| {
                ui.label("Cursor Address:");

                // Create a model for the cursor address.
                let model = Model::new(
                    Arc::new(RwLock::new(self.model.cursor_address().clone().into())),
                    sept::dy::TupleTerm::from(vec![]),
                    None,
                );
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
                    let layout_job = self.model.cursor_address().run_ui(ui, &mut view_ctx, None);
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
                        .range(6.0..=30.0)
                        .max_decimals(0)
                        .suffix("pt")
                        .speed(0.0625),
                );
                egui::ComboBox::from_id_salt("font family combobox")
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

            let has_unsaved_changes_before = self.model.has_unsaved_changes();

            // TODO: Handle all keyboard/text/paste (and possibly other) events for the cursor before
            // starting the render, as these can change the cursor and therefore what should be rendered.
            // Technically mouse events can also change the cursor, but I'm not sure how that can
            // possibly be decoupled using an immediate mode GUI, unless maybe you can guarantee that no
            // more than one cursor-changing mouse event is received at a time.
            ui.input_mut(|input_state| {
                let mut event_v = std::mem::take(&mut input_state.events)
                    .into_iter()
                    .collect::<VecDeque<_>>();
                if !event_v.is_empty() {
                    tracing::trace!("App::update; {} events:", event_v.len());
                    for event in event_v.iter() {
                        tracing::trace!("    {:?}", event);
                    }
                }
                let unhandled_event_v = self.handle_input_events(event_v);
                // Pass all unhandled events through to the UI's InputState, so that they can be
                // handled on the UI pass.
                input_state.events = unhandled_event_v;
            });

            if self.model.has_unsaved_changes() != has_unsaved_changes_before {
                self.update_title(ctx);
            }

            egui::ScrollArea::vertical()
                // .always_show_scroll(true)
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    self.model.run_ui(ui, &self.view_options);
                });
        });
    }
}

impl Default for App {
    fn default() -> Self {
        let model = Model::default();
        // Start with the cursor on the root value.
        let view_options = ViewOptions::default();

        Self {
            model,
            view_options,
        }
    }
}

// impl Default for App {
//     fn default() -> Self {
//         let a1 = sept::dy::ArrayTerm::from(vec![
//             true.into(),
//             false.into(),
//             sept::dy::ArrayTerm::from(vec![]).into(),
//             123i8.into(),
//             200u8.into(),
//             12345i16.into(),
//             45678u16.into(),
//             1234567i32.into(),
//             4567890u32.into(),
//             1000000000000i64.into(),
//             9223372036854775808u64.into(),
//             10101.202f32.into(),
//             1.01020304050607f64.into(),
//             sept::st::Void.into(),
//             sept::st::True.into(),
//             sept::st::False.into(),
//             sept::st::EmptyType.into(),
//             sept::st::Void.into(),
//             sept::st::Void.into(),
//             sept::st::Void.into(),
//             sept::st::Void.into(),
//             sept::st::Void.into(),
//             sept::st::Void.into(),
//             sept::st::Void.into(),
//             sept::st::Void.into(),
//             sept::st::Void.into(),
//             sept::st::Void.into(),
//             sept::st::Void.into(),
//             sept::st::Void.into(),
//         ]);
//         let a2 =
//             sept::dy::ArrayTerm::from(vec![true.into(), 606i32.into(), a1.into(), false.into()]);

//         let m0 = sept::dy::OrderedMapTerm::from(maplit::btreemap! {});
//         let m1 = sept::dy::OrderedMapTerm::from(
//             maplit::btreemap! { 3i32.into() => "blah".to_string().into(), 5.5f32.into() => sept::st::Void.into() },
//         );
//         let m2 = sept::dy::OrderedMapTerm::from(maplit::btreemap! {
//             sept::dy::OrderedMapTerm::from(maplit::btreemap! { false.into() => 123u32.into() }).into() => 505.606f64.into(),
//             true.into() => sept::st::Void.into(),
//             sept::st::Bool.into() => sept::dy::OrderedMapTerm::from(
//                 maplit::btreemap! { 3i32.into() => "blah".to_string().into(), 5.5f32.into() => sept::st::Void.into() },
//             ).into()
//         });

//         let t1 = sept::dy::TupleTerm::from(vec![
//             true.into(),
//             false.into(),
//             123i8.into(),
//             sept::dy::TupleTerm::from(vec![
//                 200u8.into(),
//                 sept::dy::TupleTerm::from(vec![
//                     12345i16.into(),
//                     45678u16.into(),
//                     1234567i32.into(),
//                 ])
//                 .into(),
//                 4567890u32.into(),
//                 1000000000000i64.into(),
//             ])
//             .into(),
//             9223372036854775808u64.into(),
//             10101.202f32.into(),
//             1.01020304050607f64.into(),
//         ]);

//         let t2 = sept::dy::TupleTerm::from(vec![
//             sept::st::Void.into(),
//             sept::st::VoidType.into(),
//             sept::st::Bool.into(),
//             sept::st::BoolType.into(),
//             "blah\nthing\tWAAAA\tXyz\n\n!!!\rx".to_string().into(),
//         ]);
//         let t3 = sept::dy::TupleTerm::from(vec![
//             sept::st::Sint32.into(),
//             sept::st::Utf8String.into(),
//             sept::st::Array.into(),
//         ]);

//         let st1 = sept::dy::StructTerm::new(
//             vec![
//                 ("age".into(), sept::st::Uint8.into()),
//                 ("gravity".into(), sept::st::Float64.into()),
//                 ("thingy".into(), t3.into()),
//             ]
//             .into(),
//         )
//         .unwrap();

//         let st0 = sept::dy::StructTerm::new(vec![].into()).unwrap();

//         use sept::dy::ConstructorT;

//         // Make an empty StructTermTerm
//         let stt0 = st0.construct(sept::dy::TupleTerm::from(vec![])).unwrap();

//         let stt1 = st1
//             .construct(sept::dy::TupleTerm::from(vec![
//                 28u8.into(),
//                 4035.56f64.into(),
//                 sept::dy::TupleTerm::from(vec![
//                     445566i32.into(),
//                     "OSTRICH".to_string().into(),
//                     sept::dy::ArrayTerm::from(vec![]).into(),
//                 ])
//                 .into(),
//             ]))
//             .unwrap();

//         let s0 = String::new();
//         let s1 = "+++ one day, a big hippo came along and wrecked\teverything.\nyes, i mean absolutely everything!\nthere was nothing left.\n\n\tnothing left but hippos.".to_string();
//         // Very long string.
//         let s2 = "blah blah blahhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh blah blah blahhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh blah blah blahhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh blah blah blahhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh blah blah blahhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh blah blah blahhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh blah blah blahhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh blah blah blahhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh".to_string();

//         let gsr0 = sept::dy::GlobalSymRefTerm::new_unchecked("Hippo".to_string());
//         // NOTE: This uses the dereferenced StructTerm as the type and not the GlobalSymRefTerm as intended.
//         // TODO: Figure out how to get it to construct correctly.
//         let stt2 = gsr0
//             .construct(sept::dy::TupleTerm::from(vec![
//                 28u8.into(),
//                 4035.56f64.into(),
//             ]))
//             .unwrap();
//         // Manually construct the StructTermTerm so that the GlobalSymRefTerm is used as its type.
//         let stt3 = sept::dy::StructTermTerm::new_checked(
//             gsr0.clone().into(),
//             sept::dy::TupleTerm::from(vec![28u8.into(), 4035.56f64.into()]),
//         )
//         .unwrap();

//         let local_symbol_table_la = Arc::new(RwLock::new(
//             sept::dy::SymbolTable::new_without_parent("fancy".to_string()).expect("test"),
//         ));
//         local_symbol_table_la
//             .write()
//             .unwrap()
//             .define_symbol("blah", sept::dy::Value::from(1230321i32))
//             .expect("test");
//         tracing::debug!(
//             "local_symbol_table_la: {:#?}",
//             local_symbol_table_la.read().unwrap()
//         );

//         let lsr0 =
//             sept::dy::LocalSymRefTerm::new_checked(local_symbol_table_la.clone(), "blah".into())
//                 .expect("test");

//         let gsr1 =
//             sept::dy::GlobalSymRefTerm::new_unchecked("this one doesn't resolve".to_string());
//         let lsr1 = sept::dy::LocalSymRefTerm::new_unchecked(
//             local_symbol_table_la.clone(),
//             "also doesn't resolve".into(),
//         );

//         // TEMP HACK
//         // let value: sept::dy::Value = sept::dy::ArrayTerm::from(vec![s1.into()]).into();
//         // let value: sept::dy::Value = s1.into();

//         // let root_value: sept::dy::Value = sept::dy::ArrayTerm::from(vec![
//         //     a2.into(),
//         //     sept::dy::ArrayTerm::from(vec![]).into(),
//         //     m0.into(),
//         //     m1.into(),
//         //     m2.into(),
//         //     t1.into(),
//         //     t2.into(),
//         //     sept::dy::TupleTerm::from(vec![]).into(),
//         //     st0.into(),
//         //     st1.into(),
//         //     stt0.into(),
//         //     stt1.into(),
//         //     s0.into(),
//         //     s1.into(),
//         //     s2.into(),
//         //     gsr0.into(),
//         //     stt2.into(),
//         //     stt3.into(),
//         //     lsr0.into(),
//         //     gsr1.into(),
//         //     lsr1.into(),
//         // ])
//         // .into();
//         use sept::dy::IntoValueT;
//         // let root_value = s1.into_value();
//         let root_value = sept::dy::ArrayTerm::from(vec![
//             sept::dy::StructTerm::new(vec![
//                 ("name".to_string(), sept::st::Utf8String.into()),
//                 ("age".to_string(), sept::st::Uint8.into()),
//             ])
//             .unwrap()
//             .into(),
//             sept::dy::StructTerm::new(vec![("name".to_string(), sept::st::Utf8String.into())])
//                 .unwrap()
//                 .into(),
//             sept::dy::StructTerm::new(vec![]).unwrap().into(),
//             "".to_string().into(),
//             "a".to_string().into(),
//             "\n".to_string().into(),
//             "xy\npq\n".to_string().into(),
//             "hippos\nare\tabsolutely\nthe\nbest".to_string().into(),
//             sept::dy::ArrayTerm::from(vec![
//                 sept::dy::ArrayTerm::from(vec![]).into(),
//                 "thingy".to_string().into(),
//                 "other\nthingy".to_string().into(),
//             ])
//             .into(),
//         ])
//         .into_value();
//         let root_value_la = Arc::new(RwLock::new(root_value));
//         let model = Model {
//             root_value_la,
//             executed_command_v: VecDeque::new(),
//         };
//         // Start with the cursor on the root value.
//         let cursor_address = sept::dy::TupleTerm::from(vec![]);
//         let view_options = ViewOptions::default();

//         Self {
//             model,
//             cursor_address,
//             view_options,
//             local_symbol_table_la,
//             open_file_path_o: None,
//         }
//     }
// }

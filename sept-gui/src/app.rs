use crate::{View, ViewCtx};
use std::sync::{Arc, RwLock};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    #[serde(skip)]
    value: sept::dy::Value,
    #[serde(skip)]
    view_ctx: ViewCtx,
    #[serde(skip)]
    local_symbol_table_la: Arc<RwLock<sept::dy::SymbolTable>>,
}

impl Default for App {
    fn default() -> Self {
        let a1 = sept::dy::ArrayTerm::from(vec![
            true.into(),
            false.into(),
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

        let value: sept::dy::Value = sept::dy::ArrayTerm::from(vec![
            a2.into(),
            sept::dy::ArrayTerm::from(vec![]).into(),
            t1.into(),
            t2.into(),
            sept::dy::TupleTerm::from(vec![]).into(),
            st0.into(),
            st1.into(),
            stt1.into(),
            s0.into(),
            s1.into(),
            gsr0.into(),
            stt2.into(),
            stt3.into(),
            lsr0.into(),
            gsr1.into(),
            lsr1.into(),
        ])
        .into();

        let mut view_ctx = ViewCtx::new();
        view_ctx.inline_at_nesting_depth = 2;

        Self {
            value,
            view_ctx,
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("Inlining Depth:");
                ui.add(
                    egui::DragValue::new(&mut self.view_ctx.inline_at_nesting_depth).speed(0.0625),
                );

                ui.checkbox(
                    &mut self.view_ctx.show_expanded_item_indicator,
                    "Expanded Item Indicator",
                );

                ui.checkbox(&mut self.view_ctx.show_type_annotations, "Type Annotations");

                ui.checkbox(
                    &mut self.view_ctx.show_struct_field_name_hints,
                    "Struct Field Name Hints",
                );

                ui.label("Font:");
                ui.add(
                    egui::DragValue::new(&mut self.view_ctx.font_id.size)
                        .clamp_range(6.0..=30.0)
                        .max_decimals(0)
                        .suffix("pt")
                        .speed(0.0625),
                );
                egui::ComboBox::from_id_source("font family combobox")
                    .selected_text(format!("{:?}", &mut self.view_ctx.font_id.family))
                    .show_ui(ui, |ui| {
                        // ui.style_mut().wrap = Some(false);
                        ui.set_min_width(60.0);
                        ui.selectable_value(
                            &mut self.view_ctx.font_id.family,
                            egui::FontFamily::Monospace,
                            "Monospace",
                        );
                        ui.selectable_value(
                            &mut self.view_ctx.font_id.family,
                            egui::FontFamily::Proportional,
                            "Proportional",
                        );
                    });

                egui::warn_if_debug_build(ui);
            });
        });

        // Note that the CentralPanel must be added after side panels.
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .always_show_scroll(true)
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    let old_item_spacing = ui.spacing().item_spacing;
                    ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

                    ui.vertical(|ui| {
                        let layout_job = self.value.update(ui, &mut self.view_ctx, None);
                        ui.label(layout_job);
                    });

                    ui.spacing_mut().item_spacing = old_item_spacing;
                });
        });

        // if false {
        //     egui::Window::new("Window").show(ctx, |ui| {
        //         ui.label("Windows can be moved by dragging them.");
        //         ui.label("They are automatically sized based on contents.");
        //         ui.label("You can turn on resizing and scrolling if you like.");
        //         ui.label("You would normally chose either panels OR windows.");
        //     });
        // }
    }
}

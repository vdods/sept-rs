use crate::{View, ViewCtx};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    #[serde(skip)]
    value: sept::dy::Value,
    #[serde(skip)]
    view_ctx: ViewCtx,
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
        let a2 = sept::dy::ArrayTerm::from(vec![true.into(), a1.into(), false.into()]);

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
            "blah\nthing\they".to_string().into(),
        ]);

        let st1 = sept::dy::StructTerm::new(
            vec![
                ("age".into(), sept::st::Uint8.into()),
                ("gravity".into(), sept::st::Float64.into()),
            ]
            .into(),
        )
        .unwrap();

        use sept::dy::Constructor;
        let stt1 = st1
            .construct(sept::dy::TupleTerm::from(vec![
                28u8.into(),
                4035.56f64.into(),
            ]))
            .unwrap();

        let value: sept::dy::Value = sept::dy::ArrayTerm::from(vec![
            a2.into(),
            t1.into(),
            t2.into(),
            st1.into(),
            stt1.into(),
        ])
        .into();

        let mut view_ctx = ViewCtx::new();
        view_ctx.inline_at_nesting_depth = 2;

        Self { value, view_ctx }
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
        // let Self { label, value } = self;

        // #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
            });
        });

        egui::SidePanel::right("side_panel").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.add(
                    egui::DragValue::new(&mut self.view_ctx.inline_at_nesting_depth).speed(0.0625),
                );
                ui.label("Inline Depth");
            });

            ui.checkbox(
                &mut self.view_ctx.show_type_annotations,
                "Show Type Annotations",
            );

            egui::warn_if_debug_build(ui);
        });

        // Note that the CentralPanel must be added after side panels.
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .always_show_scroll(true)
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    let old_item_spacing = ui.spacing().item_spacing;
                    ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

                    self.value.update(ui, &mut self.view_ctx);

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

#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    {
        // Create the Hippo struct
        sept::dy::GLOBAL_SYMBOL_TABLE_LA
            .write()
            .unwrap()
            .define_symbol(
                "Hippo",
                sept::dy::StructTerm::new(
                    vec![
                        ("size".into(), sept::st::Uint8.into()),
                        ("awesomeness".into(), sept::st::Float64.into()),
                    ]
                    .into(),
                )
                .expect("test")
                .into(),
            )
            .expect("test");

        let global_symbol_table_g = sept::dy::GLOBAL_SYMBOL_TABLE_LA.read().unwrap();
        tracing::debug!("global_symbol_table_g: {:#?}", global_symbol_table_g);
        let hippo = sept::dy::GlobalSymRefTerm::new_unchecked("Hippo".into());
        use sept::st::Stringifiable;
        tracing::debug!("hippo: {}", hippo.stringify());
    }

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "SEPT",
        native_options,
        Box::new(|cc| Box::new(sept_gui::App::new(cc))),
    );
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|cc| Box::new(sept_gui::App::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}

#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    // Parse command line arguments, if any.
    let arg_v: Vec<String> = std::env::args().collect();
    tracing::info!("Command line arguments: {:?}", arg_v);
    let open_file_path_o = if arg_v.len() == 2 {
        Some(std::path::PathBuf::from(&arg_v[1]))
    } else {
        None
    };

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
        Box::new(|cc| Ok(Box::new(sept_gui::App::new(cc, open_file_path_o)))),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

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

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(sept_gui::App::new(cc, None)))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}

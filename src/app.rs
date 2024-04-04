/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state

// fn setup_custom_fonts(ctx: &egui::Context) {
//     // Start with the default fonts (we will be adding to them rather than replacing them).
//     // let mut fonts = egui::FontDefinitions::default();

//     // Install my own font (maybe supporting non-latin characters).
//     // .ttf and .otf files supported.
//     // fonts.font_data.insert(
//     //     "my_font".to_owned(),
//     //     egui::FontData::from_static(include_bytes!(
//     //         "../../../crates/epaint/fonts/Hack-Regular.ttf"
//     //     )),
//     // );

//     // // Put my font first (highest priority) for proportional text:
//     // fonts
//     //     .families
//     //     .entry(egui::FontFamily::Proportional)
//     //     .or_default()
//     //     .insert(0, "my_font".to_owned());

//     // // Put my font as last fallback for monospace:
//     // fonts
//     //     .families
//     //     .entry(egui::FontFamily::Monospace)
//     //     .or_default()
//     //     .push("my_font".to_owned());

//     // // Tell egui to use these fonts:
//     // ctx.set_fonts(fonts);
// }

pub struct MyApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl MyApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.

        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles = [
            (egui::TextStyle::Heading, egui::FontId::new(30.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Name("Heading1".into()), egui::FontId::new(60.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Name("Heading2".into()), egui::FontId::new(30.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Body, egui::FontId::new(18.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Monospace, egui::FontId::new(14.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Button, egui::FontId::new(14.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Small, egui::FontId::new(10.0, egui::FontFamily::Proportional)),
        ]
        .into();
        cc.egui_ctx.set_style(style);

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for MyApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Name("Heading2".into()));
                    egui::widgets::global_dark_light_mode_buttons(ui);
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.vertical_centered(|ui| {
                ui.style_mut().override_text_style = Some(egui::TextStyle::Name("Heading1".into()));
                ui.label("Austin Liu");
             });
            ui.vertical_centered(|ui| {
                ui.style_mut().override_text_style = Some(egui::TextStyle::Name("Heading2".into()));
                ui.hyperlink_to("Github", "https://www.youtube.com/watch?v=dQw4w9WgXcQ");
                ui.hyperlink_to("Linkedin", "https://www.youtube.com/watch?v=dQw4w9WgXcQ");
                ui.hyperlink_to("Resume", "https://www.youtube.com/watch?v=dQw4w9WgXcQ");
             });
            // ui.horizontal(|ui| {
            //     ui.label("Write something: ");
            //     ui.text_edit_singleline(&mut self.label);
            // });

            // ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            // if ui.button("Increment").clicked() {
            //     self.value += 1.0;
            // }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.add(egui::github_link_file!(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
            "Source code."
        ));
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

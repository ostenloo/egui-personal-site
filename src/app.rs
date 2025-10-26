/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, PartialEq, Eq, Debug)]
enum Page {
    Home,
    Projects,
}

impl Page {
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn from_path(path: &str) -> Self {
        let normalized = path.trim();
        let normalized = normalized.trim_start_matches('/');
        let normalized = normalized.split('/').next().unwrap_or("");
        match normalized {
            "" => Page::Home,
            "projects" => Page::Projects,
            _ => Page::Home,
        }
    }

    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn path(&self) -> &'static str {
        match self {
            Page::Home => "/",
            Page::Projects => "/projects",
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct MyApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    current_page: Page,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            current_page: Page::Home,
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
            (
                egui::TextStyle::Heading,
                egui::FontId::new(30.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Name("Heading1".into()),
                egui::FontId::new(60.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Name("Heading2".into()),
                egui::FontId::new(30.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Body,
                egui::FontId::new(18.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Monospace,
                egui::FontId::new(14.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Button,
                egui::FontId::new(14.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Small,
                egui::FontId::new(10.0, egui::FontFamily::Proportional),
            ),
        ]
        .into();
        cc.egui_ctx.set_style(style);

        let mut app: Self = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        app.pull_route_from_browser();
        app
    }
}

impl eframe::App for MyApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.pull_route_from_browser();

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

                ui.horizontal(|ui| {
                    let previous = self.current_page;
                    ui.style_mut().override_text_style =
                        Some(egui::TextStyle::Name("Heading2".into()));
                    ui.selectable_value(&mut self.current_page, Page::Home, "Home");
                    ui.selectable_value(&mut self.current_page, Page::Projects, "Projects");
                    if previous != self.current_page {
                        self.push_route_to_browser();
                    }
                });
                ui.add_space(16.0);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui.style_mut().override_text_style =
                        Some(egui::TextStyle::Name("Heading2".into()));
                    egui::widgets::global_dark_light_mode_buttons(ui);
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            match self.current_page {
                Page::Home => self.show_home(ui),
                Page::Projects => self.show_projects(ui),
            }
        });
    }
}

impl MyApp {
    fn show_home(&self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.style_mut().override_text_style = Some(egui::TextStyle::Name("Heading1".into()));
            ui.label("Austin Liu");
        });
        ui.vertical_centered(|ui| {
            ui.style_mut().override_text_style = Some(egui::TextStyle::Name("Heading2".into()));
            ui.hyperlink_to("Github", "https://github.com/ostenloo");
            ui.hyperlink_to("Linkedin", "https://www.linkedin.com/in/austindasunliu/");
            ui.hyperlink_to("Resume", "https://drive.google.com/file/d/18TzUzxpuevB1W5LIDFtFhrw2rXruR1Hd/view?usp=sharing");
        });

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            powered_by_egui_and_eframe(ui);
            egui::warn_if_debug_build(ui);
        });
    }

    fn show_projects(&self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.style_mut().override_text_style = Some(egui::TextStyle::Name("Heading1".into()));
            ui.label("Projects");
        });
        ui.add_space(32.0);

        // Project entries with clean formatting, no borders, center-aligned
        ui.vertical_centered(|ui| {
            ui.spacing_mut().item_spacing.y = 24.0; // More space between projects

            // Zeitgus
            ui.vertical_centered(|ui| {
                ui.style_mut().override_text_style = Some(egui::TextStyle::Name("Heading2".into()));
                ui.hyperlink_to("Zeitgus", "https://www.zeitgus.com");
                ui.style_mut().override_text_style = Some(egui::TextStyle::Body);
                ui.label("I tried to build a startup in college, all I have is a landing page to show for it.");
                ui.add_space(8.0);
            });

            // FIDE ratings database
            ui.vertical_centered(|ui| {
                ui.style_mut().override_text_style = Some(egui::TextStyle::Name("Heading2".into()));
                ui.hyperlink_to("FIDE Ratings Database", "https://console.cloud.google.com/bigquery?ws=!1m5!1m4!4m3!1scalm-premise-334401!2sFIDE_ratings!3sFIDE_ratings_2");
                ui.style_mut().override_text_style = Some(egui::TextStyle::Body);
                ui.label("Chess ratings analysis using Python and Google BigQuery");
                ui.add_space(8.0);
            });

            // Rusty graph coloring
            ui.vertical_centered(|ui| {
                ui.style_mut().override_text_style = Some(egui::TextStyle::Name("Heading2".into()));
                ui.hyperlink_to("Rusty Graph Coloring", "https://github.com/ostenloo/rusty-graph-coloring");
                ui.style_mut().override_text_style = Some(egui::TextStyle::Body);
                ui.label("Graph coloring algorithms implemented in Rust");
                ui.add_space(8.0);
            });

            // Personal Site
            ui.vertical_centered(|ui| {
                ui.style_mut().override_text_style = Some(egui::TextStyle::Name("Heading2".into()));
                ui.hyperlink_to("Personal Site", "https://github.com/ostenloo/egui-personal-site");
                ui.style_mut().override_text_style = Some(egui::TextStyle::Body);
                ui.label("Built with egui and Rust WebAssembly");
            });
        });

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            powered_by_egui_and_eframe(ui);
            egui::warn_if_debug_build(ui);
        });
    }

    fn pull_route_from_browser(&mut self) {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(path) = current_pathname() {
                let new_page = Page::from_path(&path);
                if new_page != self.current_page {
                    self.current_page = new_page;
                }
            }
        }
    }

    fn push_route_to_browser(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            let desired_path = self.current_page.path();

            if let Some(current_path) = current_pathname() {
                if current_path == desired_path {
                    return;
                }
            }

            if let Some(window) = web_sys::window() {
                if let Ok(history) = window.history() {
                    let _ = history.push_state_with_url(
                        &wasm_bindgen::JsValue::NULL,
                        "",
                        Some(desired_path),
                    );
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn current_pathname() -> Option<String> {
    let window = web_sys::window()?;
    window.location().pathname().ok()
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

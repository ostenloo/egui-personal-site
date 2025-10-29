use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use include_dir::{include_dir, Dir};

static BLOG_POSTS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/blog_posts");

#[derive(Clone)]
pub struct BlogPost {
    pub title: String,
    pub date: String,
    pub content: String,
    pub summary: String,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, PartialEq, Eq, Debug)]
enum Page {
    Home,
    Projects,
    Blog,
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
            "blog" => Page::Blog,
            _ => Page::Home,
        }
    }

    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn path(&self) -> &'static str {
        match self {
            Page::Home => "/",
            Page::Projects => "/projects",
            Page::Blog => "/blog",
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
    
    #[serde(skip)] // Don't serialize blog posts and cache
    blog_posts: Vec<BlogPost>,
    selected_blog: Option<usize>,
    
    #[serde(skip)] // Don't serialize the cache
    markdown_cache: CommonMarkCache,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            current_page: Page::Home,
            blog_posts: Self::create_sample_blog_posts(),
            selected_blog: None,
            markdown_cache: CommonMarkCache::default(),
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
        
        // Apply Markdown Design System Typography
        style.text_styles = [
            // H1 - Primary document title
            (
                egui::TextStyle::Name("Heading1".into()),
                egui::FontId::new(34.0, egui::FontFamily::Proportional), // 32-36px range
            ),
            // H2 - Major section header  
            (
                egui::TextStyle::Name("Heading2".into()),
                egui::FontId::new(27.0, egui::FontFamily::Proportional), // 26-28px range
            ),
            // H3 - Subsection header
            (
                egui::TextStyle::Name("Heading3".into()),
                egui::FontId::new(23.0, egui::FontFamily::Proportional), // 22-24px range
            ),
            // Default heading (fallback)
            (
                egui::TextStyle::Heading,
                egui::FontId::new(27.0, egui::FontFamily::Proportional),
            ),
            // Paragraph - Default body text
            (
                egui::TextStyle::Body,
                egui::FontId::new(16.0, egui::FontFamily::Proportional),
            ),
            // Code / Preformatted - Monospace
            (
                egui::TextStyle::Monospace,
                egui::FontId::new(15.0, egui::FontFamily::Monospace),
            ),
            // Small / Metadata - Captions, notes
            (
                egui::TextStyle::Small,
                egui::FontId::new(14.0, egui::FontFamily::Proportional),
            ),
            // Button text
            (
                egui::TextStyle::Button,
                egui::FontId::new(16.0, egui::FontFamily::Proportional),
            ),
        ]
        .into();

        // Apply color palette from design system
        let mut style = (*cc.egui_ctx.style()).clone();
        
        // Apply spacing adjustments for better typography
        style.spacing.item_spacing = egui::vec2(8.0, 12.0);
        style.spacing.button_padding = egui::vec2(12.0, 8.0);
        
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
        // Only update style if hyperlink color needs to be green (to avoid constant style updates)
        if ctx.style().visuals.hyperlink_color != egui::Color32::from_rgb(22, 163, 74) {
            let mut style = (*ctx.style()).clone();
            style.visuals.hyperlink_color = egui::Color32::from_rgb(22, 163, 74); // Green color
            style.visuals.selection.bg_fill = egui::Color32::from_rgb(22, 163, 74).linear_multiply(0.2);
            style.visuals.selection.stroke.color = egui::Color32::from_rgb(22, 163, 74);
            ctx.set_style(style);
        }
        
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
                    ui.selectable_value(&mut self.current_page, Page::Blog, "Blog");
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
                Page::Blog => self.show_blog(ui),
            }
        });
    }
}

impl MyApp {
    fn show_home(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.style_mut().override_text_style = Some(egui::TextStyle::Name("Heading1".into()));
            ui.label("Austin Liu");
        });
        ui.add_space(16.0); // Design system spacing
        
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

    fn show_projects(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.style_mut().override_text_style = Some(egui::TextStyle::Name("Heading1".into()));
            ui.label("Projects");
        });
        ui.add_space(32.0); // Design system H1 bottom margin

        // Project entries with design system spacing
        ui.vertical_centered(|ui| {
            ui.spacing_mut().item_spacing.y = 32.0; // More space between projects

            // Zeitgus
            ui.vertical_centered(|ui| {
                ui.style_mut().override_text_style = Some(egui::TextStyle::Name("Heading2".into()));
                ui.hyperlink_to("Zeitgus", "https://www.zeitgus.com");
                ui.add_space(8.0);
                ui.style_mut().override_text_style = Some(egui::TextStyle::Body);
                ui.label("I tried to build a startup in college, all I have is a landing page to show for it.");
            });

            // FIDE ratings database
            ui.vertical_centered(|ui| {
                ui.style_mut().override_text_style = Some(egui::TextStyle::Name("Heading2".into()));
                ui.hyperlink_to("FIDE Ratings Database", "https://console.cloud.google.com/bigquery?ws=!1m5!1m4!4m3!1scalm-premise-334401!2sFIDE_ratings!3sFIDE_ratings_2");
                ui.add_space(8.0);
                ui.style_mut().override_text_style = Some(egui::TextStyle::Body);
                ui.label("Chess ratings analysis using Python and Google BigQuery");
            });

            // Rusty graph coloring
            ui.vertical_centered(|ui| {
                ui.style_mut().override_text_style = Some(egui::TextStyle::Name("Heading2".into()));
                ui.hyperlink_to("Rusty Graph Coloring", "https://github.com/ostenloo/rusty-graph-coloring");
                ui.add_space(8.0);
                ui.style_mut().override_text_style = Some(egui::TextStyle::Body);
                ui.label("Graph coloring algorithms implemented in Rust");
            });

            // Personal Site
            ui.vertical_centered(|ui| {
                ui.style_mut().override_text_style = Some(egui::TextStyle::Name("Heading2".into()));
                ui.hyperlink_to("Personal Site", "https://github.com/ostenloo/egui-personal-site");
                ui.add_space(8.0);
                ui.style_mut().override_text_style = Some(egui::TextStyle::Body);
                ui.label("Built with egui and Rust WebAssembly");
            });
        });

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            powered_by_egui_and_eframe(ui);
            egui::warn_if_debug_build(ui);
        });
    }

    fn show_blog(&mut self, ui: &mut egui::Ui) {
        if let Some(blog_index) = self.selected_blog {
            // Show individual blog post with design system styling
            if let Some(blog_post) = self.blog_posts.get(blog_index) {
                ui.horizontal(|ui| {
                    // Back button with proper styling
                    if ui.button("← Back to Blog List").clicked() {
                        self.selected_blog = None;
                    }
                    ui.add_space(24.0); // Design system spacing
                });
                
                ui.add_space(16.0);
                
                // Blog post title with H1 styling
                ui.vertical_centered(|ui| {
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Name("Heading1".into()));
                    ui.label(&blog_post.title);
                });
                
                ui.add_space(8.0);
                
                // Date with metadata styling
                ui.vertical_centered(|ui| {
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Small);
                    ui.colored_label(egui::Color32::from_rgb(85, 85, 85), &blog_post.date); // #555555 in RGB
                });
                
                ui.add_space(24.0); // Design system margin before content

                // Markdown content with proper spacing
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        // Set proper line height and spacing for markdown content
                        ui.spacing_mut().item_spacing.y = 16.0; // Design system paragraph spacing
                        CommonMarkViewer::new(&format!("blog_{}", blog_index))
                            .show(ui, &mut self.markdown_cache, &blog_post.content);
                    });
            }
        } else {
            // Show blog list with design system styling
            ui.vertical_centered(|ui| {
                ui.style_mut().override_text_style = Some(egui::TextStyle::Name("Heading1".into()));
                ui.label("Blog");
            });
            ui.add_space(24.0); // Design system spacing (1.5em equivalent)

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    for (index, blog_post) in self.blog_posts.iter().enumerate() {
                        // Blog post card with design system styling
                        ui.group(|ui| {
                            ui.set_width(ui.available_width());
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        // Post title with H2 styling
                                        ui.style_mut().override_text_style = Some(egui::TextStyle::Name("Heading2".into()));
                                        if ui.link(&blog_post.title).clicked() {
                                            self.selected_blog = Some(index);
                                        }
                                        
                                        ui.add_space(8.0);
                                        
                                        // Post summary with body text styling
                                        ui.style_mut().override_text_style = Some(egui::TextStyle::Body);
                                        ui.label(&blog_post.summary);
                                    });
                                    
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                        // Date with metadata styling
                                        ui.style_mut().override_text_style = Some(egui::TextStyle::Small);
                                        ui.colored_label(egui::Color32::from_rgb(85, 85, 85), &blog_post.date); // #555555 in RGB
                                    });
                                });
                            });
                        });
                        ui.add_space(24.0); // Design system spacing between posts
                    }
                });
        }

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            powered_by_egui_and_eframe(ui);
            egui::warn_if_debug_build(ui);
        });
    }

    fn create_sample_blog_posts() -> Vec<BlogPost> {
        let mut posts = Vec::new();
        
        // Automatically discover all .md files in the blog_posts directory
        for file in BLOG_POSTS_DIR.files() {
            if let Some(extension) = file.path().extension() {
                if extension == "md" {
                    if let Ok(content) = std::str::from_utf8(file.contents()) {
                        posts.push(Self::parse_blog_post(content));
                    }
                }
            }
        }
        
        // Sort posts by date (newest first)
        posts.sort_by(|a, b| b.date.cmp(&a.date));
        
        posts
    }

    fn parse_blog_post(content: &str) -> BlogPost {
        let (frontmatter, body) = if content.starts_with("---\n") {
            // Find the end of frontmatter
            if let Some(end_pos) = content[4..].find("\n---\n") {
                let frontmatter = &content[4..end_pos + 4];
                let body = &content[end_pos + 8..]; // Skip "---\n"
                (Some(frontmatter), body)
            } else {
                (None, content)
            }
        } else {
            (None, content)
        };

        let mut title = "Untitled".to_string();
        let mut date = "Unknown".to_string();
        let mut summary = "No summary available.".to_string();

        if let Some(fm) = frontmatter {
            for line in fm.lines() {
                let line = line.trim();
                if line.starts_with("title:") {
                    title = line[6..].trim().trim_matches('"').to_string();
                } else if line.starts_with("date:") {
                    date = line[5..].trim().trim_matches('"').to_string();
                } else if line.starts_with("summary:") {
                    summary = line[8..].trim().trim_matches('"').to_string();
                }
            }
        }

        BlogPost {
            title,
            date,
            summary,
            content: body.to_string(),
        }
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

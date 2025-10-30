use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use include_dir::{include_dir, Dir};

static BLOG_POSTS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/blog_posts");

#[derive(Clone)]
pub struct BlogPost {
    pub title: String,
    pub date_display: String,
    pub content: String,
    pub slug: String,
    pub published_at: DateTime<Utc>,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq, Debug)]
enum Page {
    Home,
    Projects,
    Blog,
    BlogPost(String),
}

impl Page {
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn from_path(path: &str) -> Self {
        let trimmed = path.trim();
        let trimmed = trimmed.trim_start_matches('/');

        if trimmed.is_empty() {
            return Page::Home;
        }

        let mut segments = trimmed.split('/');
        let first = segments.next();
        let second = segments.next();

        match (first, second) {
            (Some("projects"), _) => Page::Projects,
            (Some("blog"), None | Some("")) => Page::Blog,
            (Some("blog"), Some(slug)) => Page::BlogPost(slug.to_string()),
            _ => Page::Home,
        }
    }

    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn to_url(&self) -> String {
        match self {
            Page::Home => "/".to_string(),
            Page::Projects => "/projects".to_string(),
            Page::Blog => "/blog".to_string(),
            Page::BlogPost(slug) => format!("/blog/{}", slug),
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
    /// Calculate responsive margins based on screen width
    fn calculate_responsive_margins(screen_width: f32) -> (f32, f32) {
        if screen_width > 800.0 {
            // Large screen: generous margins for better reading
            let margin = (screen_width - 700.0) / 2.0;
            (margin.max(60.0), margin.max(60.0))
        } else if screen_width > 600.0 {
            // Medium screen: moderate margins
            (40.0, 40.0)
        } else {
            // Small screen: minimal margins, start from left edge
            (16.0, 16.0)
        }
    }

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

        // Apply spacing adjustments for better typography
        style.spacing.item_spacing = egui::vec2(8.0, 12.0);
        style.spacing.button_padding = egui::vec2(12.0, 8.0);

        cc.egui_ctx.set_style(style);

        let mut app: Self = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        // Rehydrate transient state that we intentionally skip during serialization.
        app.blog_posts = Self::create_sample_blog_posts();
        if let Some(selected) = app.selected_blog {
            if selected >= app.blog_posts.len() {
                app.selected_blog = None;
            }
        }
        app.markdown_cache = CommonMarkCache::default();

        app.pull_route_from_browser();
        app.sync_blog_selection_from_route();
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
            style.visuals.selection.bg_fill =
                egui::Color32::from_rgb(22, 163, 74).linear_multiply(0.2);
            style.visuals.selection.stroke.color = egui::Color32::from_rgb(22, 163, 74);
            ctx.set_style(style);
        }

        self.pull_route_from_browser();
        self.sync_blog_selection_from_route();

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
                    ui.style_mut().override_text_style =
                        Some(egui::TextStyle::Name("Heading2".into()));

                    let previous = self.current_page.clone();
                    let mut changed = false;

                    let select_nav = |ui: &mut egui::Ui, is_selected: bool, label: &str| -> bool {
                        ui.selectable_label(is_selected, label).clicked()
                    };

                    if select_nav(ui, matches!(self.current_page, Page::Home), "Home")
                        && !matches!(self.current_page, Page::Home)
                    {
                        self.current_page = Page::Home;
                        self.selected_blog = None;
                        changed = true;
                    }

                    if select_nav(ui, matches!(self.current_page, Page::Projects), "Projects")
                        && !matches!(self.current_page, Page::Projects)
                    {
                        self.current_page = Page::Projects;
                        self.selected_blog = None;
                        changed = true;
                    }

                    if select_nav(
                        ui,
                        matches!(self.current_page, Page::Blog | Page::BlogPost(_)),
                        "Blog",
                    ) && !matches!(self.current_page, Page::Blog)
                    {
                        self.current_page = Page::Blog;
                        self.selected_blog = None;
                        changed = true;
                    }

                    if changed && previous != self.current_page {
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
            match self.current_page.clone() {
                Page::Home => self.show_home(ui),
                Page::Projects => self.show_projects(ui),
                Page::Blog | Page::BlogPost(_) => self.show_blog(ui),
            }
        });
    }
}

impl MyApp {
    fn show_home(&mut self, ui: &mut egui::Ui) {
        ui.add_space(24.0); // Top margin

        // Calculate responsive margins
        let screen_width = ui.available_width();
        let (left_margin, right_margin) = Self::calculate_responsive_margins(screen_width);

        ui.horizontal(|ui| {
            ui.add_space(left_margin); // Responsive left margin

            ui.vertical(|ui| {
                // Calculate the content width by subtracting both margins
                let content_width = ui.available_width() - right_margin;
                ui.set_max_width(content_width);

                ui.style_mut().override_text_style = Some(egui::TextStyle::Name("Heading1".into()));
                ui.label("Austin Liu");

                ui.add_space(16.0); // Design system spacing

                ui.style_mut().override_text_style = Some(egui::TextStyle::Name("Heading2".into()));
                ui.hyperlink_to("Github", "https://github.com/ostenloo");
                ui.hyperlink_to("Linkedin", "https://www.linkedin.com/in/austindasunliu/");
                ui.hyperlink_to("Resume", "https://drive.google.com/file/d/18TzUzxpuevB1W5LIDFtFhrw2rXruR1Hd/view?usp=sharing");
            });
        });
    }

    fn show_projects(&mut self, ui: &mut egui::Ui) {
        ui.add_space(24.0); // Top margin

        // Calculate responsive margins
        let screen_width = ui.available_width();
        let (left_margin, right_margin) = Self::calculate_responsive_margins(screen_width);

        ui.horizontal(|ui| {
            ui.add_space(left_margin); // Responsive left margin

            ui.vertical(|ui| {
                // Calculate the content width by subtracting both margins
                let content_width = ui.available_width() - right_margin;
                ui.set_max_width(content_width);

                ui.style_mut().override_text_style = Some(egui::TextStyle::Name("Heading1".into()));
                ui.label("Projects");

                ui.add_space(32.0); // Design system H1 bottom margin

                // Project entries with design system spacing
                ui.spacing_mut().item_spacing.y = 32.0; // More space between projects

                // Zeitgus
                ui.vertical(|ui| {
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Name("Heading2".into()));
                    ui.hyperlink_to("Zeitgus", "https://www.zeitgus.com");
                    ui.add_space(8.0);
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Body);
                    ui.label("I tried to build a startup in college, all I have is a landing page to show for it.");
                });

                // FIDE ratings database
                ui.vertical(|ui| {
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Name("Heading2".into()));
                    ui.hyperlink_to("FIDE Ratings Database", "https://console.cloud.google.com/bigquery?ws=!1m5!1m4!4m3!1scalm-premise-334401!2sFIDE_ratings!3sFIDE_ratings_2");
                    ui.add_space(8.0);
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Body);
                    ui.label("Chess ratings analysis using Python and Google BigQuery");
                });

                // Rusty graph coloring
                ui.vertical(|ui| {
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Name("Heading2".into()));
                    ui.hyperlink_to("Rusty Graph Coloring", "https://github.com/ostenloo/rusty-graph-coloring");
                    ui.add_space(8.0);
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Body);
                    ui.label("Graph coloring algorithms implemented in Rust");
                });

                // Personal Site
                ui.vertical(|ui| {
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Name("Heading2".into()));
                    ui.hyperlink_to("Personal Site", "https://github.com/ostenloo/egui-personal-site");
                    ui.add_space(8.0);
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Body);
                    ui.label("Built with egui and Rust WebAssembly");
                });
            });
        });
    }

    fn show_blog(&mut self, ui: &mut egui::Ui) {
        if let Some(blog_index) = self.selected_blog {
            if let Some(blog_post) = self.blog_posts.get(blog_index) {
                let mut back_to_list = false;

                ui.add_space(16.0); // Top margin

                let screen_width = ui.available_width();
                let (left_margin, right_margin) = Self::calculate_responsive_margins(screen_width);

                ui.horizontal(|ui| {
                    ui.add_space(left_margin); // Responsive left margin

                    ui.vertical(|ui| {
                        let content_width = ui.available_width() - right_margin;
                        ui.set_max_width(content_width);

                        if ui.button("< Back to Blog List").clicked() {
                            back_to_list = true;
                        }

                        ui.add_space(32.0);

                        ui.style_mut().override_text_style =
                            Some(egui::TextStyle::Name("Heading1".into()));
                        ui.label(&blog_post.title);

                        ui.add_space(12.0);

                        ui.style_mut().override_text_style = Some(egui::TextStyle::Small);
                        ui.colored_label(
                            egui::Color32::from_rgb(120, 120, 120),
                            &blog_post.date_display,
                        );

                        ui.add_space(40.0);

                        ui.spacing_mut().item_spacing.y = 20.0;
                        ui.spacing_mut().indent = 24.0;

                        let viewer_id = format!("blog_{}", blog_post.slug.as_str());
                        CommonMarkViewer::new(viewer_id).show(
                            ui,
                            &mut self.markdown_cache,
                            &blog_post.content,
                        );

                        ui.add_space(60.0);
                    });
                });

                if back_to_list {
                    self.selected_blog = None;
                    self.current_page = Page::Blog;
                    self.push_route_to_browser();
                }
            } else {
                self.selected_blog = None;
            }
        } else {
            ui.add_space(24.0); // Top margin for blog list

            let screen_width = ui.available_width();
            let (left_margin, right_margin) = Self::calculate_responsive_margins(screen_width);

            ui.horizontal(|ui| {
                ui.add_space(left_margin); // Responsive left margin

                ui.vertical(|ui| {
                    let content_width = ui.available_width() - right_margin;
                    ui.set_max_width(content_width);

                    ui.style_mut().override_text_style =
                        Some(egui::TextStyle::Name("Heading1".into()));
                    ui.label("Blog");

                    ui.add_space(40.0);

                    for index in 0..self.blog_posts.len() {
                        let blog_post = &self.blog_posts[index];
                        let title = blog_post.title.clone();
                        let slug = blog_post.slug.clone();
                        let date_display = blog_post.date_display.clone();
                        let mut open_post = false;

                        ui.group(|ui| {
                            ui.set_width(ui.available_width());
                            ui.vertical(|ui| {
                                ui.add_space(16.0);

                                ui.horizontal(|ui| {
                                    ui.add_space(20.0);

                                    ui.vertical(|ui| {
                                        ui.style_mut().override_text_style =
                                            Some(egui::TextStyle::Name("Heading2".into()));
                                        if ui.link(&title).clicked() {
                                            open_post = true;
                                        }
                                    });

                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::TOP),
                                        |ui| {
                                            ui.add_space(20.0);
                                            ui.style_mut().override_text_style =
                                                Some(egui::TextStyle::Small);
                                            ui.colored_label(
                                                egui::Color32::from_rgb(120, 120, 120),
                                                &date_display,
                                            );
                                        },
                                    );
                                });

                                ui.add_space(16.0);
                            });
                        });

                        ui.add_space(32.0);

                        if open_post {
                            self.selected_blog = Some(index);
                            self.current_page = Page::BlogPost(slug);
                            self.push_route_to_browser();
                        }
                    }

                    ui.add_space(40.0);
                });
            });
        }
    }

    fn create_sample_blog_posts() -> Vec<BlogPost> {
        let mut posts = Vec::new();

        // Automatically discover all .md files in the blog_posts directory
        for file in BLOG_POSTS_DIR.files() {
            if let Some(extension) = file.path().extension() {
                if extension == "md" {
                    if let Ok(content) = std::str::from_utf8(file.contents()) {
                        let slug = file
                            .path()
                            .file_stem()
                            .map(|stem| stem.to_string_lossy().to_string())
                            .unwrap_or_default();
                        posts.push(Self::parse_blog_post(content, slug));
                    }
                }
            }
        }

        // Sort posts by published timestamp (newest first)
        posts.sort_by(|a, b| b.published_at.cmp(&a.published_at));

        posts
    }

    fn parse_blog_post(content: &str, mut slug: String) -> BlogPost {
        let (frontmatter, body) = if let Some(stripped) = content.strip_prefix("---\n") {
            // Find the end of frontmatter
            if let Some(end_pos) = stripped.find("\n---\n") {
                let frontmatter = &stripped[..end_pos];
                let body = &stripped[end_pos + 5..]; // Skip "\n---\n"
                (Some(frontmatter), body)
            } else {
                (None, content)
            }
        } else {
            (None, content)
        };

        let mut title = "Untitled".to_string();
        let mut date_display = "Unknown".to_string();
        let mut published_at = Utc.from_utc_datetime(
            &NaiveDate::from_ymd_opt(1970, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );

        if let Some(fm) = frontmatter {
            for line in fm.lines() {
                let line = line.trim();
                if let Some(rest) = line.strip_prefix("title:") {
                    title = rest.trim().trim_matches('"').to_string();
                } else if let Some(rest) = line.strip_prefix("date:") {
                    let value = rest.trim().trim_matches('"');
                    date_display = value.to_string();

                    if let Ok(dt) = DateTime::parse_from_rfc3339(value) {
                        date_display = dt.format("%B %-d, %Y").to_string();
                        published_at = dt.with_timezone(&Utc);
                    } else if let Ok(dt) = NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S")
                    {
                        date_display = dt.format("%B %-d, %Y").to_string();
                        published_at = Utc.from_utc_datetime(&dt);
                    } else if let Ok(dt) = NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S")
                    {
                        date_display = dt.format("%B %-d, %Y").to_string();
                        published_at = Utc.from_utc_datetime(&dt);
                    } else if let Ok(date) = NaiveDate::parse_from_str(value, "%Y-%m-%d") {
                        date_display = date.format("%B %-d, %Y").to_string();
                        published_at = Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0).unwrap());
                    } else if let Ok(date) = NaiveDate::parse_from_str(value, "%B %d, %Y") {
                        date_display = date.format("%B %-d, %Y").to_string();
                        published_at = Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0).unwrap());
                    }
                } else if let Some(rest) = line.strip_prefix("slug:") {
                    let candidate = rest.trim().trim_matches('"').to_string();
                    if !candidate.is_empty() {
                        slug = candidate;
                    }
                }
            }
        }

        BlogPost {
            title,
            date_display,
            content: body.to_string(),
            slug,
            published_at,
        }
    }

    fn sync_blog_selection_from_route(&mut self) {
        match self.current_page.clone() {
            Page::BlogPost(slug) => {
                if let Some(index) = self.blog_posts.iter().position(|post| post.slug == slug) {
                    self.selected_blog = Some(index);
                } else {
                    self.selected_blog = None;
                    self.current_page = Page::Blog;
                    self.push_route_to_browser();
                }
            }
            Page::Blog => {
                self.selected_blog = None;
            }
            _ => {}
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
            let desired_path = self.current_page.to_url();

            if let Some(current_path) = current_pathname() {
                if current_path.as_str() == desired_path.as_str() {
                    return;
                }
            }

            if let Some(window) = web_sys::window() {
                if let Ok(history) = window.history() {
                    let _ = history.push_state_with_url(
                        &wasm_bindgen::JsValue::NULL,
                        "",
                        Some(&desired_path),
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

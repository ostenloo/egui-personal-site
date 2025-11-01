use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use egui_extras::{Size, StripBuilder};
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

struct ThemeToggleButton {
    is_dark: bool,
}

impl ThemeToggleButton {
    fn new(is_dark: bool) -> Self {
        Self { is_dark }
    }
}

impl egui::Widget for ThemeToggleButton {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let size = egui::vec2(36.0, 36.0);
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);
            let painter = ui.painter_at(rect);
            painter.rect(rect, visuals.rounding, visuals.bg_fill, egui::Stroke::NONE);

            let icon_padding = rect.height() * 0.25;
            let icon_rect = rect.shrink(icon_padding);
            let scale = (icon_rect.width().min(icon_rect.height()) / 24.0).max(0.01);
            let offset = icon_rect.min;
            let to_pos = |x: f32, y: f32| egui::pos2(offset.x + x * scale, offset.y + y * scale);

            let fg = visuals.fg_stroke.color;
            let bg = visuals.bg_fill;

            if self.is_dark {
                // Show a sun when currently dark (suggesting a click switches to light)
                painter.circle_filled(to_pos(12.0, 12.0), 6.0 * scale, fg);
                let stroke = egui::Stroke::new(visuals.fg_stroke.width, fg);
                let beams = [
                    (12.0, 1.0, 12.0, 3.0),
                    (12.0, 21.0, 12.0, 23.0),
                    (4.22, 4.22, 5.64, 5.64),
                    (18.36, 18.36, 19.78, 19.78),
                    (1.0, 12.0, 3.0, 12.0),
                    (21.0, 12.0, 23.0, 12.0),
                    (4.22, 19.78, 5.64, 18.36),
                    (18.36, 5.64, 19.78, 4.22),
                ];
                for (x1, y1, x2, y2) in beams {
                    painter.line_segment([to_pos(x1, y1), to_pos(x2, y2)], stroke);
                }
            } else {
                // Show a crescent moon when currently light (suggesting a click switches to dark)
                painter.circle_filled(to_pos(12.0, 12.0), 6.0 * scale, fg);
                painter.circle_filled(to_pos(16.0, 10.0), 6.0 * scale, bg);
                let glow = egui::Stroke::new(visuals.fg_stroke.width, fg.gamma_multiply(0.8));
                painter.circle_stroke(to_pos(12.0, 12.0), 6.0 * scale, glow);
            }
        }

        response
    }
}

struct MenuToggleButton {
    is_open: bool,
}

impl MenuToggleButton {
    fn new(is_open: bool) -> Self {
        Self { is_open }
    }
}

impl egui::Widget for MenuToggleButton {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let size = egui::vec2(36.0, 36.0);
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);
            let painter = ui.painter_at(rect);
            painter.rect(rect, visuals.rounding, visuals.bg_fill, egui::Stroke::NONE);

            let icon_padding = rect.height() * 0.25;
            let icon_rect = rect.shrink(icon_padding);
            let scale = (icon_rect.width().min(icon_rect.height()) / 24.0).max(0.01);
            let offset = icon_rect.min;
            let to_pos = |x: f32, y: f32| egui::pos2(offset.x + x * scale, offset.y + y * scale);

            let stroke = egui::Stroke::new(visuals.fg_stroke.width, visuals.fg_stroke.color);

            if self.is_open {
                painter.line_segment([to_pos(6.0, 6.0), to_pos(18.0, 18.0)], stroke);
                painter.line_segment([to_pos(18.0, 6.0), to_pos(6.0, 18.0)], stroke);
            } else {
                let lines = [6.0, 12.0, 18.0];
                for y in lines {
                    painter.line_segment([to_pos(6.0, y), to_pos(18.0, y)], stroke);
                }
            }
        }

        response
    }
}

struct NavBarButton<'a> {
    label: &'a str,
    selected: bool,
}

impl<'a> NavBarButton<'a> {
    fn new(label: &'a str, selected: bool) -> Self {
        Self { label, selected }
    }
}

impl egui::Widget for NavBarButton<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let padding = ui.spacing().button_padding;
        let text_style = egui::TextStyle::Name("Heading2".into());

        let text = egui::WidgetText::from(
            egui::RichText::new(self.label)
                .text_style(text_style.clone())
                .color(ui.style().visuals.text_color()),
        );
        let galley = text.into_galley(ui, Some(false), f32::INFINITY, text_style.clone());

        let height = ui
            .spacing()
            .interact_size
            .y
            .max(galley.size().y + padding.y * 2.0);
        let width = galley.size().x + padding.x * 2.0;

        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::click());
        let visuals = ui.style().interact_selectable(&response, self.selected);

        if ui.is_rect_visible(rect) {
            let fill = if self.selected || response.hovered() {
                visuals.bg_fill
            } else {
                egui::Color32::TRANSPARENT
            };

            let painter = ui.painter_at(rect);
            painter.rect(rect, egui::Rounding::ZERO, fill, egui::Stroke::NONE);

            let text_pos = egui::pos2(
                rect.left() + padding.x,
                rect.center().y - galley.size().y * 0.5,
            );
            painter.galley(text_pos, galley, visuals.text_color());
        }

        response
    }
}

struct NavMenuButton<'a> {
    label: &'a str,
    selected: bool,
}

impl<'a> NavMenuButton<'a> {
    fn new(label: &'a str, selected: bool) -> Self {
        Self { label, selected }
    }
}

impl egui::Widget for NavMenuButton<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let width = ui.available_width();
        let height = 56.0;
        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::click());
        let visuals = ui.style().interact_selectable(&response, self.selected);

        if ui.is_rect_visible(rect) {
            let painter = ui.painter_at(rect);
            painter.rect(
                rect,
                egui::Rounding::same(0.0),
                visuals.bg_fill,
                egui::Stroke::NONE,
            );

            let padding = ui.spacing().button_padding;
            let text_color = visuals.text_color();
            let text = egui::WidgetText::from(
                egui::RichText::new(self.label)
                    .text_style(egui::TextStyle::Name("Heading1".into()))
                    .color(text_color),
            );
            let galley = text.into_galley(
                ui,
                Some(false),
                width - padding.x * 2.0,
                egui::TextStyle::Name("Heading1".into()),
            );

            let text_pos = egui::pos2(
                rect.left() + padding.x,
                rect.center().y - galley.size().y * 0.5,
            );
            painter.galley(text_pos, galley, text_color);
        }

        response
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
    prefer_dark: bool,

    #[serde(skip)] // Don't serialize blog posts and cache
    blog_posts: Vec<BlogPost>,
    selected_blog: Option<usize>,

    #[serde(skip)] // Don't serialize the cache
    markdown_cache: CommonMarkCache,
    #[serde(skip)] // UI-only overlay state
    show_mobile_menu: bool,
    #[serde(skip)]
    compact_text_styles: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            current_page: Page::Home,
            prefer_dark: true,
            blog_posts: Self::create_sample_blog_posts(),
            selected_blog: None,
            markdown_cache: CommonMarkCache::default(),
            show_mobile_menu: false,
            compact_text_styles: false,
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

    fn configure_text_styles(style: &mut egui::Style, compact: bool) {
        let heading1 = if compact { 26.0 } else { 34.0 };
        let heading2 = if compact { 21.0 } else { 27.0 };
        let heading3 = if compact { 18.0 } else { 23.0 };
        let heading = heading2;
        let body = if compact { 14.0 } else { 16.0 };
        let monospace = if compact { 14.0 } else { 15.0 };
        let small = if compact { 13.0 } else { 14.0 };
        let button = if compact { 14.0 } else { 16.0 };

        style.text_styles = [
            (
                egui::TextStyle::Name("Heading1".into()),
                egui::FontId::new(heading1, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Name("Heading2".into()),
                egui::FontId::new(heading2, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Name("Heading3".into()),
                egui::FontId::new(heading3, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Heading,
                egui::FontId::new(heading, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Body,
                egui::FontId::new(body, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Monospace,
                egui::FontId::new(monospace, egui::FontFamily::Monospace),
            ),
            (
                egui::TextStyle::Small,
                egui::FontId::new(small, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Button,
                egui::FontId::new(button, egui::FontFamily::Proportional),
            ),
        ]
        .into();

        style.spacing.item_spacing = if compact {
            egui::vec2(6.0, 10.0)
        } else {
            egui::vec2(8.0, 12.0)
        };
        style.spacing.button_padding = if compact {
            egui::vec2(10.0, 6.0)
        } else {
            egui::vec2(12.0, 8.0)
        };
    }

    fn update_text_styles_for_screen(&mut self, ctx: &egui::Context, compact: bool) {
        if self.compact_text_styles == compact {
            return;
        }
        let mut style = (*ctx.style()).clone();
        Self::configure_text_styles(&mut style, compact);
        ctx.set_style(style);
        self.compact_text_styles = compact;
    }

    fn ensure_theme(&self, ctx: &egui::Context) {
        let desired_dark = self.prefer_dark;
        if ctx.style().visuals.dark_mode != desired_dark {
            let mut visuals = if desired_dark {
                egui::Visuals::dark()
            } else {
                egui::Visuals::light()
            };
            let accent = egui::Color32::from_rgb(22, 163, 74);
            visuals.hyperlink_color = accent;
            visuals.selection.bg_fill = accent.linear_multiply(0.2);
            visuals.selection.stroke.color = accent;
            ctx.set_visuals(visuals);
        }
    }

    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.

        let mut style = (*cc.egui_ctx.style()).clone();

        Self::configure_text_styles(&mut style, false);

        cc.egui_ctx.set_style(style);

        let mut restored_from_storage = false;
        let mut app: Self = if let Some(storage) = cc.storage {
            restored_from_storage = true;
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        if !restored_from_storage {
            app.prefer_dark = cc.egui_ctx.style().visuals.dark_mode;
        }

        // Rehydrate transient state that we intentionally skip during serialization.
        app.blog_posts = Self::create_sample_blog_posts();
        if let Some(selected) = app.selected_blog {
            if selected >= app.blog_posts.len() {
                app.selected_blog = None;
            }
        }
        app.markdown_cache = CommonMarkCache::default();
        app.show_mobile_menu = false;
        app.compact_text_styles = false;

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
        self.ensure_theme(ctx);

        let screen_width = ctx.input(|input| input.screen_rect.width());
        let is_compact = screen_width < 520.0;
        self.update_text_styles_for_screen(ctx, is_compact);

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

        if !is_compact {
            self.show_mobile_menu = false;
        }

        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.set_height(40.0);

            egui::menu::bar(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(12.0);
                }

                StripBuilder::new(ui)
                    .sizes(Size::remainder(), 1)
                    .sizes(Size::exact(44.0), 1)
                    .horizontal(|mut strip| {
                        strip.cell(|ui| {
                            ui.spacing_mut().item_spacing.x = 8.0;
                            if is_compact {
                                let response = ui.add_sized(
                                    [36.0, 36.0],
                                    MenuToggleButton::new(self.show_mobile_menu),
                                );
                                if response.clicked() {
                                    self.show_mobile_menu = !self.show_mobile_menu;
                                }
                            } else {
                                self.render_nav_links(ui);
                            }
                        });

                        strip.cell(|ui| {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    let response = ui.add_sized(
                                        [36.0, 36.0],
                                        ThemeToggleButton::new(self.prefer_dark),
                                    );
                                    if response.clicked() {
                                        self.prefer_dark = !self.prefer_dark;
                                        self.ensure_theme(ctx);
                                    }
                                    ui.add_space(4.0);
                                },
                            );
                        });
                    });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    match self.current_page.clone() {
                        Page::Home => self.show_home(ui),
                        Page::Projects => self.show_projects(ui),
                        Page::Blog | Page::BlogPost(_) => self.show_blog(ui),
                    }
                });
        });

        if is_compact && self.show_mobile_menu {
            self.show_mobile_menu_overlay(ctx);
        }
    }
}

impl MyApp {
    fn navigate_to(&mut self, page: Page) -> bool {
        if self.current_page != page {
            self.current_page = page;
            self.selected_blog = None;
            true
        } else {
            false
        }
    }

    fn theme_toggle_control(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let response = ui.add_sized([36.0, 36.0], ThemeToggleButton::new(self.prefer_dark));
        if response.clicked() {
            self.prefer_dark = !self.prefer_dark;
            self.ensure_theme(ctx);
        }
    }

    fn render_nav_links(&mut self, ui: &mut egui::Ui) {
        ui.style_mut().override_text_style = Some(egui::TextStyle::Name("Heading2".into()));

        let _ = self.nav_item(
            ui,
            "Home",
            matches!(self.current_page, Page::Home),
            Page::Home,
        );
        let _ = self.nav_item(
            ui,
            "Projects",
            matches!(self.current_page, Page::Projects),
            Page::Projects,
        );
        let _ = self.nav_item(
            ui,
            "Blog",
            matches!(self.current_page, Page::Blog | Page::BlogPost(_)),
            Page::Blog,
        );
    }

    fn show_mobile_menu_overlay(&mut self, ctx: &egui::Context) {
        if ctx.input(|input| input.key_pressed(egui::Key::Escape)) {
            self.show_mobile_menu = false;
            return;
        }

        let screen_rect = ctx.screen_rect();
        let visuals = ctx.style().visuals.clone();
        let fill = visuals.panel_fill;

        egui::Area::new(egui::Id::new("mobile_menu_overlay"))
            .order(egui::Order::Foreground)
            .movable(false)
            .interactable(true)
            .fixed_pos(screen_rect.min)
            .show(ctx, |ui| {
                ui.set_min_size(screen_rect.size());
                egui::Frame::none().fill(fill).show(ui, |ui| {
                    ui.set_min_size(screen_rect.size());

                    ui.horizontal(|ui| {
                        ui.set_width(ui.available_width());
                        ui.spacing_mut().item_spacing.x = 8.0;

                        let response = ui.add_sized([36.0, 36.0], MenuToggleButton::new(true));
                        if response.clicked() {
                            self.show_mobile_menu = false;
                        }

                        ui.allocate_space(egui::vec2(ui.available_width(), 0.0));

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            self.theme_toggle_control(ui, ctx);
                            ui.add_space(4.0);
                        });
                    });

                    ui.style_mut().override_text_style =
                        Some(egui::TextStyle::Name("Heading1".into()));
                    ui.spacing_mut().item_spacing.y = 0.0;
                    ui.spacing_mut().button_padding = egui::vec2(16.0, 16.0);

                    ui.vertical(|ui| {
                        ui.set_width(ui.available_width());

                        let home = self.nav_item_full_width(
                            ui,
                            "Home",
                            matches!(self.current_page, Page::Home),
                            Page::Home,
                        );
                        if home.clicked() {
                            self.show_mobile_menu = false;
                        }

                        let projects = self.nav_item_full_width(
                            ui,
                            "Projects",
                            matches!(self.current_page, Page::Projects),
                            Page::Projects,
                        );
                        if projects.clicked() {
                            self.show_mobile_menu = false;
                        }

                        let blog = self.nav_item_full_width(
                            ui,
                            "Blog",
                            matches!(self.current_page, Page::Blog | Page::BlogPost(_)),
                            Page::Blog,
                        );
                        if blog.clicked() {
                            self.show_mobile_menu = false;
                        }
                    });
                });
            });
    }

    fn nav_item(
        &mut self,
        ui: &mut egui::Ui,
        label: &str,
        is_selected: bool,
        target: Page,
    ) -> egui::Response {
        let response = ui.add(NavBarButton::new(label, is_selected));
        if response.clicked() && self.navigate_to(target) {
            self.push_route_to_browser();
        }
        response
    }

    fn nav_item_full_width(
        &mut self,
        ui: &mut egui::Ui,
        label: &str,
        is_selected: bool,
        target: Page,
    ) -> egui::Response {
        let response = ui.add(NavMenuButton::new(label, is_selected));
        if response.clicked() && self.navigate_to(target) {
            self.push_route_to_browser();
        }
        response
    }

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
                let show_dates = screen_width >= 520.0;

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

                        if show_dates {
                            ui.style_mut().override_text_style = Some(egui::TextStyle::Small);
                            ui.colored_label(
                                egui::Color32::from_rgb(120, 120, 120),
                                &blog_post.date_display,
                            );
                        }

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
            let show_dates = screen_width >= 520.0;

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

                        egui::Frame::group(&ui.style())
                            .stroke(egui::Stroke::NONE)
                            .rounding(egui::Rounding::ZERO)
                            .show(ui, |ui| {
                                ui.set_width(ui.available_width());
                                ui.vertical(|ui| {
                                    ui.add_space(12.0);

                                    ui.horizontal(|ui| {

                                        ui.vertical(|ui| {
                                            ui.style_mut().override_text_style =
                                                Some(egui::TextStyle::Name("Heading2".into()));
                                            if ui.link(&title).clicked() {
                                                open_post = true;
                                            }
                                        });

                                        if show_dates {
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
                                        }
                                    });

                                    ui.add_space(16.0);
                                });
                            });

                        ui.add_space(16.0);

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

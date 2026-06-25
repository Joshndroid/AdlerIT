use eframe::egui::{self, Color32, Stroke};

const ACCENT: Color32 = Color32::from_rgb(25, 118, 210);
const ACCENT_DARK: Color32 = Color32::from_rgb(100, 181, 246);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeMode {
    System,
    Light,
    Dark,
}

impl ThemeMode {
    pub const ALL: [Self; 3] = [Self::System, Self::Light, Self::Dark];

    pub fn label(self) -> &'static str {
        match self {
            Self::System => "System",
            Self::Light => "Light",
            Self::Dark => "Dark",
        }
    }
}

pub fn apply(ctx: &egui::Context, mode: ThemeMode) {
    let dark = match mode {
        ThemeMode::System => ctx.system_theme().unwrap_or(egui::Theme::Dark) == egui::Theme::Dark,
        ThemeMode::Light => false,
        ThemeMode::Dark => true,
    };

    let mut visuals = if dark {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };
    let accent = if dark { ACCENT_DARK } else { ACCENT };
    visuals.selection.bg_fill = accent;
    visuals.hyperlink_color = accent;
    visuals.widgets.active.bg_fill = accent;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, accent);
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, accent);

    if dark {
        visuals.panel_fill = Color32::from_rgb(24, 26, 29);
        visuals.window_fill = Color32::from_rgb(30, 32, 36);
        visuals.extreme_bg_color = Color32::from_rgb(18, 20, 23);
        visuals.faint_bg_color = Color32::from_rgb(36, 39, 43);
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(38, 41, 46);
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(48, 52, 58);
    } else {
        visuals.panel_fill = Color32::from_rgb(248, 249, 251);
        visuals.window_fill = Color32::WHITE;
        visuals.extreme_bg_color = Color32::from_rgb(241, 244, 248);
        visuals.faint_bg_color = Color32::from_rgb(236, 241, 247);
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(245, 247, 250);
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(232, 240, 254);
    }

    visuals.window_stroke = Stroke::new(1.0, visuals.widgets.noninteractive.bg_stroke.color);
    ctx.set_visuals(visuals);
}

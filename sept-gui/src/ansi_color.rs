pub struct ANSIColor;

// Reference: https://upload.wikimedia.org/wikipedia/commons/0/02/EGA_Table.svg
impl ANSIColor {
    pub const DARK_BLACK: egui::Color32 = egui::Color32::from_rgb(0, 0, 0);
    pub const DARK_BLUE: egui::Color32 = egui::Color32::from_rgb(0, 0, 0xAA);
    pub const DARK_GREEN: egui::Color32 = egui::Color32::from_rgb(0, 0xAA, 0);
    pub const DARK_CYAN: egui::Color32 = egui::Color32::from_rgb(0, 0xAA, 0xAA);
    pub const DARK_RED: egui::Color32 = egui::Color32::from_rgb(0xAA, 0, 0);
    pub const DARK_MAGENTA: egui::Color32 = egui::Color32::from_rgb(0xAA, 0, 0xAA);
    pub const DARK_YELLOW: egui::Color32 = egui::Color32::from_rgb(0xAA, 0x55, 0);
    pub const DARK_WHITE: egui::Color32 = egui::Color32::from_rgb(0xAA, 0xAA, 0xAA);
    pub const BRIGHT_BLACK: egui::Color32 = egui::Color32::from_rgb(0x55, 0x55, 0x55);
    pub const BRIGHT_BLUE: egui::Color32 = egui::Color32::from_rgb(0x55, 0x55, 0xFF);
    pub const BRIGHT_GREEN: egui::Color32 = egui::Color32::from_rgb(0x55, 0xFF, 0x55);
    pub const BRIGHT_CYAN: egui::Color32 = egui::Color32::from_rgb(0x55, 0xFF, 0xFF);
    pub const BRIGHT_RED: egui::Color32 = egui::Color32::from_rgb(0xFF, 0x55, 0x55);
    pub const BRIGHT_MAGENTA: egui::Color32 = egui::Color32::from_rgb(0xFF, 0x55, 0xFF);
    pub const BRIGHT_YELLOW: egui::Color32 = egui::Color32::from_rgb(0xFF, 0xFF, 0x55);
    pub const BRIGHT_WHITE: egui::Color32 = egui::Color32::from_rgb(0xFF, 0xFF, 0xFF);
}

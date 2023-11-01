use bevy::prelude::Color;
use bevy::utils::HashMap;
use lazy_static::lazy_static;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Palette {
    Background,
}

impl Into<Color> for Palette {
    fn into(self) -> Color {
        COLORS[&self]
    }
}

lazy_static! {
    static ref COLORS: HashMap<Palette, Color> = HashMap::from([
        (Palette::Background, Color::BLACK),
    ]);
}
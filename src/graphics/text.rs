use bevy::ecs::system::EntityCommands;
use bevy::prelude::{Color, Commands, Text, Text2dBundle, Transform};
use bevy::sprite::Anchor;
use bevy::text::TextStyle;

use crate::screens::Fonts;

pub enum TextStyles {
    Basic,
}

impl TextStyles {
    pub fn style(&self, fonts: &Fonts) -> TextStyle {
        match self {
            TextStyles::Basic => TextStyle {
                font: fonts.absolute.clone(),
                font_size: 18.0,
                color: Color::WHITE,
            },
        }
    }
}

pub fn text<'w, 's, 'l>(commands: &'l mut Commands<'w, 's>, fonts: &Fonts, text: &str, style: TextStyles, anchor: Anchor, pos: (f32, f32, f32)) -> EntityCommands<'w, 's, 'l> {
    commands
        .spawn(Text2dBundle {
            text: Text::from_section(text, style.style(fonts)),
            text_anchor: anchor,
            transform: Transform::from_xyz(pos.0, pos.1, pos.2),
            ..Default::default()
        })
}
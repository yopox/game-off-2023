use bevy::ecs::system::EntityCommands;
use bevy::prelude::{Color, Commands, Text, Text2dBundle, Transform};
use bevy::sprite::Anchor;
use bevy::text::TextStyle;

use crate::screens::Fonts;

pub enum TextStyles {
    Basic,
    Black,
}

impl TextStyles {
    pub fn style(&self, fonts: &Fonts) -> TextStyle {
        match self {
            TextStyles::Basic => TextStyle {
                font: fonts.chunky.clone(),
                font_size: 8.0 * 4.0,
                color: Color::WHITE,
            },
            TextStyles::Black => TextStyle {
                font: fonts.chunky.clone(),
                font_size: 8.0 * 4.0,
                color: Color::BLACK,
            },
        }
    }

    pub fn style_with_alpha(&self, fonts: &Fonts, alpha: f32) -> TextStyle {
        let mut style = self.style(fonts);
        style.color.set_a(alpha);
        style
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
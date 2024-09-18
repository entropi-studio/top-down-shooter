use bevy::asset::Handle;
use bevy::color::Color;
use bevy::prelude::{default, Entity, Font, JustifyText, Text, TextBundle, TextSection, TextStyle};
use bevy::text::BreakLineOn;
use sickle_ui::prelude::UiBuilder;

#[derive(Clone)]
pub struct TextBuilder {
    sections: Vec<TextSection>,
    font: Handle<Font>,
    font_size: f32,
    color: Color,
    justify_text: JustifyText,
    break_line_on: BreakLineOn,
}

impl TextBuilder {
    pub fn new() -> Self {
        Self {
            sections: vec![],
            font: Handle::default(),
            font_size: 24.0,
            color: Color::WHITE,
            justify_text: JustifyText::default(),
            break_line_on: BreakLineOn::default(),
        }
    }

    pub fn to_text(&self) -> Text {
        Text {
            sections: self.sections.clone(),
            justify: self.justify_text,
            linebreak_behavior: self.break_line_on,
        }
    }

    pub fn to_text_bundle(&self) -> TextBundle {
        TextBundle {
            text: self.to_text(),
            ..default()
        }
    }

    pub fn append(&mut self, text: impl Into<String>) -> &Self {
        self.sections.push(TextSection {
            value: text.into(),
            style: TextStyle {
                font: self.font.clone(),
                font_size: self.font_size,
                color: self.color,
            },
        });
        self
    }

    pub fn font(&mut self, font: Handle<Font>) -> &Self {
        self.font = font;
        self
    }

    pub fn font_size(&mut self, size: f32) -> &Self {
        self.font_size = size;
        self
    }

    pub fn color(&mut self, color: Color) -> &Self {
        self.color = color;
        self
    }

    pub fn justify(&mut self, justify: JustifyText) -> &Self {
        self.justify_text = justify;
        self
    }

    pub fn break_line_on(&mut self, break_line_on: BreakLineOn) -> &Self {
        self.break_line_on = break_line_on;
        self
    }
}

pub trait TextBuilderExt {
    fn text(&mut self, base: impl Into<String>, build: impl FnOnce(&mut TextBuilder));
    fn text_empty(&mut self, build: impl FnOnce(&mut TextBuilder));
}

impl TextBuilderExt for UiBuilder<'_, Entity> {
    fn text(&mut self, base: impl Into<String>, build: impl FnOnce(&mut TextBuilder)) {
        let mut builder = TextBuilder::new();
        builder.append(base);
        build(&mut builder);
        self.spawn(builder.to_text_bundle());
    }

    fn text_empty(&mut self, build: impl FnOnce(&mut TextBuilder)) {
        let mut builder = TextBuilder::new();
        build(&mut builder);
        self.spawn(builder.to_text_bundle());
    }
}

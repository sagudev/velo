use bevy::{prelude::*, window::PrimaryWindow};

use uuid::Uuid;

use crate::{AddRect, AppState, JsonNode, JsonNodeText, NodeType};

use super::ui_helpers::{
    get_sections, pos_to_style, ArrowMeta, ArrowMode, ButtonAction, ChangeColor, EditableText,
    Rectangle, TextManipulation, TextManipulationAction, TextPosMode,
};

pub fn button_handler(
    mut commands: Commands,
    mut events: EventWriter<AddRect>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonAction),
        (Changed<Interaction>, With<ButtonAction>),
    >,
    mut nodes: Query<(Entity, &Rectangle, &mut ZIndex), With<Rectangle>>,
    arrows: Query<(Entity, &ArrowMeta), With<ArrowMeta>>,
    mut state: ResMut<AppState>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single();
    for (interaction, mut color, button_action) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => match button_action.button_type {
                super::ui_helpers::ButtonTypes::Add => {
                    events.send(AddRect {
                        node: JsonNode {
                            id: Uuid::new_v4(),
                            node_type: NodeType::Rect,
                            left: Val::Px(window.width() / 2. - 200.),
                            bottom: Val::Px(window.height() / 2.),
                            width: Val::Px(100.0),
                            height: Val::Px(100.0),
                            text: JsonNodeText {
                                text: "".to_string(),
                                pos: crate::TextPos::Center,
                            },
                            bg_color: Color::WHITE,
                            tags: vec![],
                            z_index: 0,
                        },
                        image: None,
                    });
                }
                super::ui_helpers::ButtonTypes::Del => {
                    if let Some(id) = state.entity_to_edit {
                        state.entity_to_edit = None;
                        state.entity_to_resize = None;
                        state.hold_entity = None;
                        state.arrow_to_draw_start = None;
                        for (entity, node, _) in nodes.iter() {
                            if node.id == id {
                                commands.entity(entity).despawn_recursive();
                            }
                        }
                        for (entity, arrow) in arrows.iter() {
                            if arrow.start.id == id || arrow.end.id == id {
                                commands.entity(entity).despawn_recursive();
                            }
                        }
                    }
                }
                super::ui_helpers::ButtonTypes::Front => {
                    if let Some(id) = state.entity_to_edit {
                        for (_, node, mut z_index) in nodes.iter_mut() {
                            if node.id == id {
                                if let ZIndex::Local(i) = *z_index {
                                    *z_index = ZIndex::Local(i + 1);
                                }
                            }
                        }
                    }
                }
                super::ui_helpers::ButtonTypes::Back => {
                    if let Some(id) = state.entity_to_edit {
                        for (_, node, mut z_index) in nodes.iter_mut() {
                            if node.id == id {
                                if let ZIndex::Local(i) = *z_index {
                                    *z_index = ZIndex::Local(i - 1);
                                }
                            }
                        }
                    }
                }
            },
            Interaction::Hovered => {
                color.0 = Color::GRAY;
            }
            Interaction::None => {
                color.0 = Color::rgb(0.8, 0.8, 0.8);
            }
        }
    }
}

pub fn change_color_pallete(
    mut interaction_query: Query<
        (&Interaction, &ChangeColor, &mut BackgroundColor),
        (Changed<Interaction>, With<ChangeColor>, Without<Rectangle>),
    >,
    mut nodes: Query<(&mut BackgroundColor, &Rectangle), With<Rectangle>>,
    state: Res<AppState>,
) {
    for (interaction, change_color, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                let color = change_color.color;
                if state.entity_to_edit.is_some() {
                    for (mut bg_color, node) in nodes.iter_mut() {
                        if node.id == state.entity_to_edit.unwrap() {
                            bg_color.0 = color;
                        }
                    }
                }
            }
            Interaction::Hovered => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.5);
            }
            Interaction::None => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 1.);
            }
        }
    }
}

pub fn change_text_pos(
    mut interaction_query: Query<
        (&Interaction, &TextPosMode, &mut BackgroundColor),
        (Changed<Interaction>, With<TextPosMode>),
    >,
    mut nodes: Query<(&mut Style, &Rectangle), With<Rectangle>>,
    state: Res<AppState>,
) {
    for (interaction, text_pos_mode, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if state.entity_to_edit.is_some() {
                    for (mut style, node) in nodes.iter_mut() {
                        if node.id == state.entity_to_edit.unwrap() {
                            let (justify_content, align_items) =
                                pos_to_style(text_pos_mode.text_pos.clone());
                            style.justify_content = justify_content;
                            style.align_items = align_items;
                        }
                    }
                }
            }
            Interaction::Hovered => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.8);
            }
            Interaction::None => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.5);
            }
        }
    }
}

pub fn change_arrow_type(
    mut interaction_query: Query<
        (&Interaction, &ArrowMode, &mut BackgroundColor),
        (Changed<Interaction>, With<ArrowMode>),
    >,
    mut state: ResMut<AppState>,
) {
    for (interaction, arrow_mode, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                state.arrow_type = arrow_mode.arrow_type;
            }
            Interaction::Hovered => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.8);
            }
            Interaction::None => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.5);
            }
        }
    }
}

pub fn text_manipulation(
    mut interaction_query: Query<
        (&Interaction, &TextManipulationAction, &mut BackgroundColor),
        (Changed<Interaction>, With<TextManipulationAction>),
    >,
    mut editable_text: Query<(&mut Text, &EditableText), With<EditableText>>,
    state: Res<AppState>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/iosevka-regular.ttf");
    for (interaction, text_manipulation, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                #[cfg(not(target_arch = "wasm32"))]
                let mut clipboard = arboard::Clipboard::new().unwrap();

                match text_manipulation.action_type {
                    TextManipulation::Cut => {
                        if let Some(id) = state.entity_to_edit {
                            for (mut text, node) in editable_text.iter_mut() {
                                if node.id == id {
                                    let mut str = "".to_string();
                                    for section in text.sections.iter_mut() {
                                        str = format!("{}{}", str, section.value.clone());
                                    }
                                    text.sections = vec![TextSection {
                                        value: "".to_string(),
                                        style: TextStyle {
                                            font: font.clone(),
                                            font_size: 20.0,
                                            color: Color::BLACK,
                                        },
                                    }];
                                    #[cfg(not(target_arch = "wasm32"))]
                                    clipboard.set_text(str).unwrap()
                                }
                            }
                        }
                    }
                    TextManipulation::Paste =>
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        if let Ok(clipboard_text) = clipboard.get_text() {
                            for (mut text, editable_text) in editable_text.iter_mut() {
                                if Some(editable_text.id) == state.entity_to_edit {
                                    let mut str = "".to_string();
                                    for section in text.sections.iter_mut() {
                                        str = format!("{}{}", str, section.value.clone());
                                    }
                                    str = format!("{}{}", str, clipboard_text);
                                    text.sections = get_sections(str, font.clone()).0;
                                }
                            }
                        }
                    }
                    TextManipulation::Copy => {
                        if let Some(id) = state.entity_to_edit {
                            for (mut text, node) in editable_text.iter_mut() {
                                if node.id == id {
                                    let mut str = "".to_string();
                                    for section in text.sections.iter_mut() {
                                        str = format!("{}{}", str, section.value.clone());
                                    }
                                    #[cfg(not(target_arch = "wasm32"))]
                                    clipboard.set_text(str).unwrap()
                                }
                            }
                        }
                    }
                    TextManipulation::OpenAllLinks => {
                        if let Some(id) = state.entity_to_edit {
                            for (mut text, node) in editable_text.iter_mut() {
                                if node.id == id {
                                    let mut str = "".to_string();
                                    for section in text.sections.iter_mut() {
                                        str = format!("{}{}", str, section.value.clone());
                                    }
                                    let (sections, is_link) = get_sections(str, font.clone());
                                    for (i, section) in sections.iter().enumerate() {
                                        if is_link[i] {
                                            #[cfg(not(target_arch = "wasm32"))]
                                            open::that(section.value.clone()).unwrap();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Interaction::Hovered => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.8);
            }
            Interaction::None => {
                bg_color.0 = Color::rgba(bg_color.0.r(), bg_color.0.g(), bg_color.0.b(), 0.5);
            }
        }
    }
}

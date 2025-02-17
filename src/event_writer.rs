use crate::widgets::{
    button::*,
    list_view::*,
    selection::*,
    text::*,
    text_input::*,
    image::*,
    circular::*,
    *,
};

use bevy::ecs::event::EventWriter;
use bevy::prelude::*;

/// Interaction Event: Hover, Press, Leave (None)
#[derive(Event, Debug)]
pub struct FaInteractionEvent {
    pub entity: Entity,
    pub widget_id: Option<String>,
    pub interaction: Interaction,
    pub widget: WidgetType,
}

impl FaInteractionEvent {
    fn new(
        entity: Entity,
        widget_id: Option<String>,
        interaction: Interaction,
        widget: WidgetType,
    ) -> Self {
        Self {
            entity,
            widget_id,
            interaction,
            widget,
        }
    }

    pub fn send_event<T>(
        interaction_q: &mut Query<(Entity, &T, Option<&FamiqWidgetId>, &Interaction), Changed<Interaction>>,
        writer: &mut EventWriter<FaInteractionEvent>,
        widget: WidgetType,
    ) where
        T: Component,
    {
        for (entity, _, widget_id, interaction) in interaction_q {
            writer.send(FaInteractionEvent::new(
                entity,
                widget_id.map(|id| id.0.clone()), // return Option<String> or None
                *interaction,
                widget,
            ));
        }
    }

    /// Check if emitted event is a button pressed event.
    pub fn is_button_pressed(&self) -> bool {
        self.widget == WidgetType::Button && self.interaction == Interaction::Pressed
    }

    /// Check if emitted event is a text_input's toggle-icon password pressed.
    pub fn is_password_toggle_icon_pressed(&self) -> bool {
        self.widget == WidgetType::TextInputTogglePasswordIcon && self.interaction == Interaction::Pressed
    }
}

pub fn btn_interaction_system(
    mut interaction_q: Query<
        (Entity, &IsFamiqButton, Option<&FamiqWidgetId>, &Interaction),
        Changed<Interaction>,
    >,
    mut writer: EventWriter<FaInteractionEvent>,
) {
    FaInteractionEvent::send_event(&mut interaction_q, &mut writer, WidgetType::Button);
}

pub fn image_interaction_system(
    mut interaction_q: Query<
        (Entity, &IsFamiqImage, Option<&FamiqWidgetId>, &Interaction),
        Changed<Interaction>,
    >,
    mut writer: EventWriter<FaInteractionEvent>,
) {
    FaInteractionEvent::send_event(&mut interaction_q, &mut writer, WidgetType::Image);
}

pub fn text_input_interaction_system(
    mut interaction_q: Query<
        (Entity, &IsFamiqTextInput, Option<&FamiqWidgetId>, &Interaction),
        Changed<Interaction>,
    >,
    mut writer: EventWriter<FaInteractionEvent>,
) {
    FaInteractionEvent::send_event(&mut interaction_q, &mut writer, WidgetType::TextInput);
}

pub fn text_input_toggle_password_icon_interaction_system(
    mut interaction_q: Query<
        (Entity, &TogglePasswordIcon, Option<&FamiqWidgetId>, &Interaction),
        Changed<Interaction>,
    >,
    mut writer: EventWriter<FaInteractionEvent>,
) {
    FaInteractionEvent::send_event(&mut interaction_q, &mut writer, WidgetType::TextInputTogglePasswordIcon);
}

pub fn selection_interaction_system(
    mut interaction_q: Query<
        (Entity, &IsFamiqSelectionSelector, Option<&FamiqWidgetId>, &Interaction),
        Changed<Interaction>,
    >,
    mut writer: EventWriter<FaInteractionEvent>,
) {
    FaInteractionEvent::send_event(&mut interaction_q, &mut writer, WidgetType::Selection);
}

pub fn selection_choice_interaction_system(
    mut interaction_q: Query<
        (Entity, &IsFamiqSelectionChoice, Option<&FamiqWidgetId>, &Interaction),
        Changed<Interaction>,
    >,
    mut writer: EventWriter<FaInteractionEvent>,
) {
    FaInteractionEvent::send_event(&mut interaction_q, &mut writer, WidgetType::SelectionChoice);
}

pub fn text_interaction_system(
    mut interaction_q: Query<
        (Entity, &IsFamiqText, Option<&FamiqWidgetId>, &Interaction),
        Changed<Interaction>,
    >,
    mut writer: EventWriter<FaInteractionEvent>,
) {
    FaInteractionEvent::send_event(&mut interaction_q, &mut writer, WidgetType::Text);
}

pub fn circular_interaction_system(
    mut interaction_q: Query<
        (Entity, &IsFamiqCircular, Option<&FamiqWidgetId>, &Interaction),
        Changed<Interaction>,
    >,
    mut writer: EventWriter<FaInteractionEvent>,
) {
    FaInteractionEvent::send_event(&mut interaction_q, &mut writer, WidgetType::Circular);
}

pub fn listview_interaction_system(
    mut interaction_q: Query<
        (Entity, &IsFamiqListView, Option<&FamiqWidgetId>, &Interaction),
        Changed<Interaction>,
    >,
    mut writer: EventWriter<FaInteractionEvent>,
) {
    FaInteractionEvent::send_event(&mut interaction_q, &mut writer, WidgetType::ListView);
}

pub fn listview_item_interaction_system(
    mut interaction_q: Query<
        (Entity, &IsFamiqListViewItem, Option<&FamiqWidgetId>, &Interaction),
        Changed<Interaction>,
    >,
    mut writer: EventWriter<FaInteractionEvent>,
) {
    FaInteractionEvent::send_event(&mut interaction_q, &mut writer, WidgetType::ListViewItem);
}

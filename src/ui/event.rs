use sfml::{cpp::FBox, graphics::Texture};

use crate::ui::ui_id::UiId;

pub enum EventFromUi {
    ButtonClicked(UiId),
}

pub enum EventToUi {
    SetTexture(UiId, FBox<Texture>),
}

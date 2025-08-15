use bevy::prelude::*;

#[derive(Component)]
pub struct EntityStates {
    pub hp: EntitiyStatesElement,
    pub mp: EntitiyStatesElement,
    pub strength: EntitiyStatesElement,
    pub dexterity: EntitiyStatesElement,
    pub vitality: EntitiyStatesElement,
    pub intelligence: EntitiyStatesElement,
    pub agility: EntitiyStatesElement,
    pub luck: EntitiyStatesElement,
}
impl EntityStates {
    pub fn default() -> Self {
        EntityStates {
            hp: EntitiyStatesElement::default(),
            mp: EntitiyStatesElement::default(),
            strength: EntitiyStatesElement::default(),
            dexterity: EntitiyStatesElement::default(),
            vitality: EntitiyStatesElement::default(),
            intelligence: EntitiyStatesElement::default(),
            agility: EntitiyStatesElement::default(),
            luck: EntitiyStatesElement::default(),
        }
    }
}

pub struct EntitiyStatesElement {
    default_value: f32,
    pub value: f32,
}
impl EntitiyStatesElement {
    pub fn default() -> Self {
        EntitiyStatesElement {
            default_value: 100.0,
            value: 100.0,
        }
    }

    pub fn new(value: f32) -> Self {
        EntitiyStatesElement {
            default_value: value,
            value: value,
        }
    }

    pub fn reset(&self) -> Self {
        EntitiyStatesElement {
            default_value: self.default_value,
            value: self.default_value,
        }
    }
}

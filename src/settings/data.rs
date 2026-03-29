use macroquad::prelude::KeyCode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ControlProfile {
    Solo,
    OneVsOneP1,
    OneVsOneP2,
}

impl ControlProfile {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Solo => "MODE SOLO",
            Self::OneVsOneP1 => "JOUEUR 1",
            Self::OneVsOneP2 => "JOUEUR 2",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BindingAction {
    MoveLeft,
    MoveRight,
    Jump,
    Shoot,
}

impl BindingAction {
    pub(crate) const ALL: [Self; 4] = [Self::MoveLeft, Self::MoveRight, Self::Jump, Self::Shoot];

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::MoveLeft => "Reculer",
            Self::MoveRight => "Avancer",
            Self::Jump => "Sauter",
            Self::Shoot => "Tirer",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct BindingTarget {
    pub(crate) profile: ControlProfile,
    pub(crate) action: BindingAction,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ConflictInfo {
    pub(crate) conflicting_profile: ControlProfile,
    pub(crate) conflicting_action: BindingAction,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SettingsValidationError {
    ForbiddenKey(KeyCode),
    OneVsOneConflict {
        first: BindingTarget,
        second: BindingTarget,
        key: KeyCode,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SettingsData {
    pub(crate) controls: ControlsConfig,
}

impl Default for SettingsData {
    fn default() -> Self {
        Self {
            controls: ControlsConfig::default(),
        }
    }
}

impl SettingsData {
    pub(crate) fn validate(&self) -> Result<(), SettingsValidationError> {
        self.controls.validate()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ControlsConfig {
    pub(crate) solo: PlayerBindings,
    pub(crate) one_vs_one_p1: PlayerBindings,
    pub(crate) one_vs_one_p2: PlayerBindings,
}

impl Default for ControlsConfig {
    fn default() -> Self {
        Self {
            solo: PlayerBindings::solo_default(),
            one_vs_one_p1: PlayerBindings::one_vs_one_p1_default(),
            one_vs_one_p2: PlayerBindings::one_vs_one_p2_default(),
        }
    }
}

impl ControlsConfig {
    pub(crate) fn bindings(self, profile: ControlProfile) -> PlayerBindings {
        match profile {
            ControlProfile::Solo => self.solo,
            ControlProfile::OneVsOneP1 => self.one_vs_one_p1,
            ControlProfile::OneVsOneP2 => self.one_vs_one_p2,
        }
    }

    pub(crate) fn binding(self, target: BindingTarget) -> KeyCode {
        self.bindings(target.profile).binding(target.action)
    }

    pub(crate) fn set_binding(&mut self, target: BindingTarget, key: KeyCode) {
        self.bindings_mut(target.profile)
            .set_binding(target.action, key);
    }

    pub(crate) fn one_vs_one_conflict_for(
        self,
        candidate_key: KeyCode,
        edited_profile: ControlProfile,
    ) -> Option<ConflictInfo> {
        let (other_profile, other_bindings) = match edited_profile {
            ControlProfile::OneVsOneP1 => (ControlProfile::OneVsOneP2, self.one_vs_one_p2),
            ControlProfile::OneVsOneP2 => (ControlProfile::OneVsOneP1, self.one_vs_one_p1),
            ControlProfile::Solo => return None,
        };

        BindingAction::ALL.into_iter().find_map(|action| {
            (other_bindings.binding(action) == candidate_key).then_some(ConflictInfo {
                conflicting_profile: other_profile,
                conflicting_action: action,
            })
        })
    }

    pub(crate) fn validate(&self) -> Result<(), SettingsValidationError> {
        for profile in [
            ControlProfile::Solo,
            ControlProfile::OneVsOneP1,
            ControlProfile::OneVsOneP2,
        ] {
            let bindings = self.bindings(profile);
            for action in BindingAction::ALL {
                let key = bindings.binding(action);
                if is_forbidden_key(key) {
                    return Err(SettingsValidationError::ForbiddenKey(key));
                }
            }
        }

        for first_action in BindingAction::ALL {
            let first_key = self.one_vs_one_p1.binding(first_action);
            for second_action in BindingAction::ALL {
                if first_key == self.one_vs_one_p2.binding(second_action) {
                    return Err(SettingsValidationError::OneVsOneConflict {
                        first: BindingTarget {
                            profile: ControlProfile::OneVsOneP1,
                            action: first_action,
                        },
                        second: BindingTarget {
                            profile: ControlProfile::OneVsOneP2,
                            action: second_action,
                        },
                        key: first_key,
                    });
                }
            }
        }

        Ok(())
    }

    fn bindings_mut(&mut self, profile: ControlProfile) -> &mut PlayerBindings {
        match profile {
            ControlProfile::Solo => &mut self.solo,
            ControlProfile::OneVsOneP1 => &mut self.one_vs_one_p1,
            ControlProfile::OneVsOneP2 => &mut self.one_vs_one_p2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct PlayerBindings {
    #[serde(with = "keycode_serde")]
    pub(crate) move_left: KeyCode,
    #[serde(with = "keycode_serde")]
    pub(crate) move_right: KeyCode,
    #[serde(with = "keycode_serde")]
    pub(crate) jump: KeyCode,
    #[serde(with = "keycode_serde")]
    pub(crate) shoot: KeyCode,
}

impl PlayerBindings {
    pub(crate) const fn solo_default() -> Self {
        Self {
            move_left: KeyCode::Q,
            move_right: KeyCode::D,
            jump: KeyCode::Z,
            shoot: KeyCode::S,
        }
    }

    pub(crate) const fn one_vs_one_p1_default() -> Self {
        Self::solo_default()
    }

    pub(crate) const fn one_vs_one_p2_default() -> Self {
        Self {
            move_left: KeyCode::Left,
            move_right: KeyCode::Right,
            jump: KeyCode::Up,
            shoot: KeyCode::RightControl,
        }
    }

    pub(crate) fn binding(self, action: BindingAction) -> KeyCode {
        match action {
            BindingAction::MoveLeft => self.move_left,
            BindingAction::MoveRight => self.move_right,
            BindingAction::Jump => self.jump,
            BindingAction::Shoot => self.shoot,
        }
    }

    pub(crate) fn set_binding(&mut self, action: BindingAction, key: KeyCode) {
        match action {
            BindingAction::MoveLeft => self.move_left = key,
            BindingAction::MoveRight => self.move_right = key,
            BindingAction::Jump => self.jump = key,
            BindingAction::Shoot => self.shoot = key,
        }
    }
}

pub(crate) fn is_forbidden_key(key: KeyCode) -> bool {
    matches!(key, KeyCode::Escape | KeyCode::Unknown)
}

pub(crate) fn keycode_to_storage_string(key: KeyCode) -> String {
    format!("{key:?}")
}

pub(crate) fn keycode_to_display_label(key: KeyCode) -> String {
    match key {
        KeyCode::Left => "LEFT".to_owned(),
        KeyCode::Right => "RIGHT".to_owned(),
        KeyCode::Up => "UP".to_owned(),
        KeyCode::Down => "DOWN".to_owned(),
        KeyCode::LeftControl => "LCTRL".to_owned(),
        KeyCode::RightControl => "RCTRL".to_owned(),
        KeyCode::LeftShift => "LSHIFT".to_owned(),
        KeyCode::RightShift => "RSHIFT".to_owned(),
        KeyCode::LeftAlt => "LALT".to_owned(),
        KeyCode::RightAlt => "RALT".to_owned(),
        KeyCode::LeftSuper => "LSUP".to_owned(),
        KeyCode::RightSuper => "RSUP".to_owned(),
        KeyCode::Space => "SPACE".to_owned(),
        KeyCode::Enter => "ENTER".to_owned(),
        KeyCode::Tab => "TAB".to_owned(),
        KeyCode::Backspace => "BACK".to_owned(),
        KeyCode::PageUp => "PGUP".to_owned(),
        KeyCode::PageDown => "PGDN".to_owned(),
        KeyCode::CapsLock => "CAPS".to_owned(),
        KeyCode::ScrollLock => "SCRLK".to_owned(),
        KeyCode::NumLock => "NUMLK".to_owned(),
        KeyCode::PrintScreen => "PRTSC".to_owned(),
        KeyCode::Pause => "PAUSE".to_owned(),
        KeyCode::KpEnter => "KPENT".to_owned(),
        KeyCode::KpDecimal => "KP.".to_owned(),
        KeyCode::KpDivide => "KP/".to_owned(),
        KeyCode::KpMultiply => "KP*".to_owned(),
        KeyCode::KpSubtract => "KP-".to_owned(),
        KeyCode::KpAdd => "KP+".to_owned(),
        KeyCode::KpEqual => "KP=".to_owned(),
        _ => keycode_to_storage_string(key).to_uppercase(),
    }
}

pub(crate) fn keycode_from_storage_string(value: &str) -> Option<KeyCode> {
    match value.trim() {
        "Space" => Some(KeyCode::Space),
        "Apostrophe" => Some(KeyCode::Apostrophe),
        "Comma" => Some(KeyCode::Comma),
        "Minus" => Some(KeyCode::Minus),
        "Period" => Some(KeyCode::Period),
        "Slash" => Some(KeyCode::Slash),
        "Key0" => Some(KeyCode::Key0),
        "Key1" => Some(KeyCode::Key1),
        "Key2" => Some(KeyCode::Key2),
        "Key3" => Some(KeyCode::Key3),
        "Key4" => Some(KeyCode::Key4),
        "Key5" => Some(KeyCode::Key5),
        "Key6" => Some(KeyCode::Key6),
        "Key7" => Some(KeyCode::Key7),
        "Key8" => Some(KeyCode::Key8),
        "Key9" => Some(KeyCode::Key9),
        "Semicolon" => Some(KeyCode::Semicolon),
        "Equal" => Some(KeyCode::Equal),
        "A" => Some(KeyCode::A),
        "B" => Some(KeyCode::B),
        "C" => Some(KeyCode::C),
        "D" => Some(KeyCode::D),
        "E" => Some(KeyCode::E),
        "F" => Some(KeyCode::F),
        "G" => Some(KeyCode::G),
        "H" => Some(KeyCode::H),
        "I" => Some(KeyCode::I),
        "J" => Some(KeyCode::J),
        "K" => Some(KeyCode::K),
        "L" => Some(KeyCode::L),
        "M" => Some(KeyCode::M),
        "N" => Some(KeyCode::N),
        "O" => Some(KeyCode::O),
        "P" => Some(KeyCode::P),
        "Q" => Some(KeyCode::Q),
        "R" => Some(KeyCode::R),
        "S" => Some(KeyCode::S),
        "T" => Some(KeyCode::T),
        "U" => Some(KeyCode::U),
        "V" => Some(KeyCode::V),
        "W" => Some(KeyCode::W),
        "X" => Some(KeyCode::X),
        "Y" => Some(KeyCode::Y),
        "Z" => Some(KeyCode::Z),
        "LeftBracket" => Some(KeyCode::LeftBracket),
        "Backslash" => Some(KeyCode::Backslash),
        "RightBracket" => Some(KeyCode::RightBracket),
        "GraveAccent" => Some(KeyCode::GraveAccent),
        "World1" => Some(KeyCode::World1),
        "World2" => Some(KeyCode::World2),
        "Escape" => Some(KeyCode::Escape),
        "Enter" => Some(KeyCode::Enter),
        "Tab" => Some(KeyCode::Tab),
        "Backspace" => Some(KeyCode::Backspace),
        "Insert" => Some(KeyCode::Insert),
        "Delete" => Some(KeyCode::Delete),
        "Right" => Some(KeyCode::Right),
        "Left" => Some(KeyCode::Left),
        "Down" => Some(KeyCode::Down),
        "Up" => Some(KeyCode::Up),
        "PageUp" => Some(KeyCode::PageUp),
        "PageDown" => Some(KeyCode::PageDown),
        "Home" => Some(KeyCode::Home),
        "End" => Some(KeyCode::End),
        "CapsLock" => Some(KeyCode::CapsLock),
        "ScrollLock" => Some(KeyCode::ScrollLock),
        "NumLock" => Some(KeyCode::NumLock),
        "PrintScreen" => Some(KeyCode::PrintScreen),
        "Pause" => Some(KeyCode::Pause),
        "F1" => Some(KeyCode::F1),
        "F2" => Some(KeyCode::F2),
        "F3" => Some(KeyCode::F3),
        "F4" => Some(KeyCode::F4),
        "F5" => Some(KeyCode::F5),
        "F6" => Some(KeyCode::F6),
        "F7" => Some(KeyCode::F7),
        "F8" => Some(KeyCode::F8),
        "F9" => Some(KeyCode::F9),
        "F10" => Some(KeyCode::F10),
        "F11" => Some(KeyCode::F11),
        "F12" => Some(KeyCode::F12),
        "F13" => Some(KeyCode::F13),
        "F14" => Some(KeyCode::F14),
        "F15" => Some(KeyCode::F15),
        "F16" => Some(KeyCode::F16),
        "F17" => Some(KeyCode::F17),
        "F18" => Some(KeyCode::F18),
        "F19" => Some(KeyCode::F19),
        "F20" => Some(KeyCode::F20),
        "F21" => Some(KeyCode::F21),
        "F22" => Some(KeyCode::F22),
        "F23" => Some(KeyCode::F23),
        "F24" => Some(KeyCode::F24),
        "F25" => Some(KeyCode::F25),
        "Kp0" => Some(KeyCode::Kp0),
        "Kp1" => Some(KeyCode::Kp1),
        "Kp2" => Some(KeyCode::Kp2),
        "Kp3" => Some(KeyCode::Kp3),
        "Kp4" => Some(KeyCode::Kp4),
        "Kp5" => Some(KeyCode::Kp5),
        "Kp6" => Some(KeyCode::Kp6),
        "Kp7" => Some(KeyCode::Kp7),
        "Kp8" => Some(KeyCode::Kp8),
        "Kp9" => Some(KeyCode::Kp9),
        "KpDecimal" => Some(KeyCode::KpDecimal),
        "KpDivide" => Some(KeyCode::KpDivide),
        "KpMultiply" => Some(KeyCode::KpMultiply),
        "KpSubtract" => Some(KeyCode::KpSubtract),
        "KpAdd" => Some(KeyCode::KpAdd),
        "KpEnter" => Some(KeyCode::KpEnter),
        "KpEqual" => Some(KeyCode::KpEqual),
        "LeftShift" => Some(KeyCode::LeftShift),
        "LeftControl" => Some(KeyCode::LeftControl),
        "LeftAlt" => Some(KeyCode::LeftAlt),
        "LeftSuper" => Some(KeyCode::LeftSuper),
        "RightShift" => Some(KeyCode::RightShift),
        "RightControl" => Some(KeyCode::RightControl),
        "RightAlt" => Some(KeyCode::RightAlt),
        "RightSuper" => Some(KeyCode::RightSuper),
        "Menu" => Some(KeyCode::Menu),
        "Back" => Some(KeyCode::Back),
        "Unknown" => Some(KeyCode::Unknown),
        _ => None,
    }
}

mod keycode_serde {
    use super::{keycode_from_storage_string, keycode_to_storage_string};
    use macroquad::prelude::KeyCode;
    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(key: &KeyCode, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&keycode_to_storage_string(*key))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<KeyCode, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        keycode_from_storage_string(&value)
            .ok_or_else(|| D::Error::custom(format!("unsupported key code `{value}`")))
    }
}


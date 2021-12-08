//! This is the custom config system for d3_settings. It is not compatible with
//! i3 and will overwrite any existing i3 config if it exists.
//!
//! TODO: Implement some form of config extraction for existing i3 users

use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

/// The key binding that will be used to perform actions inside of the window
/// manager. Alt is generally considered the default in i3, however, d3 will
/// default to the meta (windows) key
#[derive(Debug, Deserialize, Serialize)]
pub enum ModifierEnum {
    Alt,
    Meta,
}

impl Default for ModifierEnum {
    fn default() -> Self {
        ModifierEnum::Meta
    }
}

impl Display for ModifierEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModifierEnum::Meta => write!(f, "Mod4"),
            ModifierEnum::Alt => write!(f, "Mod1"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum KeyCode {
    Mod,
    Ctrl,
    Shift,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    N0,
    Tidle,
    Dash,
    Equal,
    Backspace,
    Enter,
    Space,
    Tab,
    Left,
    Right,
    Up,
    Down,
    ForwardSlash,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Escape,
    SemiColon,
}

#[derive(Deserialize, Serialize)]
pub struct ExecParams {
    pub command: String,
    pub no_startup_id: Option<bool>,

    /// This only works when you are specifying a startup application. If this
    /// option is true, the command will always be executed after a restart.
    /// if it is false or isn't specified, it will run when first launching
    /// i3, but not after a restart.
    pub always_run: Option<bool>,
}

impl ExecParams {
    pub fn new(command: &str) -> Self {
        ExecParams {
            command: command.to_string(),
            no_startup_id: None,
            always_run: None,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Deserialize, Serialize)]
pub enum Action {
    Kill,
    Exec(ExecParams),
    Focus(Direction),
    Move(Direction),
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub modifier: ModifierEnum,

    pub preferred_terminal: Option<String>,

    pub key_bindings: HashMap<Vec<KeyCode>, Action>,
    pub startup_apps: Vec<Action>,

    /// Allows to specify other files that will be included within the outputted
    /// i3 config file, allowing for extra customization.
    ///
    /// For further information, please see the i3 user manual:
    /// <https://i3wm.org/docs/userguide.html#include>
    pub includes: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        let mut key_bindings = HashMap::new();

        key_bindings.insert(
            vec![KeyCode::Mod, KeyCode::Enter],
            Action::Exec(ExecParams::new("i3-sensible-terminal")),
        );

        // TODO: Native application launcher over ulauncher
        key_bindings.insert(
            vec![KeyCode::Mod, KeyCode::D],
            Action::Exec(ExecParams::new("ulauncher")),
        );

        key_bindings.insert(vec![KeyCode::Mod, KeyCode::Q], Action::Kill);

        // TODO: Custom file manager
        key_bindings.insert(vec![], Action::Exec(ExecParams::new("pcmanfm")));

        // TODO: Find an ok music player or write my own, bind to F5

        // Vim focus bindings
        key_bindings.insert(
            vec![KeyCode::Mod, KeyCode::J],
            Action::Focus(Direction::Left),
        );
        key_bindings.insert(
            vec![KeyCode::Mod, KeyCode::K],
            Action::Focus(Direction::Down),
        );
        key_bindings.insert(vec![KeyCode::Mod, KeyCode::L], Action::Focus(Direction::Up));
        key_bindings.insert(
            vec![KeyCode::Mod, KeyCode::SemiColon],
            Action::Focus(Direction::Right),
        );

        // Sane focus bindings
        key_bindings.insert(
            vec![KeyCode::Mod, KeyCode::Left],
            Action::Focus(Direction::Left),
        );
        key_bindings.insert(
            vec![KeyCode::Mod, KeyCode::Down],
            Action::Focus(Direction::Down),
        );
        key_bindings.insert(
            vec![KeyCode::Mod, KeyCode::Up],
            Action::Focus(Direction::Up),
        );
        key_bindings.insert(
            vec![KeyCode::Mod, KeyCode::Right],
            Action::Focus(Direction::Right),
        );

        // Vim move bindings
        key_bindings.insert(
            vec![KeyCode::Mod, KeyCode::Shift, KeyCode::J],
            Action::Move(Direction::Left),
        );
        key_bindings.insert(
            vec![KeyCode::Mod, KeyCode::Shift, KeyCode::K],
            Action::Move(Direction::Down),
        );
        key_bindings.insert(
            vec![KeyCode::Mod, KeyCode::Shift, KeyCode::L],
            Action::Move(Direction::Up),
        );
        key_bindings.insert(
            vec![KeyCode::Mod, KeyCode::Shift, KeyCode::SemiColon],
            Action::Move(Direction::Right),
        );

        // Sane move bindings
        key_bindings.insert(
            vec![KeyCode::Mod, KeyCode::Shift, KeyCode::Left],
            Action::Move(Direction::Left),
        );
        key_bindings.insert(
            vec![KeyCode::Mod, KeyCode::Shift, KeyCode::Down],
            Action::Move(Direction::Down),
        );
        key_bindings.insert(
            vec![KeyCode::Mod, KeyCode::Shift, KeyCode::Up],
            Action::Move(Direction::Up),
        );
        key_bindings.insert(
            vec![KeyCode::Mod, KeyCode::Shift, KeyCode::Right],
            Action::Move(Direction::Right),
        );

        Self {
            modifier: ModifierEnum::default(),
            includes: vec![],
            preferred_terminal: None,
            key_bindings,
            startup_apps: vec![],
        }
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "# This config file was generated by d3, any modifications will be overwritten\n\n"
        )?;
        write!(f, "set $mod {}\n", self.modifier)
    }
}

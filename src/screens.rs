use bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub(crate) enum Screen {
    #[default]
    MainMenu,
    Gameplay,
}

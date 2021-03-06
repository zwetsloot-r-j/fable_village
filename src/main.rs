mod systems;
mod states;
mod components;
mod bundles;
mod extensions;
mod loaders;
mod events;

use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    core::{
        transform::TransformBundle,
    },
    input::{InputBundle, StringBindings},
    winit::{
        MouseButton,
    },
    ui::{
        RenderUi,
        UiBundle,
    },
};
use crate::states::LoadingState;
use crate::{
    bundles::{
        TouchInputBundle,
        touch_input_bundle::{
            LogLevel,
        },
    },
    events::{
        GameStateEvent,
        GameStateEventReader,
    },
    systems::{
        GlobalHotkeySystem,
        ButtonDispatchSystem,
    },
};

use serde::Deserialize;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let bindings_path = app_root.join("config").join("bindings.ron");
    let assets_dir = app_root.join("assets");

    let render_to_window = RenderToWindow::from_config_path(display_config_path)?
        .with_clear([0.0, 0.0, 0.0, 1.0])
        ;

    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(render_to_window)
        .with_plugin(RenderFlat2D::default())
        .with_plugin(RenderUi::default())
        ;

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(bindings_path)?
        ;

    let game_data = GameDataBuilder::default()
        .with_bundle(rendering_bundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(TouchInputBundle::new()
            .with_mouse_simulation(MouseButton::Left)
            .with_touchables()
            //.with_logging(LogLevel::Touchables)
        )?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(GlobalHotkeySystem, "global_hotkey_system", &[])
        .with(ButtonDispatchSystem, "button_dispatch_system", &[])
        ;

    let mut game = CoreApplication::<_, GameStateEvent, GameStateEventReader>::new(
        assets_dir,
        LoadingState::default(),
        game_data
    )?;

    game.run();

    Ok(())
}

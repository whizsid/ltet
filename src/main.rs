use amethyst::{
    animation::{AnimationBundle},
    core::transform::{
        TransformBundle,
        Transform
    },
    input::{
        StringBindings
    },
    prelude::*,
    renderer::{
        plugins::{RenderPbr3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
        light::{
            SunLight, Light
        }
    },
    utils::{
        application_root_dir,
        auto_fov::AutoFovSystem
    },
    assets::{
        ProgressCounter,
        Loader,
        AssetStorage,
        Handle,
    },
    controls:: {
        FlyControlBundle
    }
};

use amethyst_gltf::{
    GltfSceneLoaderSystemDesc,
    GltfSceneFormat,
    GltfSceneAsset
};

mod entities;
use entities::camera::Camera;
use entities::soldier::Soldier;
use entities::behaviour::{r#move, rotate, RotateDirection};

struct LoadingState {
    progress_counter: ProgressCounter,
    soldier_handle: Option<Handle<GltfSceneAsset>>
}

struct GamePlayState {
    soldier_handle: Handle<GltfSceneAsset>
}

impl SimpleState for GamePlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut camera = Camera::new(data.world);
        let soldier = Soldier::new(data.world, self.soldier_handle.clone());

        r#move(data.world,&mut camera,
            (0.0,0.0,-12.0)
        );

        rotate(data.world,&mut camera,
            RotateDirection::Horizontal,
            std::f32::consts::PI
        );

        let light:Light = SunLight::default().into();

        data.world.create_entity().with(light).build();
    }

}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {

        let loader = &data.world.read_resource::<Loader>();

        let soldier_asset_handler = loader.load(
            "soldier.gltf",
            GltfSceneFormat::default(),
            &mut self.progress_counter,
            &data.world.read_resource::<AssetStorage<GltfSceneAsset>>(),
        );
        self.soldier_handle = Some(soldier_asset_handler);
    }

    fn update(
        &mut self,
        _data: &mut StateData<'_, GameData<'_, '_>>,
    ) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            Trans::Switch(Box::new(GamePlayState {
                soldier_handle: self.soldier_handle
                    .take()
                    .expect(
                        "Expected `texture_handle` to exist when \
                        `progress_counter` is complete."
                    ),
            }))
        } else {
            Trans::None
        }
    }

}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
        .with(AutoFovSystem::default(), "auto_fov", &[])
        .with_system_desc(GltfSceneLoaderSystemDesc::default(), "gltf_loader", &[])
        .with_bundle(
            AnimationBundle::<usize, Transform>::new("animation_control", "sampler_interpolation")
                .with_dep(&["gltf_loader"]),
        )?
        .with_bundle(
            FlyControlBundle::<StringBindings>::new(
                Some(String::from("move_x")),
                Some(String::from("move_y")),
                Some(String::from("move_z")),
            )
            .with_sensitivity(0.1, 0.1),
        )?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderPbr3D::default()),
        )?
        .with_bundle(TransformBundle::new())?;

    let mut game = Application::new(assets_dir, LoadingState {
        progress_counter: ProgressCounter::new(),
        soldier_handle: None
    }, game_data)?;
    game.run();

    Ok(())
}

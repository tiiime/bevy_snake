use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use iyes_loopless::prelude::*;

fn main() {
    let board = BoardSize {
        x: 9,
        y: 9,
        window_width: 720.,
        window_height: 720.,
    };

    App::new()
        .insert_resource(WindowDescriptor {
            width: board.window_width,
            height: board.window_height,
            ..default()
        })
        .insert_resource(board)
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup_world)
        .add_system(system_map_block_to_board)
        .run()
}

/// 初始化棋盘资源
fn setup_world(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
    add_block_sprit(commands)
}

// 添加一个 block sprite
fn add_block_sprit(mut commands: Commands) {
    let id = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1., 0., 0.),
                ..default()
            },
            ..default()
        })
        .id();

    commands.entity(id).insert(Block {
        entity: id,
        position: Position(1, 1),
    });
}

fn system_map_block_to_board(mut query: Query<(&Block, &mut Transform)>, res: Res<BoardSize>) {
    let res = res.as_ref();

    query.for_each_mut(|(block, mut transform)| {
        transform.scale = Vec3::new(res.block_width(), res.block_height(), 0.);
        transform.translation.x = block.position.0 as f32 * res.block_width();
        transform.translation.y = block.position.1 as f32 * res.block_height();
    });
}


/// 棋盘格子数量
struct BoardSize {
    x: i32,
    y: i32,
    window_width: f32,
    window_height: f32,
}

impl BoardSize {
    fn block_width(&self) -> f32 {
        return self.window_width / (self.x as f32);
    }
    fn block_height(&self) -> f32 {
        return self.window_height / (self.y as f32);
    }
}
/// 棋盘格子坐标
struct Position(i32, i32);

/// 需要绘制的 block
#[derive(Component)]
struct Block {
    position: Position,
    entity: Entity,
}

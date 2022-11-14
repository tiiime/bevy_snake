use std::time::Duration;

use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};
use bevy_inspector_egui::{egui::Key, WorldInspectorPlugin};
use iyes_loopless::prelude::*;

fn main() {
    let board = BoardConfig {
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
        .add_fixed_timestep(Duration::from_millis(300), "step")
        .add_fixed_timestep_system("step", 0, system_snake_step.label("move_forward"))
        .add_fixed_timestep_system("step", 0, system_snake_drop_tail.after("move_forward"))
        .add_system(system_keyevent)
        .run()
}

/// 初始化棋盘资源
fn setup_world(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1., 0., 0.),
                ..default()
            },
            ..default()
        })
        .insert(Block {
            position: Position { x: 0, y: 0 },
        })
        .insert(PrevBlock {
            prev_entity: Option::None,
        })
        .insert(Head {
            direction: Direction::Up,
        })
        .insert(Tail);
}

fn system_map_block_to_board(mut query: Query<(&Block, &mut Transform)>, res: Res<BoardConfig>) {
    let board = res.as_ref();

    query.for_each_mut(|(block, mut transform)| {
        transform.scale = Vec3::new(board.block_width(), board.block_height(), 0.);
        transform.translation.x = (board.block_width() - board.window_width) / 2.
            + block.position.x as f32 * board.block_width();
        transform.translation.y = (board.block_height() - board.window_height) / 2.
            + block.position.y as f32 * board.block_height();
    });
}

fn system_snake_step(
    mut commands: Commands,
    mut queryHead: Query<(Entity, &Head, &Block, &mut PrevBlock)>,
    board: Res<BoardConfig>,
) {
    let board = board.as_ref();
    if let Some((entity, head, block, mut prevBlock)) = queryHead.iter_mut().next() {
        let next = block.next_block(head.direction);
        if board.validate(next.position) {
            let prev = commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(1., 0., 0.),
                        ..default()
                    },
                    ..default()
                })
                .insert(next)
                .insert(*head)
                .insert(PrevBlock {
                    prev_entity: Option::None,
                })
                .id();
            prevBlock.prev_entity = Some(prev);
            commands.entity(entity).remove::<Head>();
        }
    }
}

fn system_snake_drop_tail(
    mut commands: Commands,
    mut queryTail: Query<(Entity, &PrevBlock), With<Tail>>,
) {
    if let Some((entity, prev)) = queryTail.into_iter().next() {
        if let Some(p) = prev.prev_entity {
            commands.entity(p).insert(Tail);
        }
        commands.entity(entity).despawn()
    }
}

fn system_keyevent(mut input: EventReader<KeyboardInput>, mut query: Query<&mut Head>) {
    if let Some(mut head) = query.iter_mut().next() {
        for ev in input.iter() {
            if let ButtonState::Pressed = ev.state {
                head.direction = Direction::from(ev.key_code.unwrap())
            }
        }
    }
}

/// 棋盘格子数量
struct BoardConfig {
    x: i32,
    y: i32,
    window_width: f32,
    window_height: f32,
}

impl BoardConfig {
    fn block_width(&self) -> f32 {
        return self.window_width / (self.x as f32);
    }
    fn block_height(&self) -> f32 {
        return self.window_height / (self.y as f32);
    }

    fn validate(&self, position: Position) -> bool {
        position.x >= 0 && position.x < self.x && position.y >= 0 && position.y < self.y
    }
}
/// 棋盘格子坐标
#[derive(Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

/// 需要绘制的 block
#[derive(Component)]
struct Block {
    position: Position,
}

impl Block {
    fn next_block(&self, direction: Direction) -> Block {
        let position = match direction {
            Direction::Up => Position {
                y: self.position.y + 1,
                ..self.position
            },
            Direction::Down => Position {
                y: self.position.y - 1,
                ..self.position
            },
            Direction::Left => Position {
                x: self.position.x - 1,
                ..self.position
            },
            Direction::Right => Position {
                x: self.position.x + 1,
                ..self.position
            },
        };

        Block { position: position }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<KeyCode> for Direction {
    fn from(code: KeyCode) -> Self {
        match code {
            KeyCode::Up => Direction::Up,
            KeyCode::Down => Direction::Down,
            KeyCode::Left => Direction::Left,
            KeyCode::Right => Direction::Right,
            _ => Direction::Up,
        }
    }
}
#[derive(Component, Clone, Copy)]
struct Head {
    direction: Direction,
}

#[derive(Component)]
struct PrevBlock {
    prev_entity: Option<Entity>,
}

#[derive(Component)]
struct Tail;
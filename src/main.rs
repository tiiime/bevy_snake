use std::time::Duration;

use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};
use bevy_inspector_egui::*;
use iyes_loopless::prelude::*;
use rand::prelude::random;

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
        .add_startup_system(system_create_food)
        .add_system(system_map_block_to_board)
        .add_fixed_timestep(Duration::from_millis(300), "step")
        .add_fixed_timestep_child_stage("step")
        .add_fixed_timestep_system("step", 0, system_snake_step.label("move_forward"))
        .add_fixed_timestep_system(
            "step",
            0,
            system_check_eat.label("check_eat").after("move_forward"),
        )
        .add_fixed_timestep_system("step", 1, system_snake_drop_tail.after("check_eat"))
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
        .insert(Position { x: 0, y: 0 })
        .insert(PrevBlock {
            prev_entity: Option::None,
        })
        .insert(Head {
            direction: Direction::Up,
        })
        .insert(Tail);
}

fn system_create_food(mut commands: Commands, board: Res<BoardConfig>) {
    let board = board.as_ref();

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0., 1., 0.),
                ..default()
            },
            ..default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * board.x as f32) as i32,
            y: (random::<f32>() * board.x as f32) as i32,
        });
}

fn system_map_block_to_board(mut query: Query<(&Position, &mut Transform)>, res: Res<BoardConfig>) {
    let board = res.as_ref();

    query.for_each_mut(|(position, mut transform)| {
        transform.scale = Vec3::new(board.block_width(), board.block_height(), 0.);
        transform.translation.x = (board.block_width() - board.window_width) / 2.
            + position.x as f32 * board.block_width();
        transform.translation.y = (board.block_height() - board.window_height) / 2.
            + position.y as f32 * board.block_height();
    });
}

fn system_snake_step(
    mut commands: Commands,
    mut query_head: Query<(Entity, &Head, &Position, &mut PrevBlock)>,
    board: Res<BoardConfig>,
) {
    let board = board.as_ref();
    if let Some((entity, head, position, mut prev_block)) = query_head.iter_mut().next() {
        let next = position.calc_next(head.direction);
        if board.validate(next) {
            let new_head = commands
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
            prev_block.prev_entity = Some(new_head);
            commands.entity(entity).remove::<Head>();
        }
    }
}

fn system_check_eat(
    mut commands: Commands,
    food: Query<(Entity, &Position), With<Food>>,
    head: Query<&Position, With<Head>>,
    tail: Query<Entity, With<Tail>>,
    board: Res<BoardConfig>,
) {
    if let Some((food_entity, food_position)) = food.iter().next() {
        if let Some(head) = head.iter().next() {
            if let Some(tail) = tail.iter().next() {
                if food_position.x == head.x && food_position.y == head.y {
                    commands.entity(tail).insert(AppendTail);
                    info!("append#{:?}", tail);
                    commands.entity(food_entity).despawn();

                    system_create_food(commands, board)
                }
            }
        }
    }
}

fn system_snake_drop_tail(
    mut commands: Commands,
    query_tail: Query<(Entity, &PrevBlock, Option<&AppendTail>), With<Tail>>,
) {
    if let Some((entity, prev, option_append)) = query_tail.into_iter().next() {
        info!("drop tail#{:?}", entity);
        if let Some(_) = option_append {
            info!("skip drop");
            commands.entity(entity).remove::<AppendTail>();
            return;
        }
        info!("drop");
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
#[derive(Component, Clone, Copy, PartialEq, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn calc_next(&self, direction: Direction) -> Position {
        match direction {
            Direction::Up => Position {
                y: self.y + 1,
                ..*self
            },
            Direction::Down => Position {
                y: self.y - 1,
                ..*self
            },
            Direction::Left => Position {
                x: self.x - 1,
                ..*self
            },
            Direction::Right => Position {
                x: self.x + 1,
                ..*self
            },
        }
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

#[derive(Component)]
struct Food;

#[derive(Component, Debug)]
struct AppendTail;

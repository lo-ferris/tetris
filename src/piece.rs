use crate::board::*;
use bevy::prelude::*;

// 四格骨牌
#[derive(Component)]
pub enum Piece {
    // ####
    I,

    // #
    // ###
    J,

    //   #
    // ###
    L,

    // ##
    // ##
    O,

    //  ##
    // ##
    S,

    //  #
    // ###
    T,

    // ##
    //  ##
    Z,
}

// 可移动方向
#[derive(Component)]
pub struct Movable {
    pub can_down: bool,
    pub can_left: bool,
    pub can_right: bool,
}

// 自动向下移动四格骨牌计时器
#[derive(Component, Deref, DerefMut)]
pub struct AutoMovePieceDownTimer(pub Timer);

pub fn spawn_piece(mut commands: Commands) {
    let new_sprite_bundle = |block: &Block| SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.7, 0.7, 0.7),
            ..default()
        },
        transform: Transform {
            scale: Vec3::new(BLOCK_LENGTH, BLOCK_LENGTH, BLOCK_LENGTH),
            translation: block.translation(),
            ..default()
        },
        ..default()
    };
    let block = Block::new(0, 19);
    commands
        .spawn(Piece::I)
        .insert(new_sprite_bundle(&block))
        .insert(block)
        .insert(Movable {
            can_down: true,
            can_left: true,
            can_right: true,
        })
        .insert(AutoMovePieceDownTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )));
    let block = Block::new(1, 19);
    commands
        .spawn(Piece::I)
        .insert(new_sprite_bundle(&block))
        .insert(block)
        .insert(Movable {
            can_down: true,
            can_left: true,
            can_right: true,
        })
        .insert(AutoMovePieceDownTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )));
    let block = Block::new(2, 19);
    commands
        .spawn(Piece::I)
        .insert(new_sprite_bundle(&block))
        .insert(block)
        .insert(Movable {
            can_down: true,
            can_left: true,
            can_right: true,
        })
        .insert(AutoMovePieceDownTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )));
    let block = Block::new(3, 19);
    commands
        .spawn(Piece::I)
        .insert(new_sprite_bundle(&block))
        .insert(block)
        .insert(Movable {
            can_down: true,
            can_left: true,
            can_right: true,
        })
        .insert(AutoMovePieceDownTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )));
}

// 手动移动四格骨牌
pub fn manually_move_piece(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Block, &mut Transform, &Movable), With<Piece>>,
) {
    if keyboard_input.just_pressed(KeyCode::Left) {
        for (mut block, mut transform, movable) in &mut query {
            if movable.can_left {
                block.x -= 1;
                transform.translation = block.translation();
            }
        }
    } else if keyboard_input.just_pressed(KeyCode::Right) {
        for (mut block, mut transform, movable) in &mut query {
            if movable.can_right {
                block.x += 1;
                transform.translation = block.translation();
            }
        }
    } else if keyboard_input.just_pressed(KeyCode::Down) {
        for (mut block, mut transform, movable) in &mut query {
            if movable.can_down {
                block.y -= 1;
                transform.translation = block.translation();
            }
        }
    }
}

// 自动向下移动四格骨牌
pub fn auto_move_piece_down(
    time: Res<Time>,
    mut query: Query<
        (
            &mut AutoMovePieceDownTimer,
            &mut Block,
            &mut Transform,
            &Movable,
        ),
        With<Piece>,
    >,
) {
    for (mut timer, mut block, mut transform, movable) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if movable.can_down {
                block.y -= 1;
                transform.translation = block.translation();
            }
        }
    }
}

// 检查碰撞
pub fn check_collision(
    mut piece_query: Query<(&mut Block, &mut Movable), With<Piece>>,
    board_query: Query<&Block, Without<Piece>>,
) {
    let mut can_down = true;
    let mut can_left = true;
    let mut can_right = true;

    // 检查是否碰撞边界
    for (block, _) in &mut piece_query {
        if block.x == 0 {
            // 碰撞左边界
            can_left = false;
        }
        if block.x == 9 {
            // 碰撞右边界
            can_right = false;
        }
        if block.y == 0 {
            // 碰撞下边界
            can_down = false;
        }
    }

    // 检查是否碰撞面板方块
    for (block, _) in &piece_query {
        for board_block in &board_query {
            if board_block.x == block.x - 1 {
                // 左侧碰撞
                can_left = false;
            }
            if board_block.x == block.x + 1 {
                // 右侧碰撞
                can_right = false;
            }
            if board_block.y == block.y - 1 {
                // 下侧碰撞
                can_down = false;
            }
        }
    }

    // 更新Movable
    for (_, mut movable) in &mut piece_query {
        movable.can_left = can_left;
        movable.can_right = can_right;
        movable.can_down = can_down;
    }
}

// 旋转四格骨牌
pub fn rotate_piece(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Block, &mut Transform), With<Piece>>,
) {
}

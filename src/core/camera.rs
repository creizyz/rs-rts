use crate::core::command::{Command, CommandHandler};
use crate::core::{FromString, SerializeEnum};
use bevy::math::Vec3;
use bevy::prelude::{Camera3d, Query, Transform, With};
use std::any::Any;
use std::fmt::Display;

#[repr(u16)]
enum CameraCommand {
    MoveForward  = 0x1 << 0,
    MoveBackward = 0x1 << 1,
    MoveUp       = 0x1 << 8,
    MoveDown     = 0x1 << 9,
    MoveLeft     = 0x1 << 2,
    MoveRight    = 0x1 << 3,
    RotateLeft   = 0x1 << 4,
    RotateRight  = 0x1 << 5,
    ZoomIn       = 0x1 << 6,
    ZoomOut      = 0x1 << 7,
}

impl Display for CameraCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            CameraCommand::MoveForward  => "camera.moveForward",
            CameraCommand::MoveBackward => "camera.moveBackward",
            CameraCommand::MoveUp       => "camera.moveUp",
            CameraCommand::MoveDown     => "camera.moveDown",
            CameraCommand::MoveLeft     => "camera.moveLeft",
            CameraCommand::MoveRight    => "camera.moveRight",
            CameraCommand::RotateLeft   => "camera.rotateLeft",
            CameraCommand::RotateRight  => "camera.rotateRight",
            CameraCommand::ZoomIn       => "camera.zoomIn",
            CameraCommand::ZoomOut      => "camera.zoomOut",
        };
        write!(f, "{}", str)
    }
}

impl FromString for CameraCommand {
    fn from_string(s: &str) -> Option<Self> {
        match s {
            "CameraMoveForward"  => Some(CameraCommand::MoveForward),
            "CameraMoveBackward" => Some(CameraCommand::MoveBackward),
            "CameraMoveUp"       => Some(CameraCommand::MoveUp),
            "CameraMoveDown"     => Some(CameraCommand::MoveDown),
            "CameraMoveLeft"     => Some(CameraCommand::MoveLeft),
            "CameraMoveRight"    => Some(CameraCommand::MoveRight),
            "CameraRotateLeft"   => Some(CameraCommand::RotateLeft),
            "CameraRotateRight"  => Some(CameraCommand::RotateRight),
            "CameraZoomIn"       => Some(CameraCommand::ZoomIn),
            "CameraZoomOut"      => Some(CameraCommand::ZoomOut),
            _ => None,
        }
    }
}

impl SerializeEnum for CameraCommand { }

impl Command for CameraCommand {
    fn as_any(&self) -> &dyn Any { self }
}

pub struct CameraSystem {
    speed: f32,
    forward_move : i8,
    up_move : i8,
    right_move : i8,
}

impl CameraSystem {
    pub fn new() -> Self {
        Self {
            speed: 1.0,
            forward_move: 0,
            up_move: 0,
            right_move: 0,
        }
    }

    pub fn with_speed(speed: f32) -> Self {
        Self {
            speed,
            forward_move: 0,
            up_move: 0,
            right_move: 0,
        }
    }

    pub fn update(&mut self, dt: f32, query: &mut Query<&mut Transform, With<Camera3d>>) {
        for mut transform in query.iter_mut() {
            let forward = transform.forward();
            let right = transform.right();

            let forward_xz = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
            let right_xz = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();

            if self.forward_move != 0 {
                transform.translation += forward_xz * self.forward_move as f32 * self.speed * dt;
            }

            if self.right_move != 0 {
                transform.translation += right_xz * self.right_move as f32 * self.speed * dt;
            }

            if self.up_move != 0 {
                transform.translation += Vec3::new(0.0, self.up_move as f32 * self.speed * dt, 0.0);
            }
        }
    }
}

impl CommandHandler<CameraCommand> for CameraSystem {
    fn handle_command(&mut self, commands: &Vec<CameraCommand>) {
        let mut forward_move : i8 = 0;
        let mut up_move : i8 = 0;
        let mut right_move : i8 = 0;

        for command in commands {
            match command {
                CameraCommand::MoveForward  => forward_move += 1,
                CameraCommand::MoveBackward => forward_move -= 1,
                CameraCommand::MoveUp       => up_move += 1,
                CameraCommand::MoveDown     => up_move -= 1,
                CameraCommand::MoveLeft     => right_move -= 1,
                CameraCommand::MoveRight    => right_move += 1,
                // CameraCommand::RotateLeft   => ,
                // CameraCommand::RotateRight  => ,
                // CameraCommand::ZoomIn       => ,
                // CameraCommand::ZoomOut      => ,
                _ => (),
            }
        }

        self.forward_move = forward_move.clamp(-1, 1);
        self.up_move = up_move.clamp(-1, 1);
        self.right_move = right_move.clamp(-1, 1);
    }
}

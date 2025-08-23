use bevy::math::Vec3;
use bevy::prelude::{Camera3d, Query, Transform, With};
use crate::core::{Action, SerializableEnum};
use crate::core::input::{CommandHandler, Command};

enum CameraAction {
    CameraMove{ x: f32, y: f32, z: f32 },
    CameraRotate,
    CameraZoom,
    CameraMoveTo{ x: f32, y: f32, z: f32 },
    CameraLookAt{ x: f32, y: f32, z: f32 },
    CameraReset,
}

impl SerializableEnum for CameraAction {
    fn to_string(&self) -> &'static str {
        todo!()
    }

    fn from_string(string: &str) -> Option<Self>
    where
        Self: Sized
    {
        todo!()
    }
}

impl Action for CameraAction {

}

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

impl SerializableEnum for CameraCommand {
    fn to_string(&self) -> &'static str {
        match self {
            CameraCommand::MoveForward  => "CameraMoveForward",
            CameraCommand::MoveBackward => "CameraMoveBackward",
            CameraCommand::MoveUp       => "CameraMoveUp",
            CameraCommand::MoveDown     => "CameraMoveDown",
            CameraCommand::MoveLeft     => "CameraMoveLeft",
            CameraCommand::MoveRight    => "CameraMoveRight",
            CameraCommand::RotateLeft   => "CameraRotateLeft",
            CameraCommand::RotateRight  => "CameraRotateRight",
            CameraCommand::ZoomIn       => "CameraZoomIn",
            CameraCommand::ZoomOut      => "CameraZoomOut",
        }
    }

    fn from_string(string: &str) -> Option<Self>
    where
        Self: Sized
    {
        match string {
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

impl Command for CameraCommand {

}

pub struct CameraSystem {
    speed: f32,
    forward_move : i8,
    up_move : i8,
    right_move : i8,
}

impl CommandHandler<CameraCommand> for CameraSystem {
    fn handle(&mut self, commands: &Vec<CameraCommand>) {
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
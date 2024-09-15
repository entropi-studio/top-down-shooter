use crate::level::LevelObject;
use bevy::prelude::Vec2;

pub(in super::super) fn serialize_wall(args: Vec<String>) -> Result<LevelObject, String> {
    if args.len() != 5 {
        return Err(format!(
            "[#LevelObject::deserialize] Wall: expected 5 arguments but got {}",
            args.len()
        ));
    }
    let [size_x, size_y, position_x, position_y, rotation] = &args[..] else {
        panic!("[#LevelObject::deserialize] Wall: unexpected error at destructuring args")
    };
    let Ok(size_x) = size_x.parse::<f32>() else {
        return Err("[#LevelObject::deserialize] Wall: failed to parse size.x (0)".to_string());
    };
    let Ok(size_y) = size_y.parse::<f32>() else {
        return Err("[#LevelObject::deserialize] Wall: failed to parse size.x (1)".to_string());
    };
    let Ok(position_x) = position_x.parse::<f32>() else {
        return Err("[#LevelObject::deserialize] Wall: failed to parse size.x (2)".to_string());
    };
    let Ok(position_y) = position_y.parse::<f32>() else {
        return Err("[#LevelObject::deserialize] Wall: failed to parse size.x (3)".to_string());
    };
    let Ok(rotation) = rotation.parse::<f32>() else {
        return Err("[#LevelObject::deserialize] Wall: failed to parse rotation (4)".to_string());
    };

    Ok(LevelObject::Wall {
        size: Vec2::new(size_x, size_y),
        position: Vec2::new(position_x, position_y),
        rotation,
    })
}

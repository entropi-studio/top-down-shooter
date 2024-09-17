use crate::level::LevelObject;
use bevy::prelude::Vec2;

pub(in super::super) fn deserialize_lamp(args: Vec<String>) -> Result<LevelObject, String> {
    if args.len() != 3 {
        return Err(format!(
            "[#LevelObject::deserialize] Lamp: expected 3 arguments but got {}",
            args.len()
        ));
    }
    let [radius, intensity, falloff] = &args[..] else {
        panic!("[#LevelObject::deserialize] Lamp: unexpected error at destructuring args")
    };
    let Ok(radius) = radius.parse::<f32>() else {
        return Err("[#LevelObject::deserialize] Lamp: failed to parse radius (0)".to_string());
    };
    let Ok(intensity) = intensity.parse::<f32>() else {
        return Err("[#LevelObject::deserialize] Lamp: failed to parse intensity (1)".to_string());
    };
    let Ok(falloff) = falloff.parse::<f32>() else {
        return Err("[#LevelObject::deserialize] Lamp: failed to parse falloff (2)".to_string());
    };

    Ok(LevelObject::Lamp {
        radius,
        intensity,
        falloff,
    })
}

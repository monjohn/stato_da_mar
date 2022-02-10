use bevy::prelude::*;
use bevy::sprite::Rect;
use std::collections::HashMap;

// type SpriteHash = HashMap<&'static str, usize>;
pub fn build_ship_atlas(asset_server: Res<AssetServer>) -> TextureAtlas {
    let texture_handle = asset_server.load("ships2x.png");
    TextureAtlas::new_empty(texture_handle, Vec2::new(1024.0, 512.0))
}

pub fn load_ship_atlas(ship_atlas: &mut TextureAtlas) -> HashMap<&'static str, usize> {
    let mut sprite_hash: HashMap<&'static str, usize> = HashMap::new();
    for (name, min, max) in sprite_info().iter() {
        let index = ship_atlas.add_texture(Rect {
            min: Vec2::from(*min),
            max: Vec2::from(*max),
        });
        sprite_hash.insert(name, index);
    }
    sprite_hash
}

fn sprite_info() -> Vec<(&'static str, [f32; 2], [f32; 2])> {
    let sprites = vec![
        ("cannonball", [120.0, 29.0], [130.0, 39.0]),
        ("ship-plain", [408.0, 0.0], [474.0, 113.0]),
        ("ship-green-light-damage", [340.0, 345.0], [406.0, 458.0]),
        ("ship-blue-light-damage", [340.0, 230.0], [406.0, 343.0]),
        ("ship-yellow-light-damage", [340.0, 115.0], [406.0, 228.0]),
        ("ship-plain-heavy-damage", [340.0, 0.0], [406.0, 113.0]),
        ("ship-pirate-heavy-damage", [272.0, 345.0], [338.0, 458.0]),
        ("ship-red-heavy-damage", [272.0, 230.0], [338.0, 343.0]),
        ("ship-green-heavy-damage", [272.0, 115.0], [338.0, 228.0]),
        ("ship-blue-heavy-damage", [272.0, 0.0], [338.0, 113.0]),
        ("ship-yellow-heavy-damage", [204.0, 345.0], [270.0, 458.0]),
        ("ship-plain-destroyed", [204.0, 230.0], [270.0, 343.0]),
        ("ship-pirate", [408.0, 115.0], [474.0, 228.0]),
        ("ship-pirate-destroyed", [204.0, 0.0], [270.0, 113.0]),
        ("ship-red-destroyed", [136.0, 345.0], [202.0, 458.0]),
        ("ship-green-destoyred", [136.0, 230.0], [202.0, 343.0]),
        ("ship-blue-destoyed", [136.0, 115.0], [202.0, 228.0]),
        ("ship-yellow-destroed", [136.0, 0.0], [202.0, 113.0]),
        ("ship-red", [204.0, 115.0], [270.0, 228.0]),
        ("ship-green", [68.0, 192.0], [134.0, 305.0]),
        ("ship-blue", [68.0, 77.0], [134.0, 190.0]),
        ("ship-yellow", [68.0, 307.0], [134.0, 420.0]),
        ("ship-plain-light-damage", [0.0, 192.0], [66.0, 305.0]),
        ("ship-pirate-light-damage", [0.0, 307.0], [66.0, 420.0]),
        ("ship-red-light-damage", [0.0, 77.0], [66.0, 190.0]),
        ("wood (1).png", [88.0, 449.0], [103.0, 456.0]),
        ("wood (2).png", [408.0, 472.0], [434.0, 482.0]),
        ("wood (3).png", [116.0, 440.0], [131.0, 450.0]),
        ("wood (4).png", [88.0, 440.0], [114.0, 447.0]),
        ("cannon", [88.0, 422.0], [117.0, 438.0]),
        ("cannon-loose", [439.0, 496.0], [459.0, 508.0]),
        ("cannon-mobile", [408.0, 489.0], [437.0, 509.0]),
        ("crew (1).png", [511.0, 489.0], [533.0, 509.0]),
        ("crew (2).png", [463.0, 489.0], [485.0, 509.0]),
        ("crew (3).png", [487.0, 489.0], [509.0, 509.0]),
        ("crew (4).png", [568.0, 469.0], [590.0, 491.0]),
        ("crew (5).png", [439.0, 472.0], [461.0, 494.0]),
        ("crew (6).png", [544.0, 469.0], [566.0, 491.0]),
        ("dinghyLarge1.png", [606.0, 145.0], [626.0, 183.0]),
        ("dinghyLarge2.png", [610.0, 426.0], [630.0, 464.0]),
        ("dinghyLarge3.png", [588.0, 426.0], [608.0, 464.0]),
        ("dinghySmall1.png", [628.0, 166.0], [644.0, 192.0]),
        ("dinghySmall2.png", [612.0, 110.0], [628.0, 136.0]),
        ("dinghySmall3.png", [628.0, 138.0], [644.0, 164.0]),
        ("explosion1.png", [0.0, 0.0], [74.0, 75.0]),
        ("explosion2.png", [544.0, 145.0], [604.0, 204.0]),
        ("explosion3.png", [544.0, 426.0], [586.0, 467.0]),
        ("fire1.png", [614.0, 466.0], [632.0, 505.0]),
        ("fire2.png", [120.0, 0.0], [131.0, 27.0]),
        ("flag (1).png", [120.0, 41.0], [126.0, 63.0]),
        ("flag (2).png", [600.0, 486.0], [606.0, 508.0]),
        ("flag (3).png", [630.0, 110.0], [636.0, 132.0]),
        ("flag (4).png", [128.0, 41.0], [134.0, 63.0]),
        ("flag (5).png", [592.0, 486.0], [598.0, 508.0]),
        ("flag (6).png", [632.0, 426.0], [638.0, 448.0]),
        ("hullLarge (1).png", [596.0, 316.0], [646.0, 424.0]),
        ("hullLarge (2).png", [544.0, 206.0], [594.0, 314.0]),
        ("hullLarge (3).png", [596.0, 206.0], [646.0, 314.0]),
        ("hullLarge (4).png", [544.0, 316.0], [594.0, 424.0]),
        ("hullSmall (1).png", [612.0, 0.0], [652.0, 108.0]),
        ("hullSmall (2).png", [648.0, 330.0], [688.0, 438.0]),
        ("hullSmall (3).png", [648.0, 110.0], [688.0, 218.0]),
        ("hullSmall (4).png", [648.0, 220.0], [688.0, 328.0]),
        ("nest.png", [592.0, 466.0], [612.0, 484.0]),
        ("pole.png", [119.0, 422.0], [131.0, 433.0]),
        ("sailLarge (1).png", [408.0, 279.0], [474.0, 326.0]),
        ("sailLarge (10).png", [408.0, 328.0], [474.0, 374.0]),
        ("sailLarge (11).png", [408.0, 376.0], [474.0, 422.0]),
        ("sailLarge (12).png", [408.0, 424.0], [474.0, 470.0]),
        ("sailLarge (13).png", [476.0, 0.0], [542.0, 46.0]),
        ("sailLarge (14).png", [476.0, 48.0], [542.0, 95.0]),
        ("sailLarge (15).png", [136.0, 460.0], [202.0, 507.0]),
        ("sailLarge (16).png", [68.0, 460.0], [134.0, 507.0]),
        ("sailLarge (17).png", [476.0, 97.0], [542.0, 144.0]),
        ("sailLarge (18).png", [476.0, 146.0], [542.0, 193.0]),
        ("sailLarge (19).png", [476.0, 293.0], [542.0, 340.0]),
        ("sailLarge (2).png", [272.0, 460.0], [338.0, 507.0]),
        ("sailLarge (20).png", [204.0, 460.0], [270.0, 507.0]),
        ("sailLarge (21).png", [340.0, 460.0], [406.0, 507.0]),
        ("sailLarge (22).png", [476.0, 244.0], [542.0, 291.0]),
        ("sailLarge (23).png", [408.0, 230.0], [474.0, 277.0]),
        ("sailLarge (24).png", [476.0, 342.0], [542.0, 389.0]),
        ("sailLarge (3).png", [476.0, 391.0], [542.0, 438.0]),
        ("sailLarge (4).png", [476.0, 440.0], [542.0, 487.0]),
        ("sailLarge (5).png", [544.0, 0.0], [610.0, 47.0]),
        ("sailLarge (6).png", [0.0, 460.0], [66.0, 507.0]),
        ("sailLarge (7).png", [476.0, 195.0], [542.0, 242.0]),
        ("sailLarge (8).png", [544.0, 97.0], [610.0, 143.0]),
        ("sailLarge (9).png", [544.0, 49.0], [610.0, 95.0]),
        ("sailSmall (1).png", [0.0, 422.0], [42.0, 431.0]),
        ("sailSmall (10).png", [44.0, 422.0], [86.0, 431.0]),
        ("sailSmall (11).png", [0.0, 433.0], [42.0, 442.0]),
        ("sailSmall (12).png", [44.0, 433.0], [86.0, 442.0]),
        ("sailSmall (13).png", [44.0, 444.0], [86.0, 453.0]),
        ("sailSmall (2).png", [0.0, 444.0], [42.0, 453.0]),
        ("sailSmall (3).png", [76.0, 0.0], [118.0, 9.0]),
        ("sailSmall (4).png", [76.0, 11.0], [118.0, 19.0]),
        ("sailSmall (5).png", [76.0, 21.0], [118.0, 29.0]),
        ("sailSmall (6).png", [76.0, 31.0], [118.0, 39.0]),
        ("sailSmall (7).png", [76.0, 41.0], [118.0, 49.0]),
        ("sailSmall (8).png", [76.0, 62.0], [118.0, 71.0]),
        ("sailSmall (9).png", [76.0, 51.0], [118.0, 60.0]),
    ];

    sprites
}

use bevy::{prelude::*, reflect::{TypePath, TypeUuid}};
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, TypeUuid, TypePath, Serialize, Deserialize)]
#[uuid = "b95ebd8a-8273-11ee-b962-0242ac120002"]
pub struct LevelCollisionData {
  pub hulls: Vec<LevelCollisionHullData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelCollisionHullData {
  pub pos: (f32, f32),
  pub size: (f32, f32),
}

pub fn collision_data_from_image(image: &Image) -> LevelCollisionData {
    let mut result = Vec::new();
    let width = image.texture_descriptor.size.width as usize;
    let height = image.texture_descriptor.size.height as usize;
    // iterate over image and find all hulls
    let mut hull = extract_hulls(image);

    fn get(hull: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
        if let Some(row) = hull.get(y) {
            if let Some(cell) = row.get(x) {
                return *cell;
            }
        }
        false
    }
    
    for x in 0..width {
        for y in 0..height {
            if get(&hull, x, y) {
                hull[y][x] = false;
                if get(&hull, x + 1, y) {
                    // search right
                    let mut end = x + 1;
                    while get(&hull, end, y) {
                        hull[y][end] = false;
                        end += 1;
                    }
                    result.push(LevelCollisionHullData {
                        pos: (x as f32, -(y as f32)),
                        size: ((end - x) as f32, 1.0),
                    });
                } else {
                    // search down
                    let mut end = y + 1;
                    while get(&hull, x, end) {
                        hull[end][x] = false;
                        end += 1;
                    }
                    result.push(LevelCollisionHullData {
                        pos: (x as f32, -(y as f32)),
                        size: (1.0, (end - y) as f32),
                    });
                }
                /*result.push(LevelCollisionHullData {
                    pos: (x as f32, -(y as f32)),
                    size: (1.0, 1.0),
                });*/
            }
        }
    }

    LevelCollisionData {
        hulls: result,
    }
}

fn extract_hulls(image: &Image) -> Vec<Vec<bool>> {
    let width = image.texture_descriptor.size.width as i32;
    let height = image.texture_descriptor.size.height as i32;
    let mut hulls: Vec<Vec<bool>> = Vec::new();

    fn is_hull_pixel(image: &Image, x: i32, y: i32) -> bool {
        if x < 0 || x >= image.texture_descriptor.size.width as i32 {
            return false;
        }
        if y < 0 || y >= image.texture_descriptor.size.height as i32 {
            return false;
        }
        let a: u8 = image.data[3 + 4 * (x as usize + y as usize * image.texture_descriptor.size.width as usize)];
        a > 0
    }

    for y in 0..height {
        let mut row: Vec<bool> = Vec::new();
        for x in 0..width {
            let is_hull =
                if is_hull_pixel(image, x, y) {
                    let mut neighbors = 0;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dx != 0 && dy != 0 || dx == 0 && dy == 0 {
                                continue;
                            }

                            if is_hull_pixel(image, x + dx, y + dy) {
                                neighbors += 1;
                            }
                        }
                    }
                    neighbors < 4
                } else {
                    false
                };
            row.push(is_hull);
        };
        hulls.push(row);
    }
    hulls
}
use std::io;
use std::fs;
use crate::vector::Vec3f;

pub struct Model {
    pub vertices: Vec<Vec3f>,
    pub texture_coords: Vec<Vec3f>,
    pub normals: Vec<Vec3f>,
    pub faces: Vec<Vec<ModelVertex>>
}

pub struct ModelVertex {
    pub vertex_index:i32,
    pub texture_index:i32,
    pub normal_index:i32
}

impl Model {
    pub fn new() -> Model {
        Model {
            vertices: Vec::<Vec3f>::new(),
            texture_coords: Vec::<Vec3f>::new(),
            normals: Vec::<Vec3f>::new(),
            faces: Vec::<Vec::<ModelVertex>>::new()
        }
    }

    pub fn parse(filename: &String) -> Result<Model, io::Error> {
        let content = match fs::read_to_string(filename) {
            Ok(content) => content,
            Err(e) => return Err(e)
        };

        let mut model = Model::new();
        
        for line in content.lines() {
            if line.starts_with("v ") {
                let vertex_data:Vec<&str> = line.split_ascii_whitespace().collect();
                model.vertices.push(Vec3f{
                    x:vertex_data[1].parse().unwrap(), 
                    y:vertex_data[2].parse().unwrap(), 
                    z:vertex_data[3].parse().unwrap()
                });
            } else if line.starts_with("vt ") {
                let vertex_data:Vec<&str> = line.split_ascii_whitespace().collect();
                model.texture_coords.push(Vec3f{
                    x:vertex_data[1].parse().unwrap(), 
                    y:vertex_data[2].parse().unwrap(), 
                    z:vertex_data[3].parse().unwrap()
                });
            } else if line.starts_with("vn ") {
                let vertex_data:Vec<&str> = line.split_ascii_whitespace().collect();
                model.normals.push(Vec3f{
                    x:vertex_data[1].parse().unwrap(), 
                    y:vertex_data[2].parse().unwrap(), 
                    z:vertex_data[3].parse().unwrap()
                });
            } else if line.starts_with("f ") {
                let face_data:Vec<&str> = line.split(" ").collect();

                let vertex0_data = face_data[1].split("/").collect::<Vec<&str>>();
                let vertex1_data = face_data[2].split("/").collect::<Vec<&str>>();
                let vertex2_data = face_data[3].split("/").collect::<Vec<&str>>();

                let vertex0 = vertex0_data[0].parse::<i32>().unwrap() - 1;
                let vertex1 = vertex1_data[0].parse::<i32>().unwrap() - 1;
                let vertex2 = vertex2_data[0].parse::<i32>().unwrap() - 1;

                let vertex0_texture = vertex0_data[1].parse::<i32>().unwrap() - 1;
                let vertex1_texture = vertex1_data[1].parse::<i32>().unwrap() - 1;
                let vertex2_texture = vertex2_data[1].parse::<i32>().unwrap() - 1;

                let vertex0_normal = vertex0_data[2].parse::<i32>().unwrap() - 1;
                let vertex1_normal = vertex1_data[2].parse::<i32>().unwrap() - 1;
                let vertex2_normal = vertex2_data[2].parse::<i32>().unwrap() - 1;


                model.faces.push(vec![
                    ModelVertex{vertex_index:vertex0, texture_index:vertex0_texture, normal_index:vertex0_normal },
                    ModelVertex{vertex_index:vertex1, texture_index:vertex1_texture, normal_index:vertex1_normal },
                    ModelVertex{vertex_index:vertex2, texture_index:vertex2_texture, normal_index:vertex2_normal }
                ]);
            }
        }

        Ok(model)
    }
}
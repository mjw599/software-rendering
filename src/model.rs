use std::io;
use std::fs;
use crate::vector::Vec3f;

pub struct Model {
    pub vertices: Vec<Vec3f>,
    pub faces: Vec<Vec<i32>>
}

impl Model {
    pub fn new() -> Model {
        Model {
            vertices: Vec::<Vec3f>::new(),
            faces: Vec::<Vec::<i32>>::new()
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
                let vertex_data:Vec<&str> = line.split(" ").collect();
                model.vertices.push(Vec3f{
                    x:vertex_data[1].parse().unwrap(), 
                    y:vertex_data[2].parse().unwrap(), 
                    z:vertex_data[3].parse().unwrap()
                });
            } else if line.starts_with("f ") {
                let face_data:Vec<&str> = line.split(" ").collect();

                let vertex0 = face_data[1].split("/").collect::<Vec<&str>>()[0].parse::<i32>().unwrap() - 1;
                let vertex1 = face_data[2].split("/").collect::<Vec<&str>>()[0].parse::<i32>().unwrap() - 1;
                let vertex2 = face_data[3].split("/").collect::<Vec<&str>>()[0].parse::<i32>().unwrap() - 1;

                model.faces.push(vec![
                    vertex0,
                    vertex1,
                    vertex2
                ]);
            }
        }

        Ok(model)
    }
}
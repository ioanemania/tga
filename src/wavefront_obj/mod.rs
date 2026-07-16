use std::io::{BufRead, BufReader, Read};

use nalgebra::Vector4;

type Vertex = Vector4<f32>;

pub struct Face {
    pub vertex_indices: Vec<usize>,
    // texture_indices: Vec<usize>,
    // normal_indices: Vec<usize>,
}

#[derive(Default)]
pub struct Model {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
}

fn next_required<I, T, E>(mut iter: I, name: &str) -> std::io::Result<T>
where
    I: Iterator<Item = Result<T, E>>,
    E: std::error::Error + Send + Sync + 'static,
{
    iter.next()
        .ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, format!("missing {}", name))
        })?
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

impl Model {
    pub fn from_reader<R: Read>(read: &mut R) -> std::io::Result<Self> {
        let mut model = Model::default();
        let reader = BufReader::new(read);

        for line in reader.lines() {
            let line = line?;

            let (action, other) = line.split_once(" ").or_else(|| Some((&line, ""))).unwrap();

            match action {
                "#" => continue,
                "v" => {
                    let mut parts = other.split_whitespace().map(|s| s.parse::<f32>());
                    let x = next_required(&mut parts, "vertex x coordinate")?;
                    let y = next_required(&mut parts, "vertex y coordinate")?;
                    let z = next_required(&mut parts, "vertex z coordinate")?;
                    let w = parts
                        .next()
                        .map(|r| {
                            r.map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
                        })
                        .transpose()?
                        .unwrap_or(1.0);
                    model.vertices.push(Vertex::new(x, y, z, w));
                }
                "f" => {
                    let mut face = Face {
                        vertex_indices: Vec::new(),
                    };

                    for element in other.split_whitespace() {
                        let parts: Vec<&str> = element.split("/").collect();

                        if parts.len() != 3 {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Expected three parts of a face element",
                            ));
                        }

                        let vertex = match parts[0].parse::<usize>() {
                            Ok(v) => v,
                            Err(_) => {
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::InvalidData,
                                    "Missing vertex index",
                                ));
                            }
                        };

                        face.vertex_indices.push(vertex);
                    }

                    model.faces.push(face);
                }
                _ => continue,
            };
        }

        Ok(model)
    }
}

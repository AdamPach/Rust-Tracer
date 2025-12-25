use crate::core::geometry::coordinates::{X, Y, Z};
use crate::core::geometry::point::Point;
use crate::raytracing::material::ambient::AmbientMaterialBuilder;
use crate::raytracing::material::color::{A, B, G, MaterialColor, R};
use crate::raytracing::{SceneDescriptor, Triangle, TriangulatedMeshBuilder};
use anyhow::Context;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
struct ObjData {
    vertices: Vec<Point>,
    normals: Vec<[f64; 3]>,
    tex_coords: Vec<[f64; 2]>,
    geometry: Vec<TriangulatedMeshBuilder>,
}

pub fn load_obj<T: AsRef<Path>>(path: T) -> anyhow::Result<SceneDescriptor> {
    let file = File::open(path).with_context(|| "Failed to open OBJ file!")?;

    let mut scene = SceneDescriptor::new();

    let green = scene.add_material(AmbientMaterialBuilder::new(MaterialColor::new(
        R::new(0.05),
        G::new(0.95),
        B::new(0.05),
        A::new(1.0),
    )));

    let mut obj_data = ObjData {
        vertices: Vec::new(),
        normals: Vec::new(),
        tex_coords: Vec::new(),
        geometry: vec![TriangulatedMeshBuilder::new(green)],
    };

    let lines = BufReader::new(file).lines();

    for line in lines {
        let line = line?;

        let (prefix, data) = line.split_once(' ').unwrap();

        obj_data = match prefix {
            "v" => parse_vertex(data, obj_data)?,
            "vn" => parse_normal(data, obj_data)?,
            "vt" => parse_texture_coordinates(data, obj_data)?,
            "f" => parse_faces(data, obj_data)?,
            _ => continue,
        }
    }

    println!("Loaded {:?} vertices", obj_data);

    for mesh_builder in obj_data.geometry {
        scene.add_object(mesh_builder);
    }

    Ok(scene)
}

fn parse_vertex(vertices: &str, mut data: ObjData) -> anyhow::Result<ObjData> {
    let mut array = [0.0; 3];

    let vertices = vertices.split_whitespace().collect::<Vec<&str>>();

    if vertices.len() != 3 {
        return Err(anyhow::anyhow!(
            "Only 3D vertices are supported! Found vertex with {} dimensions.",
            vertices.len()
        ));
    }

    for (i, coord_str) in vertices.into_iter().enumerate() {
        array[i] = coord_str
            .parse::<f64>()
            .with_context(|| "Failed to parse a float array")?;
    }

    data.vertices.push(Point::new(
        X::new(array[0]),
        Y::new(array[1]),
        Z::new(array[2]),
    ));

    Ok(data)
}

fn parse_normal(normals: &str, mut data: ObjData) -> anyhow::Result<ObjData> {
    let mut array = [0.0; 3];

    let normals = normals.split_whitespace().collect::<Vec<&str>>();

    if normals.len() != 3 {
        return Err(anyhow::anyhow!(
            "Only 3D normals are supported! Found normal with {} dimensions.",
            normals.len()
        ));
    }

    for (i, coord_str) in normals.into_iter().enumerate() {
        array[i] = coord_str
            .parse::<f64>()
            .with_context(|| "Failed to parse a float array")?;
    }

    data.normals.push(array);

    Ok(data)
}

fn parse_texture_coordinates(texture: &str, mut data: ObjData) -> anyhow::Result<ObjData> {
    let mut array = [0.0; 2];

    let texture = texture.split_whitespace().collect::<Vec<&str>>();

    if texture.len() != 2 {
        return Err(anyhow::anyhow!(
            "Only 2D texture coordinates are supported! Found texture with {} dimensions.",
            texture.len()
        ));
    }

    for (i, coord_str) in texture.into_iter().enumerate() {
        array[i] = coord_str
            .parse::<f64>()
            .with_context(|| "Failed to parse a float array")?;
    }

    data.tex_coords.push(array);

    Ok(data)
}

fn parse_faces(faces: &str, mut data: ObjData) -> anyhow::Result<ObjData> {
    let mut points = [Point::default(); 3];

    let faces = faces.split_whitespace().collect::<Vec<&str>>();

    if faces.len() != 3 {
        return Err(anyhow::anyhow!(
            "Only triangular faces are supported! Found face with {} vertices.",
            faces.len()
        ));
    }

    for (index, face) in faces.into_iter().enumerate() {
        let indices: Vec<&str> = face.split('/').collect();

        let vertex_index: usize = indices[0]
            .parse::<usize>()
            .with_context(|| "Failed to parse vertex index!")?;

        points[index] = match data.vertices.get(vertex_index - 1) {
            Some(vertex) => vertex.clone(),
            None => {
                return Err(anyhow::anyhow!(
                    "Vertex with index {} does not exists!",
                    vertex_index
                ));
            }
        };
    }

    let mesh_builder = data.geometry.pop().unwrap();

    data.geometry
        .push(mesh_builder.add_triangle(Triangle::new(points)));

    Ok(data)
}
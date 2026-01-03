use crate::core::geometry::coordinates::{X, Y, Z};
use crate::core::geometry::point::Point;
use crate::core::geometry::vector::Vector3;
use crate::io::wavefront::mtl::load_mtl;
use crate::io::wavefront::parse_array::parse_array;
use crate::raytracing::material::{MaterialTypeId, MaterialsRegistry};
use crate::raytracing::{GeometryRegistry, SceneDescriptor, Triangle, TriangulatedMeshBuilder};
use anyhow::Context;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

struct ObjData {
    vertices: Vec<Point>,
    normals: Vec<Vector3>,
    tex_coords: Vec<[f64; 2]>,
    geometry: Vec<TriangulatedMeshBuilder>,
    materials: HashMap<String, MaterialTypeId>,
    current_material: Option<MaterialTypeId>,
    materials_registry: MaterialsRegistry,
}

pub fn load_obj<P: AsRef<Path>>(path: P) -> anyhow::Result<SceneDescriptor> {
    let file = File::open(path.as_ref())
        .with_context(|| format!("Failed to open mtl file: {}", path.as_ref().display()))?;

    let mut obj_data = ObjData {
        vertices: Vec::new(),
        normals: Vec::new(),
        tex_coords: Vec::new(),
        geometry: Vec::new(),
        materials: HashMap::new(),
        current_material: None,
        materials_registry: MaterialsRegistry::new(),
    };

    let lines = BufReader::new(file).lines();

    for line in lines {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        let (prefix, data) = line.split_once(' ').unwrap();

        obj_data = match prefix {
            "v" => parse_vertex(data, obj_data)?,
            "vn" => parse_normal(data, obj_data)?,
            "vt" => parse_texture_coordinates(data, obj_data)?,
            "f" => parse_faces(data, obj_data)?,
            "o" | "g" => add_new_mesh(obj_data),
            "mtllib" => load_mtllib_file(data, path.as_ref().parent().unwrap(), obj_data)?,
            "usemtl" => use_mtl_material(data, obj_data)?,
            _ => continue,
        }
    }

    let mut geometry_registry = GeometryRegistry::new();

    for mesh_builder in obj_data.geometry {
        geometry_registry.add(mesh_builder);
    }

    Ok(SceneDescriptor::new(
        geometry_registry,
        obj_data.materials_registry,
    ))
}

fn parse_vertex(vertices: &str, mut data: ObjData) -> anyhow::Result<ObjData> {
    let array = parse_array::<f64, 3, fn() -> anyhow::Error>(
        vertices,
        || {
            anyhow::anyhow!(
                "Only 3D vertices are supported! Found vertex with less than 3 dimensions."
            )
        },
        || anyhow::anyhow!("Failed to parse vertex in obj file: invalid float value"),
    )?;

    data.vertices.push(Point::new(
        X::new(array[0]),
        Y::new(array[1]),
        Z::new(array[2]),
    ));

    Ok(data)
}

fn parse_normal(normals: &str, mut data: ObjData) -> anyhow::Result<ObjData> {
    let array = parse_array::<f64, 3, fn() -> anyhow::Error>(
        normals,
        || {
            anyhow::anyhow!(
                "Only 3D normals are supported! Found normal with less than 3 dimensions."
            )
        },
        || anyhow::anyhow!("Failed to parse normal in obj file: invalid float value"),
    )?;

    data.normals.push(Vector3::new(
        X::new(array[0]),
        Y::new(array[1]),
        Z::new(array[2]),
    ));

    Ok(data)
}

fn parse_texture_coordinates(texture: &str, mut data: ObjData) -> anyhow::Result<ObjData> {
    let array = parse_array::<f64, 2, fn() -> anyhow::Error>(
        texture,
        || {
            anyhow::anyhow!(
                "Only 2D texture coordinates are supported! Found texture coordinate with less than 2 dimensions."
            )
        },
        || anyhow::anyhow!("Failed to parse texture coordinate in obj file: invalid float value"),
    )?;

    data.tex_coords.push(array);

    Ok(data)
}

fn parse_faces(faces: &str, mut data: ObjData) -> anyhow::Result<ObjData> {
    let mut points = [Point::default(); 3];
    let mut normals = [Vector3::default(); 3];

    let mesh_builder = data.geometry.pop().ok_or_else(|| {
        anyhow::anyhow!("No mesh defined before face definition! Define a new object or group before defining faces.")
    })?;

    let mut faces = faces.split_whitespace();

    for i in 0..3 {
        let face = faces.next().ok_or_else(|| {
            anyhow::anyhow!(
                "Only triangular faces are supported! Found face with less than 3 vertices."
            )
        })?;

        let indices: Vec<&str> = face.split('/').collect();

        let vertex_index: usize = indices[0]
            .parse::<usize>()
            .with_context(|| "Failed to parse vertex index!")?;

        points[i] = match data.vertices.get(vertex_index - 1) {
            Some(vertex) => vertex.clone(),
            None => {
                return Err(anyhow::anyhow!(
                    "Vertex with index {} does not exists!",
                    vertex_index
                ));
            }
        };

        let normal_index: usize = indices[2]
            .parse::<usize>()
            .with_context(|| "Failed to parse normal index!")?;

        normals[i] = match data.normals.get(normal_index - 1) {
            Some(normal) => normal.clone(),
            None => {
                return Err(anyhow::anyhow!(
                    "Normal with index {} does not exists!",
                    normal_index
                ));
            }
        }
    }

    let Some(material_id) = data.current_material else {
        return Err(anyhow::anyhow!(
            "No material defined before face definition!"
        ));
    };

    data.geometry
        .push(mesh_builder.add_triangle(Triangle::new(points, normals, material_id)));

    Ok(data)
}

fn add_new_mesh(mut data: ObjData) -> ObjData {
    data.geometry.push(TriangulatedMeshBuilder::new());

    data
}

fn load_mtllib_file<P: AsRef<Path>>(
    mtllib_file: &str,
    folder_path: P,
    mut data: ObjData,
) -> anyhow::Result<ObjData> {
    let mtllib_path = folder_path.as_ref().join(mtllib_file);

    let new_materials = load_mtl(mtllib_path)?;

    for (material_name, material) in new_materials {
        let material_id = data.materials_registry.add(material);

        data.materials.insert(material_name, material_id);
    }

    Ok(data)
}

fn use_mtl_material(material_name: &str, mut data: ObjData) -> anyhow::Result<ObjData> {
    let material_id = data
        .materials
        .get(material_name)
        .ok_or_else(|| anyhow::anyhow!("Material with name {} not found!", material_name))?;

    data.current_material = Some(*material_id);

    Ok(data)
}

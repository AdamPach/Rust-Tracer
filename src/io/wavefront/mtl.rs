use crate::raytracing::material::MaterialType;
use crate::raytracing::material::ambient::AmbientMaterialBuilder;
use crate::raytracing::material::color::{A, B, G, MaterialColor, R};
use anyhow::Context;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn load_mtl<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<(String, MaterialType)>> {
    let file = File::open(path.as_ref())
        .with_context(|| format!("Failed to open mtllib file: {}", path.as_ref().display()))?;

    let lines = BufReader::new(file).lines();
    let mut materials = Vec::new();

    for line in lines {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        let (prefix, data) = line.split_once(' ').unwrap();

        materials = match prefix {
            "newmtl" => parse_newmtl(data, materials),
            "Ka" => parse_ambient(data, materials)?,
            "illum" => parse_illum(data, materials)?,
            _ => continue,
        }
    }

    let mut result = Vec::with_capacity(materials.len());

    for mtl in materials {
        let material_name = mtl.name.clone();
        let material = convert_mtl_to_material(mtl)?;
        result.push((material_name, material));
    }

    Ok(result)
}

fn parse_newmtl(material_name: &str, mut materials: Vec<MtlMaterial>) -> Vec<MtlMaterial> {
    materials.push(MtlMaterial {
        name: material_name.to_string(),
        ambient: None,
        material: None,
    });

    materials
}

fn parse_ambient(
    coefficients: &str,
    mut materials: Vec<MtlMaterial>,
) -> anyhow::Result<Vec<MtlMaterial>> {
    let mut array = [0.0; 3];

    let mut coefficients = coefficients.split_whitespace();

    for i in 0..3 {
        let coefficient = match coefficients.next() {
            Some(coord) => coord,
            None => {
                return Err(anyhow::anyhow!(
                    "Failed to parse ambient coefficient: expected 3 values, found less"
                ));
            }
        };

        array[i] = coefficient
            .parse::<f32>()
            .with_context(|| "Failed to parse ambient coefficient: Not a number")?;
    }

    let Some(last_material) = materials.last_mut() else {
        return Err(anyhow::anyhow!(
            "Ambient coefficient defined before any material"
        ));
    };

    last_material.ambient = Some(array);

    Ok(materials)
}

fn parse_illum(illum: &str, mut materials: Vec<MtlMaterial>) -> anyhow::Result<Vec<MtlMaterial>> {
    let illum_value = illum
        .parse::<u32>()
        .with_context(|| "Failed to parse illum value: Not a number")?;

    let Some(last_material) = materials.last_mut() else {
        return Err(anyhow::anyhow!("Illum value defined before any material"));
    };

    last_material.material = match illum_value {
        1 => Some(Material::Ambient),
        _ => {
            return Err(anyhow::anyhow!(
                "Illum value '{}' is not supported",
                illum_value
            ));
        }
    };

    Ok(materials)
}

fn convert_mtl_to_material(mtl: MtlMaterial) -> anyhow::Result<MaterialType> {
    match mtl.material {
        Some(Material::Ambient) => {
            let Some(ambient) = mtl.ambient else {
                return Err(anyhow::anyhow!(
                    "Material '{}' is missing ambient coefficient",
                    mtl.name
                ));
            };

            Ok(AmbientMaterialBuilder::new(MaterialColor::new(
                R::new(ambient[0]),
                G::new(ambient[1]),
                B::new(ambient[2]),
                A::new(1.0),
            ))
            .into())
        }
        None => Err(anyhow::anyhow!(
            "Material '{}' is missing illum definition",
            mtl.name
        )),
    }
}

struct MtlMaterial {
    name: String,
    ambient: Option<[f32; 3]>,
    material: Option<Material>,
}

enum Material {
    Ambient,
}

use crate::io::wavefront::parse_array::parse_array;
use crate::raytracing::material::MaterialType;
use crate::raytracing::material::ambient::AmbientMaterialBuilder;
use crate::raytracing::material::color::{A, B, G, MaterialColor, R};
use crate::raytracing::material::diffuse::DiffuseMaterialBuilder;
use crate::raytracing::material::emissive::EmissiveMaterialBuilder;
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
            "Kd" => parse_diffuse(data, materials)?,
            "Ke" => parse_emissive(data, materials)?,
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
        diffuse: None,
        emissive: None,
    });

    materials
}

fn parse_ambient(
    coefficients: &str,
    mut materials: Vec<MtlMaterial>,
) -> anyhow::Result<Vec<MtlMaterial>> {
    let array = parse_array::<f32, 3, fn() -> anyhow::Error>(
        coefficients,
        || anyhow::anyhow!("Failed to parse ambient coefficient: expected 3 values, found less"),
        || anyhow::anyhow!("Failed to parse ambient coefficient in mtllib: invalid float value"),
    )?;

    let Some(last_material) = materials.last_mut() else {
        return Err(anyhow::anyhow!(
            "Ambient coefficient defined before any material"
        ));
    };

    last_material.ambient = Some(array);

    Ok(materials)
}

fn parse_diffuse(
    coefficients: &str,
    mut materials: Vec<MtlMaterial>,
) -> anyhow::Result<Vec<MtlMaterial>> {
    let _array = parse_array::<f32, 3, fn() -> anyhow::Error>(
        coefficients,
        || anyhow::anyhow!("Failed to parse diffuse coefficient: expected 3 values, found less"),
        || anyhow::anyhow!("Failed to parse diffuse coefficient in mtllib: invalid float value"),
    )?;

    let Some(_last_material) = materials.last_mut() else {
        return Err(anyhow::anyhow!(
            "Diffuse coefficient defined before any material"
        ));
    };

    _last_material.diffuse = Some(_array);

    Ok(materials)
}

fn parse_emissive(
    coefficients: &str,
    mut materials: Vec<MtlMaterial>,
) -> anyhow::Result<Vec<MtlMaterial>> {
    let array = parse_array::<f32, 3, fn() -> anyhow::Error>(
        coefficients,
        || anyhow::anyhow!("Failed to parse emissive coefficient: expected 3 values, found less"),
        || anyhow::anyhow!("Failed to parse emissive coefficient in mtllib: invalid float value"),
    )?;

    let Some(last_material) = materials.last_mut() else {
        return Err(anyhow::anyhow!(
            "Emissive coefficient defined before any material"
        ));
    };

    last_material.emissive = Some(array);

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
        1 => Some(Material::Emissive),
        2 => Some(Material::Ambient),
        3 => Some(Material::Diffuse),
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
        Some(Material::Diffuse) => {
            let Some(diffuse) = mtl.diffuse else {
                return Err(anyhow::anyhow!(
                    "Material '{}' is missing diffuse coefficient",
                    mtl.name
                ));
            };

            Ok(DiffuseMaterialBuilder::new(MaterialColor::new(
                R::new(diffuse[0]),
                G::new(diffuse[1]),
                B::new(diffuse[2]),
                A::new(1.0),
            ))
            .into())
        }
        Some(Material::Emissive) => {
            let Some(emissive) = mtl.emissive else {
                return Err(anyhow::anyhow!(
                    "Material '{}' is missing emissive coefficient",
                    mtl.name
                ));
            };

            Ok(EmissiveMaterialBuilder::new(MaterialColor::new(
                R::new(emissive[0]),
                G::new(emissive[1]),
                B::new(emissive[2]),
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
    diffuse: Option<[f32; 3]>,
    emissive: Option<[f32; 3]>,
    material: Option<Material>,
}

enum Material {
    Ambient,
    Diffuse,
    Emissive,
}

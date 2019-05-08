use crate::{
    shape::{FromShape, ShapePrefab},
    types::{Mesh, MeshData},
};
use amethyst_assets::{
    AssetPrefab, AssetStorage, Format, Handle, Loader, PrefabData, ProgressCounter,
};
use amethyst_core::ecs::{Entity, Read, ReadExpect, WriteStorage};
use amethyst_error::Error;
use rendy::{hal::Backend, mesh::MeshBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ObjFormat;

amethyst_assets::register_format_type!(MeshData);

amethyst_assets::register_format!("OBJ", ObjFormat as MeshData);
impl Format<MeshData> for ObjFormat {
    fn name(&self) -> &'static str {
        "WAVEFRONT_OBJ"
    }

    fn import_simple(&self, bytes: Vec<u8>) -> Result<MeshData, Error> {
        rendy::mesh::obj::load_from_obj(&bytes)
            .map(|builder| builder.into())
            .map_err(|e| e.compat().into())
    }
}

/// Internal mesh loading
///
/// ### Type parameters:
///
/// `B`: `Backend` type parameter for `Mesh<B>`
/// `V`: Vertex format to use for generated `Mesh`es, must be one of:
///     * `Vec<PosTex>`
///     * `Vec<PosNormTex>`
///     * `Vec<PosNormTangTex>`
///     * `ComboMeshCreator`
/// `M`: `Format` to use for loading `Mesh`es from file
#[derive(Deserialize, Serialize)]
#[serde(bound = "")]
pub enum MeshPrefab<B, V>
where
    B: Backend,
{
    /// Load an asset Mesh from file
    Asset(AssetPrefab<Mesh<B>>),
    /// Generate a Mesh from basic type
    Shape(ShapePrefab<B, V>),
}

impl<'a, B, V> PrefabData<'a> for MeshPrefab<B, V>
where
    B: Backend,
    V: FromShape + Into<MeshBuilder<'static>>,
{
    type SystemData = (
        ReadExpect<'a, Loader>,
        WriteStorage<'a, Handle<Mesh<B>>>,
        Read<'a, AssetStorage<Mesh<B>>>,
    );
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        system_data: &mut Self::SystemData,
        entities: &[Entity],
        children: &[Entity],
    ) -> Result<(), Error> {
        match self {
            MeshPrefab::Asset(m) => {
                m.add_to_entity(entity, system_data, entities, children)?;
            }
            MeshPrefab::Shape(s) => {
                s.add_to_entity(entity, system_data, entities, children)?;
            }
        }
        Ok(())
    }

    fn load_sub_assets(
        &mut self,
        progress: &mut ProgressCounter,
        system_data: &mut Self::SystemData,
    ) -> Result<bool, Error> {
        Ok(match self {
            MeshPrefab::Asset(m) => m.load_sub_assets(progress, system_data)?,
            MeshPrefab::Shape(s) => s.load_sub_assets(progress, system_data)?,
        })
    }
}

//! Scene.

use crate::data::{GeometryMesh, Material, Mesh};

/// Scene.
#[derive(Default, Debug, Clone)]
pub struct Scene {
    /// Scene name.
    name: Option<String>,
    /// Geometry mesh.
    geometry_meshes: Vec<GeometryMesh>,
    /// Materials.
    materials: Vec<Material>,
    /// Meshes.
    meshes: Vec<Mesh>,
}

impl Scene {
    /// Creates a new `Scene`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the scene name.
    pub fn set_name(&mut self, name: impl Into<Option<String>>) {
        self.name = name.into();
    }

    /// Add a geometry mesh.
    pub(crate) fn add_geometry_mesh(&mut self, mesh: GeometryMesh) -> GeometryMeshIndex {
        let index = GeometryMeshIndex::new(self.meshes.len());
        self.geometry_meshes.push(mesh);
        index
    }

    /// Returns a reference to the geometry mesh.
    pub fn geometry_mesh(&self, i: GeometryMeshIndex) -> Option<&GeometryMesh> {
        self.geometry_meshes.get(i.to_usize())
    }

    /// Add a material.
    pub(crate) fn add_material(&mut self, material: Material) -> MaterialIndex {
        let index = MaterialIndex::new(self.materials.len());
        self.materials.push(material);
        index
    }

    /// Returns a reference to the material.
    pub fn material(&self, i: MaterialIndex) -> Option<&Material> {
        self.materials.get(i.to_usize())
    }

    /// Add a mesh.
    pub(crate) fn add_mesh(&mut self, mesh: Mesh) -> MeshIndex {
        let index = MeshIndex::new(self.meshes.len());
        self.meshes.push(mesh);
        index
    }

    /// Returns a reference to the mesh.
    pub fn mesh(&self, i: MeshIndex) -> Option<&Mesh> {
        self.meshes.get(i.to_usize())
    }
}

macro_rules! define_index_type {
    ($(
        $(#[$meta:meta])*
        $ty:ident;
    )*) => {
        $(
            define_index_type! {
                @single
                $(#[$meta])*
                $ty;
            }
        )*
    };
    (
        @single
        $(#[$meta:meta])*
        $ty:ident;
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $ty(u32);

        impl $ty {
            /// Creates a new index.
            ///
            /// # Panics
            ///
            /// Panics if the given index is larger than `std::u32::MAX`.
            pub(crate) fn new(i: usize) -> Self {
                assert!(i <= std::u32::MAX as usize);
                Self(i as u32)
            }

            /// Retuns `usize` value.
            #[allow(dead_code)]
            fn to_usize(self) -> usize {
                self.0 as usize
            }
        }
    };
}

define_index_type! {
    /// Geometry mesh index.
    GeometryMeshIndex;
    /// Material index.
    MaterialIndex;
    /// Mesh index.
    MeshIndex;
}

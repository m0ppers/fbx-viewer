//! Scene.

use crate::data::Mesh;

/// Scene.
#[derive(Default, Debug, Clone)]
pub struct Scene {
    /// Scene name.
    name: Option<String>,
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
    /// Mesh index.
    MeshIndex;
}

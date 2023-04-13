use crate::storage;

pub struct World {
    archetypes: Archetypes,
    storages: Storages,
}

pub struct Archetypes {
    archetypes: Vec<Archetype>,
}

pub struct Archetype {
    id: ArchetypeIdentifier,
}

pub struct ArchetypeIdentifier(u32);

#[derive(Default)]
struct Storages {
    tables: Tables,
}

struct Tables {
    tables: Vec<Table>,
}

struct Table {
    columns: storage::SparseSet<ComponentId, storage::Column>,
    entities: Vec<Entity>,
}

struct ComponentId(u32);

pub struct Entity {
    generation: u32,
    index: u32,
}

impl Default for World {
    fn default() -> Self {
        Self {
            archetypes: Archetypes::new(),
            storages: Default::default(),
        }
    }
}

impl World {
    #[inline]
    pub fn new() -> World {
        World::default()
    }
}

impl Archetypes {
    pub fn new() -> Archetypes {
        Archetypes {
            archetypes: Vec::new(),
        }
    }
}

impl ArchetypeIdentifier {
    pub const EMPTY: ArchetypeIdentifier = ArchetypeIdentifier(0);
    pub const INVALID: ArchetypeIdentifier = ArchetypeIdentifier(u32::MAX);

    #[inline]
    pub const fn new(id: u32) -> ArchetypeIdentifier {
        ArchetypeIdentifier(id)
    }

    #[inline]
    pub fn index(self) -> u32 {
        self.0
    }
}

impl Default for Tables {
    fn default() -> Self {
        Self { tables: Vec::new() }
    }
}


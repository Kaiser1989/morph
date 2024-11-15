//////////////////////////////////////////////////
// Using

use std::marker::PhantomData;
use std::ops::Deref;

use shrev::{Event, EventChannel, ReaderId};
use specs::prelude::*;
use specs::storage::MaskedStorage;

//////////////////////////////////////////////////
// Definition

#[derive(Debug, Default)]
pub struct ComponentTracker<T>
where
    T: Component,
    T::Storage: Tracked,
{
    reader: Option<ReaderId<ComponentEvent>>,
    inserted: BitSet,
    modified: BitSet,
    removed: BitSet,
    phantom: PhantomData<T>,
}

//////////////////////////////////////////////////
// Implementation

impl<T> ComponentTracker<T>
where
    T: Component,
    T::Storage: Tracked,
{
    pub fn setup(&mut self, world: &mut World) {
        self.reader = Some(world.write_storage::<T>().register_reader());
    }

    pub fn update<D>(&mut self, storage: &Storage<T, D>)
    where
        D: Deref<Target = MaskedStorage<T>>,
    {
        // clear bitsets
        self.modified.clear();
        self.inserted.clear();
        self.removed.clear();
        // add events
        for event in storage.channel().read(&mut self.reader.as_mut().expect("Tacker must be initialized")) {
            match event {
                ComponentEvent::Inserted(id) => {
                    self.inserted.add(*id);
                }
                ComponentEvent::Modified(id) => {
                    self.modified.add(*id);
                }
                ComponentEvent::Removed(id) => {
                    self.removed.add(*id);
                }
            }
        }
    }

    #[inline]
    pub fn inserted(&self) -> &BitSet {
        &self.inserted
    }

    #[inline]
    pub fn modified(&self) -> &BitSet {
        &self.modified
    }

    #[inline]
    pub fn removed(&self) -> &BitSet {
        &self.removed
    }
}

//////////////////////////////////////////////////
// Trait implementation

impl<T: Event + Clone> Default for Events<T> {
    fn default() -> Self {
        Self::new()
    }
}

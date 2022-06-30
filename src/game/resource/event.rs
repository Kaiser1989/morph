//////////////////////////////////////////////////
// Using

use std::marker::PhantomData;
use std::ops::Deref;

use shrev::{Event, EventChannel, ReaderId};
use specs::prelude::*;
use specs::storage::MaskedStorage;

//////////////////////////////////////////////////
// Definition

#[derive(Debug)]
pub struct Events<T: Event + Clone> {
    channel: EventChannel<T>,
    queue: Vec<(f32, T)>,
}

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

impl<T: Event + Clone> Events<T> {
    pub fn new() -> Events<T> {
        Events {
            channel: EventChannel::new(),
            queue: Vec::new(),
        }
    }

    pub fn register(&mut self) -> ReaderId<T> {
        self.channel.register_reader()
    }

    pub fn write(&mut self, event: T) {
        self.channel.single_write(event);
    }

    pub fn write_delayed(&mut self, event: T, delay: f32) {
        self.queue.push((delay, event));
    }

    pub fn update_delayed(&mut self, elapsed_time: f32) {
        // update delay
        self.queue.iter_mut().for_each(|(time, _)| {
            *time -= elapsed_time;
        });
        // find ready events, write to channel, remove from queue
        let mut ready = Vec::new();
        self.queue.retain(|(time, event)| {
            if *time <= 0.0 {
                ready.push(event.clone());
                false
            } else {
                true
            }
        });
        self.channel.iter_write(ready.into_iter());
    }

    pub fn read(&self, reader: &mut ReaderId<T>) -> Vec<T> {
        self.channel.read(reader).cloned().collect()
    }

    pub fn read_opt(&self, reader: &mut Option<ReaderId<T>>) -> Vec<T> {
        if let Some(reader) = reader.as_mut() {
            self.read(reader)
        } else {
            Vec::new()
        }
    }

    pub fn ignore(&self, reader: &mut ReaderId<T>) {
        self.channel.read(reader);
    }
}

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

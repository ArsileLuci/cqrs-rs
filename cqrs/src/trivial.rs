use super::{Since, Precondition};
use super::{EventSource, EventAppend, SnapshotSource, SnapshotPersist, EventDecorator};
use super::{VersionedEvent, VersionedSnapshot};
use error::{AppendEventsError, Never};
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NullEventStore<Event, AggregateId> {
    _phantom: PhantomData<(Event, AggregateId)>,
}

impl<Event, AggregateId> Default for NullEventStore<Event, AggregateId> {
    fn default() -> Self {
        NullEventStore {
            _phantom: PhantomData,
        }
    }
}

impl<Event, AggregateId> EventSource for NullEventStore<Event, AggregateId> {
    type AggregateId = AggregateId;
    type Event = Event;
    type Events = Vec<VersionedEvent<Self::Event>>;
    type Error = Never;

    #[inline]
    fn read_events(&self, _aggregate_id: &Self::AggregateId, _version: Since) -> Result<Option<Self::Events>, Self::Error> {
        Ok(None)
    }
}

impl<Event, AggregateId> EventAppend for NullEventStore<Event, AggregateId> {
    type AggregateId = AggregateId;
    type Event = Event;
    type Error = AppendEventsError<Never>;

    #[inline]
    fn append_events(&self, _aggregate_id: &Self::AggregateId, _events: &[Self::Event], _condition: Precondition) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct NullSnapshotStore<Snapshot, AggregateId> {
    _phantom: PhantomData<(Snapshot, AggregateId)>,
}

impl<Snapshot, AggregateId> Default for NullSnapshotStore<Snapshot, AggregateId> {
    fn default() -> Self {
        NullSnapshotStore {
            _phantom: PhantomData,
        }
    }
}

impl<Snapshot, AggregateId> SnapshotSource for NullSnapshotStore<Snapshot, AggregateId> {
    type AggregateId = AggregateId;
    type Snapshot = Snapshot;
    type Error = Never;

    #[inline]
    fn get_snapshot(&self, _agg_id: &Self::AggregateId) -> Result<Option<VersionedSnapshot<Self::Snapshot>>, Self::Error> {
        Ok(None)
    }
}

impl<Snapshot, AggregateId> SnapshotPersist for NullSnapshotStore<Snapshot, AggregateId> {
    type AggregateId = AggregateId;
    type Snapshot = Snapshot;
    type Error = Never;

    #[inline]
    fn persist_snapshot(&self, _agg_id: &Self::AggregateId, _snapshot: VersionedSnapshot<Self::Snapshot>) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct NopEventDecorator<'a, Event: 'a> {
    _phantom: PhantomData<Event>,
    _phantom_lifetime: PhantomData<&'a ()>,
}

impl<'a, Event: 'a> Clone for NopEventDecorator<'a, Event> {
    fn clone(&self) -> Self {
        Default::default()
    }
}

impl<'a, Event: 'a> Copy for NopEventDecorator<'a, Event> {}

impl<'a, Event: 'a> Default for NopEventDecorator<'a, Event> {
    fn default() -> Self {
        NopEventDecorator {
            _phantom: PhantomData,
            _phantom_lifetime: PhantomData,
        }
    }
}

impl<'a, Event: 'a> EventDecorator for NopEventDecorator<'a, Event>
{
    type Event = Event;
    type DecoratedEvent = Event;

    #[inline]
    fn decorate(&self, event: Self::Event) -> Self::DecoratedEvent {
        event
    }
}

#[cfg(test)]
#[path = "trivial_tests.rs"]
mod tests;

use std::fmt::Debug;

use anyhow::Result;
use chrono::{Datelike as _, Timelike as _, Utc};
use tokio::sync::mpsc;
use tracing::instrument;
use ulid::{Generator, Ulid};

use crate::{event::Event, namespace::Namespace, producer::Producer};

pub type Sink = mpsc::Sender<Segment>;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Key(String);

impl From<&Segment> for Key {
    fn from(segment: &Segment) -> Self {
        Self(format!(
            "{}-{}-{}-{}-{}",
            *segment.namespace, segment.year, segment.month, segment.day, segment.hour
        ))
    }
}

pub struct Segment {
    pub namespace: Namespace,
    pub producer: Producer,
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,

    pub events: Vec<(Ulid, Event)>,
    pub size: usize,

    generator: Generator,
}

impl Debug for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Segment")
            .field("namespace", &self.namespace)
            .field("producer", &self.producer)
            .field("year", &self.year)
            .field("month", &self.month)
            .field("day", &self.day)
            .field("hour", &self.hour)
            .field("events", &self.events.len())
            .field("size", &self.size)
            .finish()
    }
}

impl Segment {
    pub fn new(namespace: Namespace, producer: Producer, events: Vec<Event>) -> Self {
        let now = Utc::now();

        let mut seg = Self {
            namespace,
            producer,

            year: now.year(),
            month: now.month(),
            day: now.day(),
            hour: now.hour(),

            events: Vec::with_capacity(events.len()),
            size: 0,

            generator: Generator::new(),
        };

        for event in events {
            seg.insert(event);
        }

        seg
    }

    #[instrument(level = "trace", skip(self), ret)]
    pub fn filename(&self) -> String {
        format!(
            "{}.{}.{}.{}.{}.{}_{}_{}",
            *self.namespace,
            self.year,
            self.month,
            self.day,
            self.hour,
            self.first_ulid().unwrap_or(Ulid::nil()),
            self.last_ulid().unwrap_or(Ulid::nil()),
            self.producer,
        )
    }

    #[instrument(level = "trace", skip(self), ret)]
    pub fn merge(&mut self, mut other: Segment) -> Result<(usize, usize)> {
        if Key::from(&*self) != Key::from(&other) {
            return Err(anyhow::anyhow!("segments key mismatch"));
        }

        self.events.append(&mut other.events);
        self.size += other.size;

        Ok((self.events.len(), self.size))
    }

    #[instrument(level = "trace", skip(self), ret)]
    fn insert(&mut self, event: Event) -> usize {
        let size = self.size + event.len();
        self.size = size;
        self.events.push((
            self.generator.generate().unwrap_or_else(|_| Ulid::new()),
            event,
        ));

        size
    }

    fn first_ulid(&self) -> Option<Ulid> {
        self.events.first().map(|(id, _)| *id)
    }

    fn last_ulid(&self) -> Option<Ulid> {
        self.events.last().map(|(id, _)| *id)
    }
}

#[cfg(test)]
mod tests {
    use super::Segment;
    use crate::{event::Event, namespace::Namespace, producer::Producer};

    #[test]
    fn segment_construct() {
        let namespace = Namespace::try_from("segmenter_bucktown_v5".to_string()).unwrap();
        let producer = Producer::try_from("segment_producer").unwrap();
        let payload = hex::encode("payload for bucket testing");
        let event = Event::try_from(payload.clone()).unwrap();
        let segment = Segment::new(namespace, producer, vec![event]);

        assert_eq!(
            segment.size,
            payload.len(),
            "event payload size is reflected in bucket size"
        );
        assert_eq!(segment.events.len(), 1);
    }
}

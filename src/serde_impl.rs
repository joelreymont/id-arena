use super::{Arena, Id};
use serde::ser::{Serialize, SerializeStruct, Serializer};

impl<T> Serialize for Id<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (self.idx, self.arena_id).serialize(serializer)
    }
}

impl<T> Serialize for Arena<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut arena = serializer.serialize_struct("Arena", 2)?;
        arena.serialize_field("arena_id", &self.arena_id)?;
        arena.serialize_field("items", &self.items)?;
        arena.end()
    }
}

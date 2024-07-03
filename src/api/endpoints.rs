use crate::choc::*;

pub enum EndpointType {
    Unknown = 0,
    Stream = 1,
    Value = 2,
    Event = 3
}

pub enum EndpointPurpose {
    Unknown,
    Console,
    AudioIn,
    MidiIn,
    MidiOut,
    ParameterControl,
    TimeSignature,
    Tempo,
    TransportState,
    TimlinePosition
}

pub struct EndpointDetailsList;

impl EndpointDetailsList {
    pub fn from_json(value: Value, is_input: bool) -> Self {
        todo!()
    }
}

pub struct EndpointId {
    id: String
}

impl EndpointId {
    pub fn create(id: String) -> Self {
        Self {
            id
        }
    }
}

impl std::fmt::Display for EndpointId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

pub struct EndpointDetails {
    pub id: EndpointId,
    pub ty: EndpointType,
    pub is_input: bool,
    // data_types: Vector<Type,
    pub annotation: serde_json::Value,
    pub location: String
}
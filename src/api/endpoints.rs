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

impl ToString for EndpointId {
    fn to_string(&self) -> String {
        self.id.clone()
    }
}

pub struct EndpointDetails {
    id: EndpointId,
    ty: EndpointType,
    is_input: bool,
    // data_types: Vector<Type,
    annotation: Value,
    location: String
}
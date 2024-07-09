use crate::choc::*;

#[derive(PartialEq)]
pub enum EndpointType {
    Unknown = 0,
    Stream = 1,
    Value = 2,
    Event = 3
}

#[derive(PartialEq)]
#[repr(u32)]
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
    pub types: Vec<Type>,
    pub annotation: Option<serde_json::Value>,
    pub location: Option<String>
}

impl EndpointDetails {
    pub fn is_output(&self) -> bool {
        !self.is_input
    }

    pub fn is_stream(&self) -> bool {
        self.ty == EndpointType::Stream
    }

    pub fn is_event(&self) -> bool {
        self.ty == EndpointType::Event
    }

    pub fn is_value(&self) -> bool {
        self.ty == EndpointType::Value
    }

    pub fn get_suggested_purpose(&self) -> EndpointPurpose {
        todo!()
    }

    pub fn is_console(&self) -> bool {
        todo!()
    }

    pub fn is_midi(&self) -> bool {
        todo!()
    }

    pub fn get_num_audio_channels(&self) -> u32 {
        todo!()
    }

    pub fn is_parameter(&self) -> bool {
        todo!()
    }

    pub fn is_timeline(&self) -> bool {
        todo!()
    }

    pub fn is_timeline_time_signature(&self) -> bool {
        todo!()
    }

    pub fn is_timeline_tempo(&self) -> bool {
        todo!()
    }

    pub fn is_timeline_transport_state(&self) -> bool {
        todo!()
    }

    pub fn is_timeline_position(&self) -> bool {
        todo!()
    }

    // to_json
    // from_json
}

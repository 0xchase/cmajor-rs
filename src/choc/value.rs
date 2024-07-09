#[repr(C)]
pub struct Value {

}

#[derive(Clone)]
pub enum Type {
    Int32,
    Int64,
    Float32,
    Float64,
    Vector {
        element: Box<Type>,
        size: u64
    },
    Class {
        name: String,
        members: Vec<(String, Type)>
    }
}

#[derive(Clone)]
pub struct VectorType {
    pub element: Box<Type>,
    pub size: u64
}

#[derive(Clone)]
pub struct ClassType {
    pub name: String,
    pub members: Vec<(String, Type)>
}

impl Type {
    pub fn from(value: &serde_json::Value) -> Self {
        match value["type"].as_str().unwrap() {
            "int32" => Self::Int32,
            "int64" => Self::Int64,
            "float32" => Self::Float32,
            "float64" => Self::Float64,
            "vector" => {
                Self::Vector {
                    element: Box::new(Type::from(&value["element"])),
                    size: value["size"].as_u64().unwrap()
                }
            }
            "object" => {
                let name = value["class"]
                    .as_str()
                    .unwrap()
                    .to_string();

                let mut members = Vec::new();
                if let Some(map) = value["members"].as_object() {
                    for name in map.keys() {
                        members.push((name.clone(), Type::from(&map[name])));
                    }

                    Self::Class {
                        name,
                        members
                    }
                } else {
                    panic!("Unsupported type {}", value);
                }
            },
            _ => panic!("Unsupported type {}", value)
        }
    }
}

pub struct Span<T> {
    pub start: T,
    pub end: T
}
pub struct LineAndColumn {
    line: u32,
    column: u32
}
pub struct CodeLocation;
pub struct SourceFile;

pub struct FullCodeLocation {
    file_name: String,
    source_line: String,
    line_and_column: LineAndColumn
}

impl FullCodeLocation {
    pub fn from(file: &SourceFile, location: CodeLocation) -> Self {
        todo!()
    }

    pub fn from2(file: &[SourceFile], location: CodeLocation) -> Self {
        todo!()
    }

    pub fn get_location_description(&self) -> String {
        todo!()
    }

    pub fn get_annotated_source_line(&self) -> String {
        todo!()
    }

    pub fn get_full_description(&self, severity: &str, description: &str) {
        todo!()
    }

    pub fn get_annotated_description(&self, severity: &str, description: &str) {
        todo!()
    }
}

pub struct DiagnosticMessageList {
    messages: Vec<DiagnosticMessage>,
}

impl DiagnosticMessageList {
    pub fn new() -> Self {
        Self {
            messages: Vec::new()
        }
    }

    pub fn push(&mut self, message: DiagnosticMessage) {
        println!("{}", message.to_string());
        self.messages.push(message);
    }

    pub fn add_from_json_string(&mut self, json: &str) -> bool {
        let mut no_errors = true;

        let v: serde_json::Value = serde_json::from_str(json).unwrap();
        println!("Adding diagnostic messages: {}", json);

        if v.is_array() {
            for message in v.as_array().unwrap() {
                let message = DiagnosticMessage::from_json(&message);
                if message.is_error() {
                    no_errors = false;
                }

                self.push(message);
            }
        } else {
            let message = DiagnosticMessage::from_json(&v);
            if message.is_error() {
                no_errors = false;
            }

            self.push(message);
        }

        return no_errors
    }

    pub fn has_errors(&self) -> bool {
        self
            .messages
            .iter()
            .fold(false, | b, m | {
                b || m.is_error()
            }) || self.has_internal_compiler_errors()
    }

    pub fn has_warnings(&self) -> bool {
        self
            .messages
            .iter()
            .fold(false, | b, m | {
                b || m.is_warning()
            })
    }

    pub fn has_notes(&self) -> bool {
        self
            .messages
            .iter()
            .fold(false, | b, m | {
                b || m.is_note()
            })
    }

    pub fn has_internal_compiler_errors(&self) -> bool {
        self
            .messages
            .iter()
            .fold(false, | b, m | {
                b || m.is_internal_compiler_error()
            })
    }
}

pub struct DiagnosticMessage {
    location: FullCodeLocation,
    description: String,
    kind: Kind,
    category: Category
}

impl DiagnosticMessage {
    pub fn new() -> Self {
        todo!()
    }

    pub fn from_json(json: &serde_json::Value) -> Self {
        let description = json["message"].as_str().unwrap_or("").to_owned();

        let location = FullCodeLocation {
            file_name: json["fileName"].to_string(),
            source_line: json["sourceLine"].to_string(),
            line_and_column: LineAndColumn {
                line: json["sourceLine"].as_i64().unwrap_or(0) as u32,
                column: json["columnNumber"].as_i64().unwrap_or(0) as u32
            }
        };

        let kind = match json["severity"].as_str().unwrap() {
            "warning" => Kind::Warning,
            "note" => Kind::Note,
            "error" => Kind::Error,
            _ => unreachable!()
        };

        let category = match json["category"].as_str().unwrap() {
            "compile" => Category::Compile,
            "runtime" => Category::Runtime,
            _ => Category::None
        };

        Self {
            location,
            description,
            kind,
            category
        }
    }

    pub fn from_json_str(json: &str) -> Self {
        Self::from_json(&serde_json::from_str(json).unwrap())
    }

    pub fn is_note(&self) -> bool {
        self.kind == Kind::Note
    }

    pub fn is_warning(&self) -> bool {
        self.kind == Kind::Warning
    }

    pub fn is_error(&self) -> bool {
        self.kind == Kind::Error || self.kind == Kind::InternalCompilerError
    }

    pub fn is_internal_compiler_error(&self) -> bool {
        self.kind == Kind::InternalCompilerError
    }

    pub fn get_severity(&self) -> String {
        if self.is_warning() {
            String::from("warning")
        } else if self.is_note() {
            String::from("note")
        } else {
            String::from("error")
        }
    }

    pub fn get_category(&self) -> String {
        match self.category {
            Category::Compile => String::from("compile"),
            Category::Runtime => String::from("runtime"),
            _ => String::from("none")
        }
    }

    pub fn get_full_description(&self) -> String {
        self.description.clone()
    }

    pub fn get_annotated_source_line(&self) -> String {
        todo!()
    }

    pub fn with_location(location: FullCodeLocation) -> Self {
        Self {
            location,
            description: String::new(),
            kind: Kind::Error,
            category: Category::None
        }
    }

    // pub fn with_context(context) -> Self { todo!() }

    pub fn create_error(message: &str) -> Self {
        todo!()
    }
}

impl ToString for DiagnosticMessage {
    fn to_string(&self) -> String {
        return format!("{} {} in file {} line {}: {}", self.get_category(), self.get_severity(), self.location.file_name, self.location.line_and_column.line, self.get_full_description());
    }
}

#[derive(PartialEq)]
pub enum Kind {
    Error,
    Warning,
    Note,
    InternalCompilerError
}

#[derive(PartialEq)]
pub enum Category {
    None,
    Compile,
    Runtime
}

pub struct LineAndColumn;
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
        self.messages.push(message);
    }

    pub fn add_from_json_string(&mut self, json: &str) -> bool {
        todo!()
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

    pub fn from_json(json: &str) -> Self {
        todo!()
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
        todo!()
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
        todo!()
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

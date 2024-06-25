use crate::{DiagnosticMessage, DiagnosticMessageList, Object, ProgramInterfaceVtable, SyntaxTreeOptions};
use crate::com::Library;

pub struct Program {
    pub object: Option<Object<ProgramInterfaceVtable>>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            object: None
        }
    }

    pub fn reset(&mut self) {
        self.object = None;
    }

    pub fn parse(&mut self, messages: &mut DiagnosticMessageList, file_name: &str, file_content: &str) -> bool {
        if self.object.is_none() {
            self.object = Some(Library::create_program());
        }

        if let Some(object) = &self.object {
            if let Err(error) = object.parse(file_name, file_content) {
                messages.push(DiagnosticMessage::from_json_str(&error));
                return false;
            }
        }

        true
    }

    pub fn get_syntax_tree(&self, namespace: &str, include_source_locations: bool, include_comments: bool, include_function_contents: bool) -> String {
        if let Some(object) = &self.object {
            object.get_syntax_tree(namespace, include_source_locations, include_comments, include_function_contents)
        } else {
            String::new()
        }
    }
}

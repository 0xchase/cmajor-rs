use crate::{DiagnosticMessage, DiagnosticMessageList, Object, ProgramInterfaceVtable, SyntaxTreeOptions};

pub struct Program {
    object: Option<Object<ProgramInterfaceVtable>>
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
        if let Some(object) = &self.object {
            if let Err(error) = object.parse(file_name, file_content) {
                messages.push(DiagnosticMessage::from_json(&error));
                return false;
            }
        }

        true
    }

    pub fn get_syntax_tree(&self, options: &SyntaxTreeOptions) -> String {
        todo!()
        // self.object.get_syntax_tree(namespace_or_module, include_source_locations, include_comments, include_function_contents)
    }
}

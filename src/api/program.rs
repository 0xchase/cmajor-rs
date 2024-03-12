use crate::{DiagnosticMessage, DiagnosticMessageList, Object, ProgramInterfaceVtable, SyntaxTreeOptions};

pub struct Program {
    object: Object<ProgramInterfaceVtable>
}

impl Program {
    pub fn reset(&mut self) {
        todo!()
    }

    pub fn parse(&mut self, messages: &mut DiagnosticMessageList, file_name: &str, file_content: &str) -> bool {
        todo!()
    }

    pub fn get_syntax_tree(&self, options: &SyntaxTreeOptions) -> String {
        todo!()
        // self.object.get_syntax_tree(namespace_or_module, include_source_locations, include_comments, include_function_contents)
    }
}
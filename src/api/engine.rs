use std::rc::Rc;

use crate::com::*;

use self::build_settings::BuildSettings;
use serde_json::Value;

use super::*;

type ExternalVariableProviderFn = fn(&ExternalVariable) -> Value;
type ExternalFunctionProviderFn = fn(&str, &[Type]);

pub struct Engine {
    engine: Object<EngineInterfaceVtable>,
}

impl Engine {
    pub fn create(engine_type: &str) -> Result<Self, String> {
        if let Ok(factory) = Library::create_engine_factory(engine_type) {
            if let Ok(engine) = factory.create_engine("") {
                Ok(Self { engine })
            } else {
                Err(String::from("Error creating engine interface"))
            }
        } else {
            Err(String::from("Error creating engine factory"))
        }
    }

    pub fn get_engine_types() -> Vec<String> {
        Library::get_engine_types()
    }

    pub fn get_build_settings(&self) -> BuildSettings {
        let settings = self.engine.get_build_settings();
        BuildSettings::from(settings.as_str())
    }

    pub fn set_build_settings(&mut self, new_settings: &BuildSettings) {
        self.engine.set_build_settings(&new_settings.to_json());
    }

    pub fn load(
        messages: &DiagnosticMessageList,
        program: &Program,
        external_variable: ExternalVariableProviderFn,
        external_function: ExternalFunctionProviderFn,
    ) -> bool {
        todo!()
    }

    pub fn unload(&self) {
        self.engine.unload();
    }

    pub fn get_input_endpoints(&self) -> EndpointDetailsList {
        todo!()
    }

    pub fn get_output_endpoints(&self) -> EndpointDetailsList {
        todo!()
    }

    pub fn get_endpoint_handle(&self, endpoint_id: &str) -> Result<EndpointHandle, String> {
        if self.engine.is_loaded() {
            Ok(self.engine.get_endpoint_handle(endpoint_id))
        } else {
            Err(String::from("Engine is not loaded"))
        }
    }

    pub fn get_program_details(&self) -> Value {
        todo!()
    }

    pub fn link(
        &self,
        messages: &mut DiagnosticMessageList,
        cache: &Object<CacheDatabaseInterfaceVtable>,
    ) -> bool {
        if !self.is_loaded() || self.is_linked() {
            messages.add(DiagnosticMessage::create_error(
                "Program must be loaded but not linked",
            ));
            return false;
        }

        if let Err(message) = &self.engine.link(cache) {
            return messages.add_from_json_string(message);
        }

        true
    }

    pub fn create_performer(&self) -> Result<Performer, String> {
        if !self.is_linked() {
            return Err(String::from("Engine is not linked"));
        }

        if let Ok(performer) = self.engine.create_performer() {
            Ok(Performer(performer))
        } else {
            return Err(String::from("Performer creation failed"));
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.engine.is_loaded()
    }

    pub fn is_linked(&self) -> bool {
        self.engine.is_linked()
    }

    pub fn get_last_build_log(&self) -> String {
        self.engine.get_last_build_log()
    }

    pub fn generate_code(&self, target_type: &str, extra_options: &str) -> CodeGenOutput {
        todo!()
    }

    pub fn get_available_code_gen_target_types(&self) -> Vec<String> {
        todo!()
    }
}

pub struct CodeGenOutput {
    generated_code: String,
    main_class_name: String,
    messages: DiagnosticMessageList,
}

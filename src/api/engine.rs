use std::{ffi::c_void, rc::Rc};

use crate::com::*;

use self::build_settings::BuildSettings;

use super::*;

use crate::choc::*;

type ExternalVariableProviderFn = fn(&ExternalVariable) -> Value;
type ExternalFunctionProviderFn = fn(*const i8, Span<Type>) -> *const c_void;

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
        // Library::get_engine_types()
        todo!()
    }

    pub fn get_build_settings(&self) -> BuildSettings {
        let settings = self.engine.get_build_settings();
        BuildSettings::from(settings.as_str())
    }

    pub fn set_build_settings(&mut self, new_settings: &BuildSettings) {
        self.engine.set_build_settings(&new_settings.to_json());
    }

    pub fn load(
        &mut self,
        messages: &mut DiagnosticMessageList,
        program: &Program,
        get_external_variable: ExternalVariableProviderFn,
        get_external_function: ExternalFunctionProviderFn,
    ) -> bool {
        if self.engine.is_null() {
            messages.push(DiagnosticMessage::create_error("missing engine"));
            return false;
        }

        let resolver = ExternalResolver::new(
            self.engine.clone(),
            get_external_variable,
            get_external_function,
        );

        let result = self.engine.load(
            program.object.clone().unwrap(),
            &resolver as *const ExternalResolver as *const c_void,
            ExternalResolver::resolve_variable,
            &resolver as *const ExternalResolver as *const c_void,
            ExternalResolver::resolve_function
        );

        if let Some(result) = result {
            return messages.add_from_json_string(&result);
        }

        return true;
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
        cache: Option<Object<CacheDatabaseInterfaceVtable>>,
    ) -> bool {
        if !self.is_loaded() || self.is_linked() {
            messages.push(DiagnosticMessage::create_error(
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
            Ok(Performer::from(performer))
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

struct ExternalResolver {
    engine: Object<EngineInterfaceVtable>,
    get_variable: ExternalVariableProviderFn,
    get_function: ExternalFunctionProviderFn,
}

impl ExternalResolver {
    fn new(
        engine: Object<EngineInterfaceVtable>,
        get_variable: ExternalVariableProviderFn,
        get_function: ExternalFunctionProviderFn,
    ) -> Self {
        Self {
            engine,
            get_variable,
            get_function,
        }
    }

    fn resolve_variable(context: *const c_void, ext: *const i8) {
        if !context.is_null() {
            // let external_variable = ExternalVariable::from_json(ext);
        } else {
            // Context is nullptr
        }

        todo!()
    }

    fn resolve_function(context: *const c_void, function_name: *const i8, parameter_types: *const i8) {
        if !context.is_null() {
            let mut types = Vec::<Type>::new();

            // if parseJSONTypeList(types, parameterTypes)) {
            //     return context->get_function(functionName, types);
            // }
        } else {
            // Context is nullptr
        }

        todo!()
    }

    fn parse_json_type_list(result: &Vec<Type>, json: &str) -> bool {
        todo!()
    }
}

pub struct CodeGenOutput {
    generated_code: String,
    main_class_name: String,
    messages: DiagnosticMessageList,
}

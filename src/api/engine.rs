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
            std::ptr::addr_of!(resolver) as *const c_void,
            ExternalResolver::resolve_variable,
            std::ptr::addr_of!(resolver) as *const c_void,
            ExternalResolver::resolve_function
        );

        if let Some(result) = result {
            messages.add_from_json_string(&result);
            todo!()
        }

        return true;
    }

    pub fn unload(&self) {
        self.engine.unload();
    }

    pub fn get_input_endpoints(&self) -> Vec<EndpointDetails> {
        let details = self.get_program_details();
        parse_endpoint_details(&details["inputs"], true)
    }

    pub fn get_output_endpoints(&self) -> Vec<EndpointDetails> {
        let details = self.get_program_details();
        parse_endpoint_details(&details["outputs"], false)
    }

    pub fn get_endpoint_handle(&self, endpoint_id: &str) -> Result<EndpointHandle, String> {
        if self.engine.is_loaded() {
            Ok(self.engine.get_endpoint_handle(endpoint_id))
        } else {
            Err(String::from("Engine is not loaded"))
        }
    }

    pub fn get_program_details(&self) -> serde_json::Value {
        if !self.is_loaded() {
            panic!("Should be loaded");
        }

        let details = self.engine.get_program_details().unwrap();
        serde_json::from_str(&details).unwrap()
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
            // println!("Error linking engine: {}", message);
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

pub fn parse_endpoint_details(v: &serde_json::Value, is_input: bool) -> Vec<EndpointDetails> {
    let mut endpoints = Vec::new();
    // println!("{}", v);
    for v in v.as_array().unwrap() {
        let id = v["endpointID"].as_str().unwrap().to_string();
        let id = EndpointId::create(id);
        let mut types = Vec::new();

        let ty = match v["endpointType"].as_str().unwrap() {
            "stream" => EndpointType::Stream,
            "value" => EndpointType::Value,
            "event" => EndpointType::Event,
            _ => unreachable!(),
        };

        let data_types = &v["dataTypes"];

        if data_types.is_array() {
            let arr = data_types.as_array().unwrap();
            for i in 0..arr.len() {
                let item = &arr[i];
                todo!();
            }
        } else {
            types.push(Type::from(&v["dataType"]));
        }

        let mut annotation = None;
        if let Some(ann) = v.get("annotation") {
            annotation = Some(ann.clone());
        }

        let mut location = None;
        if let Some(loc) = v.get("source") {
            location = Some(loc.to_string());
        }

        let endpoint = EndpointDetails {
            id,
            ty,
            types,
            is_input,
            annotation,
            location
        };

        endpoints.push(endpoint);
    }

    return endpoints;
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

    extern "C" fn resolve_variable(context: *const c_void, ext: *const i8) {
        println!("Resolve variable");
        unimplemented!();
    }

    extern "C" fn resolve_function(context: *const c_void, function_name: *const i8, parameter_types: *const i8) -> *const c_void {
        println!("Resolve function");
        unimplemented!();
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

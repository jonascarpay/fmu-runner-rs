use crate::{
    model_description::{FmiModelDescription, ScalarVariable},
    wrapper::FMIWrapper,
};
use dlopen::wrapper::Container;
use libfmi::*;
use std::{
    collections::HashMap,
    env,
    ffi::CString,
    fmt::Display,
    fs,
    iter::zip,
    ops::Deref,
    os,
    path::{Path, PathBuf},
};

pub struct FMU {
    fmu_path: PathBuf,
    model_description: FmiModelDescription,
}

pub struct FMUInstance {
    container: Container<FMIWrapper>,
    instance: *mut os::raw::c_void,
    simulation_type: fmi2Type,
    #[allow(dead_code)]
    callbacks: Box<fmi2CallbackFunctions>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum FMISignalType {
    Real,
    Integer,
    Boolean,
    // Char,
    String,
    // Byte
    Enum,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct FMUSignal<'fmu> {
    pub signal_type: FMISignalType,
    pub(crate) sv: &'fmu ScalarVariable,
}

impl FMU {
    pub fn new(fmu_path: &Path) -> Self {
        let fmu_path = fs::canonicalize(fmu_path).unwrap();
        println!("fmu_path: {:?}", fmu_path);

        let target_path = fmu_path.with_extension("");
        let _status = Self::unpack(&fmu_path, &target_path);

        let fmu_path = target_path;
        let model_description = Self::model_description(&fmu_path);

        Self {
            fmu_path: fmu_path.to_path_buf(),
            model_description,
        }
    }

    pub fn get_model_description(&self) -> &FmiModelDescription {
        return &self.model_description;
    }

    fn unpack(fmu_path: &Path, target: &Path) -> fmi2Status {
        let zipfile = std::fs::File::open(fmu_path).unwrap();
        let mut archive = zip::ZipArchive::new(zipfile).unwrap();
        let res = archive.extract(target);

        match res {
            Ok(_) => fmi2Status::fmi2OK,
            Err(_) => fmi2Status::fmi2Error,
        }
    }

    fn model_description(fmu_path: &Path) -> FmiModelDescription {
        let model_desc_path = fmu_path.join("modelDescription.xml");
        FmiModelDescription::new(&model_desc_path).unwrap()
    }
}

impl FMUInstance {
    pub fn instantiate(fmu: &FMU, simulation_type: fmi2Type, logging_on: bool) -> Self {
        let (os_type, lib_type) = match env::consts::OS {
            "macos" => ("darwin", "dylib"),
            "linux" => ("linux", "so"),
            "windows" => ("win", "dll"),
            _ => ("unknown", "so"),
        };

        let arch_type = match std::env::consts::ARCH {
            "x86" => "32",
            "x86_64" => "64",
            // "arm" => "32",
            "aarch64" => "64",
            _ => "unknown",
        };

        let fmu_guid = &fmu.model_description.guid;

        let mut model_identifier = "";
        if let Some(co_sim) = &fmu.model_description.co_simulation {
            match simulation_type {
                fmi2Type::fmi2ModelExchange => {}
                fmi2Type::fmi2CoSimulation => model_identifier = &co_sim.model_identifier,
            }
        }
        if let Some(mod_ex) = &fmu.model_description.model_exchange {
            match simulation_type {
                fmi2Type::fmi2ModelExchange => model_identifier = &mod_ex.model_identifier,
                fmi2Type::fmi2CoSimulation => {}
            }
        }

        // must be unique if we need multiple instances, not implemented for simplicity
        let instance_name = model_identifier.clone();

        // construct the library folder string
        let lib_str = os_type.to_owned() + arch_type;

        // construct the full library path
        let mut lib_path = Path::new(&fmu.fmu_path)
            .join("binaries")
            .join(lib_str)
            .join(model_identifier);
        lib_path.set_extension(lib_type);

        println!("lib_path: {:?}", lib_path);

        let callbacks = Box::<fmi2CallbackFunctions>::new(fmi2CallbackFunctions {
            logger: Some(logger::callback_logger_handler),
            allocateMemory: Some(libc::calloc),
            freeMemory: Some(libc::free),
            stepFinished: None,
            componentEnvironment: std::ptr::null_mut::<std::os::raw::c_void>(),
        });

        let fmu_guid = CString::new(fmu_guid.as_bytes()).expect("Error building CString");

        let instance_name = CString::new(instance_name).expect("Error building CString");

        let resource_location =
            "file://".to_owned() + Path::new(&fmu.fmu_path).join("resources").to_str().unwrap();
        // let resource_location = format!("{}{}{}", "file://", self.fmu_path, "resources");
        println!("res_path: {:?}", resource_location);
        let resource_location = CString::new(resource_location).expect("Error building CString");

        let visible = false as fmi2Boolean;
        let logging_on = logging_on as fmi2Boolean;

        let container: Container<FMIWrapper> = unsafe { Container::load(lib_path) }.unwrap();

        let instance = unsafe {
            container.instantiate(
                instance_name.as_ptr(),
                simulation_type,
                fmu_guid.as_ptr(),
                resource_location.as_ptr(),
                &*callbacks,
                visible,
                logging_on,
            )
        };

        if instance.is_null() {
            println!("Instantiation Failed");
        }

        Self {
            container: container,
            instance: instance,
            simulation_type: simulation_type,
            callbacks,
        }
    }

    pub fn get_types_platform(&self) -> &str {
        let types_platform =
            unsafe { std::ffi::CStr::from_ptr(self.container.get_types_platform()) }
                .to_str()
                .unwrap();
        types_platform
    }

    pub fn get_simulation_type(&self) -> fmi2Type {
        self.simulation_type
    }

    pub fn set_debug_logging(
        &self,
        logging_on: bool,
        log_categories: &[&str],
    ) -> Result<(), fmi2Status> {
        let category_cstr = log_categories
            .iter()
            .map(|c| CString::new(*c).unwrap())
            .collect::<Vec<_>>();

        let category_ptrs: Vec<_> = category_cstr.iter().map(|c| c.as_ptr()).collect();

        Self::ok_or_err(unsafe {
            self.container.set_debug_logging(
                self.instance,
                logging_on as fmi2Boolean,
                category_ptrs.len(),
                category_ptrs.as_ptr(),
            )
        })
    }

    pub fn setup_experiment(
        &self,
        start_time: f64,
        stop_time: Option<f64>,
        tolerance: Option<f64>,
    ) -> Result<(), fmi2Status> {
        Self::ok_or_err(unsafe {
            self.container.setup_experiment(
                self.instance,
                tolerance.is_some() as fmi2Boolean,
                tolerance.unwrap_or_else(|| 0.0),
                start_time,
                stop_time.is_some() as fmi2Boolean,
                stop_time.unwrap_or_else(|| 0.0),
            )
        })
    }

    pub fn enter_initialization_mode(&self) -> Result<(), fmi2Status> {
        Self::ok_or_err(unsafe { self.container.enter_initialization_mode(self.instance) })
    }

    pub fn exit_initialization_mode(&self) -> Result<(), fmi2Status> {
        Self::ok_or_err(unsafe { self.container.exit_initialization_mode(self.instance) })
    }

    pub fn get_reals<'fmu>(
        &'fmu self,
        signals: &[FMUSignal<'fmu>],
    ) -> Result<HashMap<FMUSignal, fmi2Real>, fmi2Status> {
        self.get(signals, FMIWrapper::get_real)
    }

    pub fn get_integers<'fmu>(
        &'fmu self,
        signals: &[FMUSignal<'fmu>],
    ) -> Result<HashMap<FMUSignal, fmi2Integer>, fmi2Status> {
        self.get(signals, FMIWrapper::get_integer)
    }

    pub fn get_booleans<'fmu>(
        &'fmu self,
        signals: &[FMUSignal<'fmu>],
    ) -> Result<HashMap<FMUSignal, fmi2Integer>, fmi2Status> {
        self.get(signals, FMIWrapper::get_boolean)
    }

    pub fn set_reals(&self, value_map: &HashMap<FMUSignal, fmi2Real>) -> Result<(), fmi2Status> {
        self.set(value_map, FMIWrapper::set_real)
    }

    pub fn set_integers(
        &self,
        value_map: &HashMap<FMUSignal, fmi2Integer>,
    ) -> Result<(), fmi2Status> {
        self.set(value_map, FMIWrapper::set_integer)
    }

    pub fn set_booleans(
        &self,
        value_map: &HashMap<FMUSignal, fmi2Integer>,
    ) -> Result<(), fmi2Status> {
        self.set(value_map, FMIWrapper::set_boolean)
    }

    pub fn do_step(
        &self,
        current_communication_point: fmi2Real,
        communication_step_size: fmi2Real,
        no_set_fmustate_prior_to_current_point: bool,
    ) -> Result<(), fmi2Status> {
        Self::ok_or_err(unsafe {
            self.container.do_step(
                self.instance,
                current_communication_point,
                communication_step_size,
                no_set_fmustate_prior_to_current_point as fmi2Boolean,
            )
        })
    }

    fn get<'fmu, T>(
        &'fmu self,
        signals: &[FMUSignal<'fmu>],
        func: unsafe fn(
            &FMIWrapper,
            fmi2Component,
            *const fmi2ValueReference,
            usize,
            *mut T,
        ) -> fmi2Status,
    ) -> Result<HashMap<FMUSignal, T>, fmi2Status> {
        let mut values = Vec::<T>::with_capacity(signals.len());
        match unsafe {
            values.set_len(signals.len());
            func(
                &self.container.deref(),
                self.instance,
                signals
                    .iter()
                    .map(|s| s.sv.value_reference)
                    .collect::<Vec<_>>()
                    .as_ptr(),
                signals.len(),
                values.as_mut_ptr(),
            )
        } {
            fmi2Status::fmi2OK => Ok(zip(signals.to_owned(), values).collect()),
            status => Err(status),
        }
    }

    fn set<T: Copy>(
        &self,
        value_map: &HashMap<FMUSignal, T>,
        func: unsafe fn(
            &FMIWrapper,
            fmi2Component,
            *const fmi2ValueReference,
            usize,
            *const T,
        ) -> fmi2Status,
    ) -> Result<(), fmi2Status> {
        let len = value_map.len();
        let mut vrs = Vec::<fmi2ValueReference>::with_capacity(len);
        let mut values = Vec::<T>::with_capacity(len);

        for (signal, value) in value_map.iter() {
            vrs.push(signal.sv.value_reference);
            values.push(*value);
        }

        Self::ok_or_err(unsafe {
            func(
                &self.container.deref(),
                self.instance,
                vrs.as_ptr(),
                len,
                values.as_ptr(),
            )
        })
    }

    fn ok_or_err(status: fmi2Status) -> Result<(), fmi2Status> {
        match status {
            fmi2Status::fmi2OK => Ok(()),
            status => Err(status),
        }
    }
}

impl Drop for FMUInstance {
    fn drop(&mut self) {
        unsafe { self.container.free_instance(self.instance) };
    }
}

pub fn outputs_to_string<T: Display>(outputs: &HashMap<FMUSignal, T>) -> String {
    let mut s = String::new();

    for (signal, value) in outputs.iter() {
        s.push_str(&format!("{}: {:.3} | ", signal.sv.name, value));
    }

    s
}

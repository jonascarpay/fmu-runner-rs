use crate::{
    model_description::{FmiModelDescription, FmuSignal},
    wrapper::FmiWrapper,
};
use dlopen::wrapper::Container;
use libfmi::*;
use std::{
    collections::HashMap, env, ffi::CString, fmt::Display, fs, io, iter::zip, ops::Deref, os,
    path::PathBuf,
};
use thiserror::Error;
use zip::result::ZipError;

/// A unpacked FMU with a parsed model description.
pub struct Fmu {
    #[allow(dead_code)]
    temp_dir: Option<tempfile::TempDir>,
    unpacked_dir: PathBuf,
    pub model_description: FmiModelDescription,
}

/// An instance of a loaded FMU dynamic library, ready to execute.
pub struct FmuInstance {
    container: Container<FmiWrapper>,
    instance: *mut os::raw::c_void,
    simulation_type: fmi2Type,
    #[allow(dead_code)]
    callbacks: Box<fmi2CallbackFunctions>,
}

impl Fmu {
    /// Unpack an FMU file to a tempdir and parse it's model description.
    pub fn unpack(fmu_path: impl Into<std::path::PathBuf>) -> Result<Self, FmuLoadError> {
        let temp_dir = tempfile::Builder::new().prefix("fmi-runner").tempdir()?;

        let fmu = Self::unpack_to(fmu_path, temp_dir.path())?;

        Ok(Self {
            temp_dir: Some(temp_dir),
            unpacked_dir: fmu.unpacked_dir,
            model_description: fmu.model_description,
        })
    }

    /// Unpack an FMU file to a given target dir and parse it's model description.
    pub fn unpack_to(
        fmu_path: impl Into<std::path::PathBuf>,
        target_dir: impl Into<std::path::PathBuf>,
    ) -> Result<Self, FmuLoadError> {
        let fmu_path = fs::canonicalize(fmu_path.into())?;
        let target_dir = target_dir.into();

        let zipfile = std::fs::File::open(fmu_path)?;
        let mut archive = zip::ZipArchive::new(zipfile)?;
        archive.extract(&target_dir)?;

        let model_description = FmiModelDescription::new(&target_dir.join("modelDescription.xml"))?;

        Ok(Self {
            temp_dir: None,
            unpacked_dir: target_dir,
            model_description,
        })
    }
}

unsafe impl Send for FmuInstance {}

impl FmuInstance {
    /// "dlopen" an FMU library and call `fmi2Instantiate()` on it.
    pub fn load(
        fmu: &Fmu,
        simulation_type: fmi2Type,
        logging_on: bool,
    ) -> Result<Self, FmuLoadError> {
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
        let mut lib_path = fmu
            .unpacked_dir
            .join("binaries")
            .join(lib_str)
            .join(model_identifier);
        lib_path.set_extension(lib_type);

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
            "file://".to_owned() + fmu.unpacked_dir.join("resources").to_str().unwrap();
        // let resource_location = format!("{}{}{}", "file://", self.fmu_path, "resources");
        let resource_location = CString::new(resource_location).expect("Error building CString");

        let visible = false as fmi2Boolean;
        let logging_on = logging_on as fmi2Boolean;

        let container: Container<FmiWrapper> = unsafe { Container::load(lib_path) }?;

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
            return Err(FmuLoadError::FmuInstantiateFailed);
        }

        Ok(Self {
            container,
            instance,
            simulation_type,
            callbacks,
        })
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
    ) -> Result<(), FmuError> {
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
    ) -> Result<(), FmuError> {
        Self::ok_or_err(unsafe {
            self.container.setup_experiment(
                self.instance,
                tolerance.is_some() as fmi2Boolean,
                tolerance.unwrap_or(0.0),
                start_time,
                stop_time.is_some() as fmi2Boolean,
                stop_time.unwrap_or(0.0),
            )
        })
    }

    pub fn enter_initialization_mode(&self) -> Result<(), FmuError> {
        Self::ok_or_err(unsafe { self.container.enter_initialization_mode(self.instance) })
    }

    pub fn exit_initialization_mode(&self) -> Result<(), FmuError> {
        Self::ok_or_err(unsafe { self.container.exit_initialization_mode(self.instance) })
    }

    pub fn get_reals<'fmu>(
        &'fmu self,
        signals: &[FmuSignal<'fmu>],
    ) -> Result<HashMap<FmuSignal, fmi2Real>, FmuError> {
        self.get(signals, FmiWrapper::get_real)
    }

    pub fn get_integers<'fmu>(
        &'fmu self,
        signals: &[FmuSignal<'fmu>],
    ) -> Result<HashMap<FmuSignal, fmi2Integer>, FmuError> {
        self.get(signals, FmiWrapper::get_integer)
    }

    pub fn get_booleans<'fmu>(
        &'fmu self,
        signals: &[FmuSignal<'fmu>],
    ) -> Result<HashMap<FmuSignal, fmi2Integer>, FmuError> {
        self.get(signals, FmiWrapper::get_boolean)
    }

    pub fn set_reals(&self, value_map: &HashMap<FmuSignal, fmi2Real>) -> Result<(), FmuError> {
        self.set(value_map, FmiWrapper::set_real)
    }

    pub fn set_integers(
        &self,
        value_map: &HashMap<FmuSignal, fmi2Integer>,
    ) -> Result<(), FmuError> {
        self.set(value_map, FmiWrapper::set_integer)
    }

    pub fn set_booleans(
        &self,
        value_map: &HashMap<FmuSignal, fmi2Integer>,
    ) -> Result<(), FmuError> {
        self.set(value_map, FmiWrapper::set_boolean)
    }

    pub fn do_step(
        &self,
        current_communication_point: fmi2Real,
        communication_step_size: fmi2Real,
        no_set_fmustate_prior_to_current_point: bool,
    ) -> Result<(), FmuError> {
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
        signals: &[FmuSignal<'fmu>],
        func: unsafe fn(
            &FmiWrapper,
            fmi2Component,
            *const fmi2ValueReference,
            usize,
            *mut T,
        ) -> fmi2Status,
    ) -> Result<HashMap<FmuSignal, T>, FmuError> {
        let mut values = Vec::<T>::with_capacity(signals.len());
        match unsafe {
            values.set_len(signals.len());
            func(
                self.container.deref(),
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
            status => Err(FmuError::BadFunctionCall(status)),
        }
    }

    fn set<T: Copy>(
        &self,
        value_map: &HashMap<FmuSignal, T>,
        func: unsafe fn(
            &FmiWrapper,
            fmi2Component,
            *const fmi2ValueReference,
            usize,
            *const T,
        ) -> fmi2Status,
    ) -> Result<(), FmuError> {
        let len = value_map.len();
        let mut vrs = Vec::<fmi2ValueReference>::with_capacity(len);
        let mut values = Vec::<T>::with_capacity(len);

        for (signal, value) in value_map.iter() {
            vrs.push(signal.sv.value_reference);
            values.push(*value);
        }

        Self::ok_or_err(unsafe {
            func(
                self.container.deref(),
                self.instance,
                vrs.as_ptr(),
                len,
                values.as_ptr(),
            )
        })
    }

    fn ok_or_err(status: fmi2Status) -> Result<(), FmuError> {
        match status {
            fmi2Status::fmi2OK => Ok(()),
            status => Err(FmuError::BadFunctionCall(status)),
        }
    }
}

impl Drop for FmuInstance {
    fn drop(&mut self) {
        unsafe { self.container.free_instance(self.instance) };
    }
}

pub fn outputs_to_string<T: Display>(outputs: &HashMap<FmuSignal, T>) -> String {
    let mut s = String::new();

    for (signal, value) in outputs.iter() {
        s.push_str(&format!("{}: {:.3} | ", signal.sv.name, value));
    }

    s
}

#[derive(Error, Debug)]
pub enum FmuLoadError {
    #[error("Invalid Fmu path")]
    InvalidPath(#[from] io::Error),
    #[error("Invalid Fmu archive")]
    InvalidFmu(#[from] ZipError),
    #[error("Invalid Fmu model description XML")]
    InvalidModelDescription(#[from] quick_xml::DeError),
    #[error("Error loading Fmu dynamic library")]
    DLOpen(#[from] dlopen::Error),
    #[error("fmi2Instantiate() call failed")]
    FmuInstantiateFailed,
}

#[derive(Error, Debug)]
pub enum FmuError {
    #[error("Fmu bad function call: {0:?}")]
    BadFunctionCall(fmi2Status),
    #[error("Fmu load error: {0}")]
    LoadError(#[from] FmuLoadError),
}

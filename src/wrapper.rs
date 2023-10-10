#![allow(clippy::too_many_arguments)]

extern crate dlopen;

use dlopen::wrapper::WrapperApi;
use dlopen_derive::WrapperApi;

use libfmi::*;

#[derive(WrapperApi)]
pub struct FMIWrapper {
    #[dlopen_name = "fmi2GetVersion"]
    get_version: unsafe extern "C" fn() -> *const ::std::os::raw::c_char,

    #[dlopen_name = "fmi2GetTypesPlatform"]
    get_types_platform: unsafe extern "C" fn() -> *const ::std::os::raw::c_char,

    #[dlopen_name = "fmi2GetReal"]
    get_real: unsafe extern "C" fn(
        c: fmi2Component,
        vr: *const fmi2ValueReference,
        nvr: usize,
        value: *mut fmi2Real,
    ) -> fmi2Status,

    #[dlopen_name = "fmi2SetReal"]
    set_real: unsafe extern "C" fn(
        c: fmi2Component,
        vr: *const fmi2ValueReference,
        nvr: usize,
        value: *const fmi2Real,
    ) -> fmi2Status,

    #[dlopen_name = "fmi2GetInteger"]
    get_integer: unsafe extern "C" fn(
        c: fmi2Component,
        vr: *const fmi2ValueReference,
        nvr: usize,
        value: *mut fmi2Integer,
    ) -> fmi2Status,

    #[dlopen_name = "fmi2SetInteger"]
    set_integer: unsafe extern "C" fn(
        c: fmi2Component,
        vr: *const fmi2ValueReference,
        nvr: usize,
        value: *const fmi2Integer,
    ) -> fmi2Status,

    #[dlopen_name = "fmi2GetBoolean"]
    get_boolean: unsafe extern "C" fn(
        c: fmi2Component,
        vr: *const fmi2ValueReference,
        nvr: usize,
        value: *mut fmi2Boolean,
    ) -> fmi2Status,

    #[dlopen_name = "fmi2SetBoolean"]
    set_boolean: unsafe extern "C" fn(
        c: fmi2Component,
        vr: *const fmi2ValueReference,
        nvr: usize,
        value: *const fmi2Boolean,
    ) -> fmi2Status,

    #[dlopen_name = "fmi2Instantiate"]
    instantiate: unsafe extern "C" fn(
        instance_name: fmi2String,
        fmu_type: fmi2Type,
        fmu_guid: fmi2String,
        fmu_resource_location: fmi2String,
        functions: *const fmi2CallbackFunctions,
        visible: fmi2Boolean,
        logging_on: fmi2Boolean,
    ) -> fmi2Component,

    #[dlopen_name = "fmi2SetDebugLogging"]
    set_debug_logging: unsafe extern "C" fn(
        c: fmi2Component,
        logging_on: fmi2Boolean,
        n_categories: usize,
        categories: *const fmi2String,
    ) -> fmi2Status,

    #[dlopen_name = "fmi2FreeInstance"]
    free_instance: unsafe extern "C" fn(c: fmi2Component),

    #[dlopen_name = "fmi2Terminate"]
    terminate: unsafe extern "C" fn(c: fmi2Component) -> fmi2Status,

    #[dlopen_name = "fmi2SetupExperiment"]
    setup_experiment: unsafe fn(
        c: fmi2Component,
        tolerance_defined: fmi2Boolean,
        tolerance: fmi2Real,
        start_time: fmi2Real,
        stop_time_defined: fmi2Boolean,
        stop_time: fmi2Real,
    ) -> fmi2Status,

    #[dlopen_name = "fmi2EnterInitializationMode"]
    enter_initialization_mode: unsafe extern "C" fn(c: fmi2Component) -> fmi2Status,

    #[dlopen_name = "fmi2ExitInitializationMode"]
    exit_initialization_mode: unsafe extern "C" fn(c: fmi2Component) -> fmi2Status,

    #[dlopen_name = "fmi2DoStep"]
    do_step: unsafe extern "C" fn(
        c: fmi2Component,
        current_communication_point: fmi2Real,
        communication_step_size: fmi2Real,
        no_set_fmustate_prior_to_current_point: fmi2Boolean,
    ) -> fmi2Status,
}

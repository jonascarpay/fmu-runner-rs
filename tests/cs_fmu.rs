use std::{collections::HashMap, path::Path};

use fmu_runner::*;

#[test]
fn test_bouncing_bal() {
    let fmu = FMU::new(Path::new("./tests/fmu/bouncing_ball.fmu"));

    let signals = fmu.get_model_description().map_signals();
    println!("signals: {:?}", signals);

    let fmu_cs = FMUInstance::instantiate(&fmu, fmi2Type::fmi2CoSimulation, true);

    fmu_cs.setup_experiment(0.0, None, None).unwrap();

    fmu_cs
        .set_reals(&HashMap::from([(signals["h_start"], 10.0)]))
        .unwrap();

    // Enter initialization
    fmu_cs.enter_initialization_mode().unwrap();

    // Retrieve modified start values
    let values = fmu_cs.get_reals(&[signals["h_start"]]).unwrap();
    assert_eq!(values[&signals["h_start"]], 10.0);

    // Exit initialization mode
    fmu_cs.exit_initialization_mode().unwrap();

    fmu_cs.do_step(0.0, 1.0, true).unwrap();

    let outputs = fmu_cs.get_reals(&[signals["h_m"]]).unwrap();

    println!("output: h_m {:?}", outputs[&signals["h_m"]]);
}

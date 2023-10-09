use std::{path::Path, time::Instant};

use fmu_runner::*;

#[test]
fn test_bouncing_bal() {
    let fmu_path = Path::new("./tests/fmu/bouncing_ball.fmu");

    let fmu = FMU::new(fmu_path);

    // let md = fmu.get_model_description();
    let signals = fmu.get_signal_map();

    println!("signal_map: {:?}", signals);

    // create parameter map
    let param_vars: Vec<&str> = Vec::from(["h_start"]);

    // create input map
    let input_signals: Vec<&str> = Vec::from([]);

    // create output map
    let output_signals = Vec::from(["h_m"]);

    let param_vrs = param_vars
        .iter()
        .map(|&x| signals.get(x).unwrap().value_reference)
        .collect::<Vec<_>>();
    let mut param_val = vec![0.0 as f64; param_vrs.len()];

    let input_vrs = input_signals
        .iter()
        .map(|&x| signals.get(x).unwrap().value_reference)
        .collect::<Vec<_>>();
    let _input_val = vec![0.0 as f64; input_vrs.len()];

    let output_vrs = output_signals
        .iter()
        .map(|&x| signals.get(x).unwrap().value_reference)
        .collect::<Vec<_>>();
    let mut output_val = vec![0.0 as f64; output_vrs.len()];

    let fmu_cs = FMUInstance::instantiate(&fmu, fmi2Type::fmi2CoSimulation, true);

    fmu_cs.setup_experiment(0.0, None, None);

    println!("setup_experiment");

    // Set start values inputs
    param_val[0] = 10.0;
    let _status = fmu_cs.set_real(&param_vrs, &param_val);

    // Enter initialization
    let status = fmu_cs.enter_initialization_mode();
    println!("enter_initialization_mode: {:?}", status);

    // Retrieve modified start values
    let status = fmu_cs.get_real(&param_vrs, &mut param_val);
    println!(
        "Modified default start values get_real: {:?} | {:?}",
        status, param_val
    );

    // Exit initialization mode
    let status = fmu_cs.exit_initialization_mode();
    println!("exit_initialization_mode: {:?}", status);

    // let mut simulation_time = 0.0;
    let step_size = 0.001;

    let print_count = (0.1 / step_size) as usize;

    let simulation_time_total = 3.0;

    let i_steps = (simulation_time_total / step_size) as usize;

    let now = Instant::now();

    for i in 0..i_steps {
        let simulation_time = i as f64 * step_size;

        // let _status = fmu_cs.set_real(&input_vrs, &input_val);

        let _status = fmu_cs.do_step(simulation_time, step_size, true);

        let status = fmu_cs.get_real(&output_vrs, &mut output_val);

        if i % print_count == 0 {
            println!(
                "time: {:.2} | {:?} | h_m: {:?}",
                simulation_time, status, output_val
            );
        }
    }

    let elapsed = now.elapsed();
    println!("Total: {} [ms]", elapsed.as_millis());
}

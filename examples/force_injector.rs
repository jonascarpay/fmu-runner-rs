use std::{collections::HashMap, f64::consts::PI, path::Path};

use fmu_runner::{outputs_to_string, Fmu, FmuInstance};
use libfmi::fmi2Type;

fn main() {
    let mut register_handler: Option<force_injector::RegisterHandlerFn> = None;

    let fmu = Fmu::unpack(Path::new("./tests/fmu/planar_ball.fmu"))
        .unwrap()
        .load_with_handler(fmi2Type::fmi2CoSimulation, |lib| {
            register_handler = unsafe { lib.get(b"register_handler\0") }
                .map(|sym| *sym)
                .ok();
        })
        .unwrap();

    let signals = fmu.variables();

    println!("signals: {:?}", signals);

    {
        let fmu_cs = FmuInstance::instantiate(&fmu, true).unwrap();

        fmu_cs.setup_experiment(0.0, None, None).unwrap();

        // Enter initialization
        fmu_cs.enter_initialization_mode().unwrap();

        let instance_id = 2;

        fmu_cs
            .set_integers(&HashMap::from([(&signals["instanceID"], instance_id)]))
            .unwrap();

        if let Some(register_handler) = register_handler {
            register_handler(instance_id, get_force);
        }

        // Exit initialization mode
        fmu_cs.exit_initialization_mode().unwrap();

        let mut sim_time = 0.0;
        const STEP_SIZE: f64 = 0.1;
        loop {
            fmu_cs.do_step(sim_time, STEP_SIZE, true).unwrap();
            sim_time += STEP_SIZE;

            let outputs = fmu_cs
                .get_reals(&[
                    &signals["position[1]"],
                    &signals["position[2]"],
                    &signals["velocity[1]"],
                    &signals["velocity[2]"],
                ])
                .unwrap();
            println!("t: {:.1} | {}", &sim_time, outputs_to_string(&outputs));

            if sim_time > 4.0 {
                break;
            }
        }
    }
}

extern "C" fn get_force(t: f64) -> force_injector::Vec2 {
    force_injector::Vec2 {
        x: 10.0 * (PI * t).cos(),
        y: 10.0 * (PI * t).cos(),
    }
}

use std::env;
use log::debug;
use windows::Devices::Radios::*;
use env_logger;

/// String representation of RadioState enum
fn state_to_str(state: RadioState) -> &'static str {
    match state {
        RadioState::On => "On",
        RadioState::Off => "Off",
        RadioState::Unknown => "Unknown",
        RadioState::Disabled => "Disabled",
        _ => "{Unknown enum value}"
    }
}

/// String representation of RadioAccessStatus enum
fn access_to_str(access: RadioAccessStatus) -> &'static str {
    match access {
        RadioAccessStatus::Unspecified => "Unspecified",
        RadioAccessStatus::Allowed => "Allowed",
        RadioAccessStatus::DeniedByUser => "DeniedByUser",
        RadioAccessStatus::DeniedBySystem => "DeniedBySystem",
        _ => "{Unknown enum value}"
    }
}

/// String representation of RadioKind enum
fn kind_to_str(kind: RadioKind) -> &'static str {
    match kind {
        RadioKind::Bluetooth => "Bluetooth",
        RadioKind::WiFi => "WiFi",
        RadioKind::FM => "FM",
        RadioKind::MobileBroadband => "MobileBroadband",
        RadioKind::Other => "Other",
        _ => "{Unknown enum value}"
    }
}

/// Command line option requesting new state
enum RequestedState {
    /// When requested a new state
    SingleState(RadioState), 
    /// When requsted to put off state and then on state, i.e. powercycle
    OffOn 
}

/// Well, the main
fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    debug!("Command line parameters are: {:#?}", &args);

    if args.len() != 3 {
        println!(r#"Not enough arguments to run the app. Please specify such parameters:
<exefile> <radio kind: w for wifi and b for bluetooth> <new state: on or off or offon>"#);
        return;
    }

    let kind_arg = &args[1];
    let kind_selection  = match kind_arg.as_str() {
        "w" => RadioKind::WiFi,
        "b" => RadioKind::Bluetooth,
        _ => {
            println!("Requested kind parameter is unknown: '{}'", kind_arg);
            return;
            }
    };

    let new_state_arg = &args[2];
    let new_state = match new_state_arg.as_str() {
        "on" => RequestedState::SingleState(RadioState::On),
        "off" => RequestedState::SingleState(RadioState::Off),
        "offon" => RequestedState::OffOn,
        _ => {
            println!("Requested state parameter is unknown '{}'", new_state_arg);
            return;
            }
    };

    // Get the list of radios
    let radios_async = match Radio::GetRadiosAsync() {
        Ok(async_call) => async_call,
        Err(e) => {panic!("Error during getting a list of radios: {}", e);}
    };
    let radios_list = match radios_async.get() {
        Ok(res) => res,
        Err(e) => {panic!("Error getting result of querying a radio list: {}", e)}
    };

    // Cycle and make action
    for r in &radios_list {

        let name = r.Name().unwrap();
        let kind = r.Kind().unwrap();
        let state = r.State().unwrap();
            
        debug!("Found a radio instance: name={}, kind={}, state={}", name, kind_to_str(kind), state_to_str(state));
    
        // If this is the requested kind and different state then make the change
        if kind == kind_selection {
            if let RequestedState::SingleState(new_single_state) = new_state {
                // We are asked to turn off or on
                if state != new_single_state {
                    // Request change of the state
                    println!("Change state of radio {} to {}", name, new_state_arg);
                    let access_result = match r.SetStateAsync(new_single_state) {
                        Ok(async_call) => {
                            println!("Done");
                            async_call.get().unwrap()
                        },
                        Err(e) => {
                            println!("Error changing state: {}", e);
                            return;
                        }
                    };
                    println!("Access result of the change is {}", access_to_str(access_result));
                }
                else
                {
                    println!("No change of the state is needed")
                }
            }
            else if let RequestedState::OffOn = new_state {
                // We are asked to power cycle
                println!("Doing powercycle of the radio");

                println!("Turning radio off");

                let access_result_off = match r.SetStateAsync(RadioState::Off) {
                    Ok(async_call) => {
                        println!("Done");
                        async_call.get().unwrap()
                    },
                    Err(e) => {
                        println!("Error changing state: {}", e);
                        return;
                    }
                };
                
                println!("Turning radio on");

                let access_result_on = match r.SetStateAsync(RadioState::On) {
                    Ok(async_call) => {
                        println!("Done");
                        async_call.get().unwrap()
                    },
                    Err(e) => {
                        println!("Error changing state: {}", e);
                        return;
                    }
                };
                
                println!("Access result of the change is {} for off and {} for on", access_to_str(access_result_off), access_to_str(access_result_on));
            }
        }
    };
}

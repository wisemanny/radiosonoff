use std::env;
use windows::Devices::Radios::*;

/* 
fn state_to_str(state: RadioState) -> &'static str {
    match state {
        RadioState::On => "On",
        RadioState::Off => "Off",
        RadioState::Unknown => "Unknown",
        RadioState::Disabled => "Disabled",
        _ => "Unknown enum value"
    }
}
*/

fn access_to_str(access: RadioAccessStatus) -> &'static str {
    match access {
        RadioAccessStatus::Unspecified => "Unspecified",
        RadioAccessStatus::Allowed => "Allowed",
        RadioAccessStatus::DeniedByUser => "DeniedByUser",
        RadioAccessStatus::DeniedBySystem => "DeniedBySystem",
        _ => "Unknown enum value"
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args.clone());

    if args.len() != 3 {
        println!(r#"Not enough arguments to run the app. Please specify:
<exefile> radio_kind <w for wifi and b for bluetooth> new_state <on or off>"#);
        return;
    }

    let kind_arg = &args[1];
    let kind_selection  = match kind_arg.as_str() {
        "w" => RadioKind::WiFi,
        "b" => RadioKind::Bluetooth,
        _ => {
            println!("Wrong kind requested '{}'", kind_arg);
            return;
            }
    };

    let new_state_arg = &args[2];
    let new_state = match new_state_arg.as_str() {
        "on" => RadioState::On,
        "off" => RadioState::Off,
        _ => {
            println!("Wrong state requested '{}'", new_state_arg);
            return;
            }
    };

    // Get the list of radios
    let radios_async = match Radio::GetRadiosAsync() {
        Ok(async_call) => async_call,
        Err(e) => {panic!("Error during get radios {}", e);}
    };
    let radios_list = match radios_async.get() {
        Ok(res) => res,
        Err(e) => {panic!("Error getting result {}", e)}
    };


    // Cycle and make action
    for r in &radios_list {
        let name = r.Name().unwrap();
        let kind = r.Kind().unwrap();
        let state = r.State().unwrap();
    
        if kind == kind_selection {
            //println!("name: {}", name);
            //println!("kind: {:#?}", kind);
            //println!("state: {:#?}", state);
            if state != new_state {
                println!("Change state of radio {} to {}", name, new_state_arg);
                let access_result = match r.SetStateAsync(new_state) {
                    Ok(async_call) => {
                        println!("Done");
                        async_call.get().unwrap()
                    },
                    Err(e) => {
                        println!("Error changing state {}", e);
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
    };
}
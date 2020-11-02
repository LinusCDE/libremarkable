use libremarkable::input::{ev::EvDevContext, ev::INPUT_DEVICE_PATHS, InputDevice, InputEvent};
use std::sync::mpsc::channel;

fn main() {
    let start = std::time::SystemTime::now();
    let model = libremarkable::device::DEVICE.get_model();
    let duration = start.elapsed();
    println!("This is a {} (took {:?} to determine)", model, duration);

    // Display paths for InputDevices
    for device in [
        InputDevice::GPIO,
        InputDevice::Multitouch,
        InputDevice::Wacom,
    ]
    .iter()
    {
        eprintln!("{:?} is {:?}", INPUT_DEVICE_PATHS[device], device);
    }

    // Send all input events to input_rx
    let (input_tx, input_rx) = channel::<InputEvent>();
    EvDevContext::new(InputDevice::GPIO, input_tx.clone()).start();
    EvDevContext::new(InputDevice::Multitouch, input_tx.clone()).start();
    EvDevContext::new(InputDevice::Wacom, input_tx).start();

    eprintln!("Waiting for input events...");
    while let Ok(event) = input_rx.recv() {
        println!("{:?}", event);
    }
    eprintln!("All event loops were closed?!?");
}

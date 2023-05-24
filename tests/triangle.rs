use std::ffi::CStr;

#[test]
fn triangle() {
    let instance =
        alexandria::Instance::new(CStr::from_bytes_with_nul(b"Triangle Test\0").unwrap(), 1)
            .unwrap();
    let window = instance.create_window("Triangle Test", 1920, 1080).unwrap();

    let device = locate_best_device(&window);
    println!("Selected device: {}", device.name());

    let graphics_context = window.create_graphics_context(device).unwrap();

    while window.poll_events().unwrap() {}
}

fn locate_best_device(window: &alexandria::Window) -> alexandria::Device {
    let mut best_device = None;
    for device in window.enumerate_devices().unwrap() {
        if let Some(old_best_device) = best_device {
            best_device = Some(compare_device_types(old_best_device, device));
        } else {
            if device_type_score(&device).is_some() {
                best_device = Some(device);
            }
        }
    }

    best_device.expect("No suitable device could be found")
}

fn compare_device_types(
    device1: alexandria::Device,
    device2: alexandria::Device,
) -> alexandria::Device {
    let device1_score = device_type_score(&device1).unwrap();
    let device2_score = device_type_score(&device2);

    if let Some(device2_score) = device2_score {
        if device1_score >= device2_score {
            device1
        } else {
            device2
        }
    } else {
        device1
    }
}

fn device_type_score(device: &alexandria::Device) -> Option<usize> {
    Some(match device.r#type() {
        alexandria::DeviceType::CPU => 1,
        alexandria::DeviceType::VirtualGPU => 2,
        alexandria::DeviceType::IntegratedGPU => 3,
        alexandria::DeviceType::DiscreteGPU => 4,
        _ => 0,
    })
}

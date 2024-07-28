macro_rules! get_device_proc_addr {
    ($f: expr, $device: expr, $name: expr) => {
        $f.get_device_proc_addr($device, $name)
            .ok_or($name)
            .map(|f| unsafe { ::std::mem::transmute(f) })
    };
}

pub(in crate::functions) use get_device_proc_addr;

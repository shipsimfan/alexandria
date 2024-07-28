macro_rules! get_instance_proc_addr {
    ($name: expr) => {
        $crate::functions::get_instance_proc_addr!(::std::ptr::null_mut(), $name)
    };
    ($instance: expr, $name: expr) => {
        unsafe { ::vulkan::vkGetInstanceProcAddr($instance, $name.as_ptr()) }
            .ok_or($name)
            .map(|f| unsafe { ::std::mem::transmute(f) })
    };
}

pub(in crate::functions) use get_instance_proc_addr;

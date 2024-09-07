use nvml_wrapper::enum_wrappers::device::TemperatureSensor;

fn main() {
    get_nvidia_gpu_info();
}

fn get_nvidia_gpu_info() {
    let nvml = nvml_wrapper::Nvml::init().expect("Failed to initialize NVML");
    println!("You have {0} NVIDIA GPUs", nvml.device_count().expect("Failed to list NVIDIA GPUs"));

    println!("NVIDIA Driver version: {0}", nvml.sys_driver_version().expect("Failed to retrieve NVIDIA driver version"));
    println!("NVIDIA CUDA version: {0}", nvml.sys_cuda_driver_version().expect("Failed to retrieve NVIDIA CUDA version"));

    for gpu_index in 0..nvml.device_count().unwrap() {        
        let gpu = nvml.device_by_index(gpu_index).expect("Failed to retrieve GPU with that index");
        println!("{0}: GPU Architecture: {1}", gpu_index, gpu.architecture().expect("Failed to retrieve GPU architecture"));
        println!("{0}: GPU Brand: {1:?}", gpu_index, gpu.brand().expect("Failed to get GPU brand"));
        println!("{0}: 🌡️  GPU Temperature: {1:?} °C ", gpu_index, gpu.temperature(TemperatureSensor::Gpu).expect("Failed to retrieve GPU temperature"));
        println!("{0}: GPU Driver Model: {1:?}", gpu_index, gpu.driver_model().expect("Failed to retrieve GPU driver model").current);
        println!("{0}: GPU Power Consumption: {1} watts", gpu_index, (gpu.power_usage().expect("Failed to get GPU power consumption")/1000) as f32);
        
    }
}
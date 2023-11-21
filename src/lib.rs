const SIZE: usize = 256;

pub async fn run() {
    // This will later store the raw pixel value data locally. We'll create it now as
    // a convenient size reference.
    let mut data = vec![0_u8; SIZE];

    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .unwrap();

    println!("Operating on: {:?}", adapter.get_info());

    let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

    std::mem::drop(shader);
}
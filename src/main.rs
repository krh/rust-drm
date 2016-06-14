mod drm;
mod gen9;

const I915_PARAM_EU_TOTAL: u32 =  34;

fn main() {
    let mut dev = drm::Device::new().unwrap();

    let eu_total = dev.get_param(I915_PARAM_EU_TOTAL).unwrap();

    println!("EU total: {}", eu_total);

    let mut buffer = dev.create_buffer(8192).unwrap();
    println!("buffer: {:?}", buffer);

    gen9::_3DSTATE_VS {
        command_type: 3,
        command_sub_type: 3,
        _3d_command_opcode: 0,
        _3d_command_sub_opcode: 16,
        dword_length: 7,
        kernel_start_pointer: 0,
        single_vertex_dispatch: false,
        vector_mask_enable: false,
        sampler_count: 0,
        binding_table_entry_count: 0,
        thread_dispatch_priority: 0,
        floating_point_mode: 0,
        illegal_opcode_exception_enable: false,
        accesses_uav: false,
        software_exception_enable: false,
        scratch_space_base_pointer: 0,
        per_thread_scratch_space: 0,
        dispatch_grf_start_register_for_urb_data: 0,
        vertex_urb_entry_read_length: 0,
        vertex_urb_entry_read_offset: 0,
        maximum_number_of_threads: 0,
        statistics_enable: true,
        simd8_dispatch_enable: true,
        vertex_cache_disable: false,
        function_enable: true,
        vertex_urb_entry_output_read_offset: 0,
        vertex_urb_entry_output_length: 0,
        user_clip_distance_clip_test_enable_bitmask: 0,
        user_clip_distance_cull_test_enable_bitmask: 0
    }.pack(&mut buffer);

    buffer.dump();
}

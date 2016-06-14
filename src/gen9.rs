fn gen_uint(value: u32, start: u32, end: u32) -> u32 {
    let width = end - start + 1;

    debug_assert!(width == 64 || value < (1 << width));

    value << start
}

fn gen_bool(value: bool, start: u32) -> u32 {
    if value { 1 << start } else { 0 }
}

#[allow(non_camel_case_types)]
pub struct _3DSTATE_VS {
    pub command_type:   u32,
    pub command_sub_type:   u32,
    pub _3d_command_opcode:   u32,
    pub _3d_command_sub_opcode:   u32,
    pub dword_length:   u32,
    pub kernel_start_pointer:   u64,
    pub single_vertex_dispatch:   bool,
    pub vector_mask_enable:   bool,
    pub sampler_count:   u32,
    pub binding_table_entry_count:   u32,
    pub thread_dispatch_priority:   u32,
    pub floating_point_mode:   u32,
    pub illegal_opcode_exception_enable:   bool,
    pub accesses_uav:   bool,
    pub software_exception_enable:   bool,
    pub scratch_space_base_pointer:   u64,
    pub per_thread_scratch_space:   u32,
    pub dispatch_grf_start_register_for_urb_data:   u32,
    pub vertex_urb_entry_read_length:   u32,
    pub vertex_urb_entry_read_offset:   u32,
    pub maximum_number_of_threads:   u32,
    pub statistics_enable:   bool,
    pub simd8_dispatch_enable:   bool,
    pub vertex_cache_disable:   bool,
    pub function_enable:   bool,
    pub vertex_urb_entry_output_read_offset:   u32,
    pub vertex_urb_entry_output_length:   u32,
    pub user_clip_distance_clip_test_enable_bitmask:   u32,
    pub user_clip_distance_cull_test_enable_bitmask:   u32,
}

impl _3DSTATE_VS {
    pub fn pack(&self, b: &mut super::drm::Buffer) {
        let dwords = [
            gen_uint(self.command_type, 29, 31) |
            gen_uint(self.command_sub_type, 27, 28) |
            gen_uint(self._3d_command_opcode, 24, 26) |
            gen_uint(self._3d_command_sub_opcode, 16, 23) |
            gen_uint(self.dword_length, 0, 7),

            0,
            0,

            gen_bool(self.single_vertex_dispatch, 31) |
            gen_bool(self.vector_mask_enable, 30) |
            gen_uint(self.sampler_count, 27, 29) |
            gen_uint(self.binding_table_entry_count, 18, 25) |
            gen_uint(self.thread_dispatch_priority, 17, 17) |
            gen_uint(self.floating_point_mode, 16, 16) |
            gen_bool(self.illegal_opcode_exception_enable, 13) |
            gen_bool(self.accesses_uav, 12) |
            gen_bool(self.software_exception_enable, 7),
            0,
        ];

        for n in dwords.iter() {
            println!("dw 0x{:x}", n)
        }

        b.emit(&dwords[..])
    }
}

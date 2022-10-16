use {crate::PhysicalDeviceFeatures, ash::vk};

impl PhysicalDeviceFeatures {
    /// Returns true if all of the features in this instance are supported by
    /// the available features.
    ///
    /// # Params
    ///
    /// * `available` - the available features
    ///
    /// # Returns
    ///
    /// False if any feature on `self` is `vk::TRUE` but `available` is
    /// `vk::FALSE`. True otherwise.
    pub fn is_supported_by(&self, available: &PhysicalDeviceFeatures) -> bool {
        fn this_and_not_that(this: u32, that: u32) -> bool {
            this == vk::TRUE && that == vk::FALSE
        }
        macro_rules! check_vulkan_13_feature {
            ($feature_name:ident) => {
                if this_and_not_that(
                    self.physical_device_vulkan_13_features.$feature_name,
                    available.physical_device_vulkan_13_features.$feature_name,
                ) {
                    log::warn!(
                        "maintenence4 - {} is not supported",
                        stringify!($feature_name)
                    );
                    return false;
                }
            };
        }
        macro_rules! check_descriptor_indexing_feature {
            ($feature_name:ident) => {
                if this_and_not_that(
                    self.descriptor_indexing_features.$feature_name,
                    available.descriptor_indexing_features.$feature_name,
                ) {
                    log::warn!(
                        "descriptor_indexing_features - {} is not supported",
                        stringify!($feature_name)
                    );
                    return false;
                }
            };
        }
        macro_rules! check_feature {
            ($feature_name:ident) => {
                if this_and_not_that(
                    self.physical_device_features2.features.$feature_name,
                    available.physical_device_features2.features.$feature_name,
                ) {
                    log::warn!(
                        "physical_device_features - {} is not supported",
                        stringify!($feature_name)
                    );
                    return false;
                }
            };
        }

        check_feature!(robust_buffer_access);
        check_feature!(full_draw_index_uint32);
        check_feature!(image_cube_array);
        check_feature!(independent_blend);
        check_feature!(geometry_shader);
        check_feature!(tessellation_shader);
        check_feature!(sample_rate_shading);
        check_feature!(dual_src_blend);
        check_feature!(logic_op);
        check_feature!(multi_draw_indirect);
        check_feature!(draw_indirect_first_instance);
        check_feature!(depth_clamp);
        check_feature!(depth_bias_clamp);
        check_feature!(fill_mode_non_solid);
        check_feature!(depth_bounds);
        check_feature!(wide_lines);
        check_feature!(large_points);
        check_feature!(alpha_to_one);
        check_feature!(multi_viewport);
        check_feature!(sampler_anisotropy);
        check_feature!(texture_compression_etc2);
        check_feature!(texture_compression_astc_ldr);
        check_feature!(texture_compression_bc);
        check_feature!(occlusion_query_precise);
        check_feature!(pipeline_statistics_query);
        check_feature!(vertex_pipeline_stores_and_atomics);
        check_feature!(fragment_stores_and_atomics);
        check_feature!(shader_tessellation_and_geometry_point_size);
        check_feature!(shader_image_gather_extended);
        check_feature!(shader_storage_image_extended_formats);
        check_feature!(shader_storage_image_multisample);
        check_feature!(shader_storage_image_read_without_format);
        check_feature!(shader_storage_image_write_without_format);
        check_feature!(shader_uniform_buffer_array_dynamic_indexing);
        check_feature!(shader_sampled_image_array_dynamic_indexing);
        check_feature!(shader_storage_buffer_array_dynamic_indexing);
        check_feature!(shader_storage_image_array_dynamic_indexing);
        check_feature!(shader_clip_distance);
        check_feature!(shader_cull_distance);
        check_feature!(shader_float64);
        check_feature!(shader_int64);
        check_feature!(shader_int16);
        check_feature!(shader_resource_residency);
        check_feature!(shader_resource_min_lod);
        check_feature!(sparse_binding);
        check_feature!(sparse_residency_buffer);
        check_feature!(sparse_residency_image2_d);
        check_feature!(sparse_residency_image3_d);
        check_feature!(sparse_residency2_samples);
        check_feature!(sparse_residency4_samples);
        check_feature!(sparse_residency8_samples);
        check_feature!(sparse_residency16_samples);
        check_feature!(sparse_residency_aliased);
        check_feature!(variable_multisample_rate);
        check_feature!(inherited_queries);

        check_vulkan_13_feature!(robust_image_access);
        check_vulkan_13_feature!(inline_uniform_block);
        check_vulkan_13_feature!(
            descriptor_binding_inline_uniform_block_update_after_bind
        );
        check_vulkan_13_feature!(pipeline_creation_cache_control);
        check_vulkan_13_feature!(private_data);
        check_vulkan_13_feature!(shader_demote_to_helper_invocation);
        check_vulkan_13_feature!(shader_terminate_invocation);
        check_vulkan_13_feature!(subgroup_size_control);
        check_vulkan_13_feature!(compute_full_subgroups);
        check_vulkan_13_feature!(synchronization2);
        check_vulkan_13_feature!(texture_compression_astc_hdr);
        check_vulkan_13_feature!(shader_zero_initialize_workgroup_memory);
        check_vulkan_13_feature!(dynamic_rendering);
        check_vulkan_13_feature!(shader_integer_dot_product);
        check_vulkan_13_feature!(maintenance4);

        check_descriptor_indexing_feature!(
            shader_input_attachment_array_dynamic_indexing
        );
        check_descriptor_indexing_feature!(
            shader_uniform_texel_buffer_array_dynamic_indexing
        );
        check_descriptor_indexing_feature!(
            shader_storage_texel_buffer_array_dynamic_indexing
        );
        check_descriptor_indexing_feature!(
            shader_uniform_buffer_array_non_uniform_indexing
        );
        check_descriptor_indexing_feature!(
            shader_sampled_image_array_non_uniform_indexing
        );
        check_descriptor_indexing_feature!(
            shader_storage_buffer_array_non_uniform_indexing
        );
        check_descriptor_indexing_feature!(
            shader_storage_image_array_non_uniform_indexing
        );
        check_descriptor_indexing_feature!(
            shader_input_attachment_array_non_uniform_indexing
        );
        check_descriptor_indexing_feature!(
            shader_uniform_texel_buffer_array_non_uniform_indexing
        );
        check_descriptor_indexing_feature!(
            shader_storage_texel_buffer_array_non_uniform_indexing
        );
        check_descriptor_indexing_feature!(
            descriptor_binding_uniform_buffer_update_after_bind
        );
        check_descriptor_indexing_feature!(
            descriptor_binding_sampled_image_update_after_bind
        );
        check_descriptor_indexing_feature!(
            descriptor_binding_storage_image_update_after_bind
        );
        check_descriptor_indexing_feature!(
            descriptor_binding_storage_buffer_update_after_bind
        );
        check_descriptor_indexing_feature!(
            descriptor_binding_uniform_texel_buffer_update_after_bind
        );
        check_descriptor_indexing_feature!(
            descriptor_binding_storage_texel_buffer_update_after_bind
        );
        check_descriptor_indexing_feature!(
            descriptor_binding_update_unused_while_pending
        );
        check_descriptor_indexing_feature!(descriptor_binding_partially_bound);
        check_descriptor_indexing_feature!(
            descriptor_binding_variable_descriptor_count
        );
        check_descriptor_indexing_feature!(runtime_descriptor_array);

        true
    }
}

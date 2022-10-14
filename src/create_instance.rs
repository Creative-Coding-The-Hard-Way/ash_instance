use {
    crate::{InstanceError, InstanceResult, VulkanInstance},
    ash::{extensions::ext::DebugUtils, vk},
    std::ffi::{c_char, CString},
};

impl VulkanInstance {
    pub(super) fn with_additional_extensions(
        required_extensions: &[String],
    ) -> Vec<String> {
        let mut required_extensions_with_debug = required_extensions.to_vec();
        if cfg!(debug_assertions) {
            required_extensions_with_debug
                .push(DebugUtils::name().to_str().unwrap().to_owned());
        }
        required_extensions_with_debug
    }

    /// Create the Ash loader and instance.
    pub(super) fn create_instance(
        required_extensions: &[String],
        required_layers: &[String],
    ) -> InstanceResult<(ash::Entry, ash::Instance)> {
        let entry = unsafe { ash::Entry::load()? };

        Self::check_extensions(&entry, required_extensions)?;
        Self::check_layers(&entry, required_layers)?;

        let (_layer_names, layer_ptrs) =
            unsafe { Self::to_os_ptrs(required_layers) };
        let (_ext_names, ext_ptrs) =
            unsafe { Self::to_os_ptrs(required_extensions) };

        let app_name = CString::new("ash starter").unwrap();
        let engine_name = CString::new("no engine").unwrap();

        let app_info = vk::ApplicationInfo {
            p_engine_name: engine_name.as_ptr(),
            p_application_name: app_name.as_ptr(),
            application_version: vk::make_api_version(0, 1, 0, 0),
            engine_version: vk::make_api_version(0, 1, 0, 0),
            api_version: vk::make_api_version(0, 1, 3, 0),
            ..Default::default()
        };
        let create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            pp_enabled_layer_names: layer_ptrs.as_ptr(),
            enabled_layer_count: layer_ptrs.len() as u32,
            pp_enabled_extension_names: ext_ptrs.as_ptr(),
            enabled_extension_count: ext_ptrs.len() as u32,
            ..Default::default()
        };
        let instance = unsafe { entry.create_instance(&create_info, None)? };

        Ok((entry, instance))
    }

    /// Check that all required extensions are available.
    ///
    /// # Params
    ///
    /// * `entry` - the Ash library entry
    /// * `required_extensions` - all of the extensions required by the
    ///   application
    ///
    /// # Returns
    ///
    /// Returns an error if any of the rquired extensions are missing.
    fn check_extensions(
        entry: &ash::Entry,
        required_extensions: &[String],
    ) -> InstanceResult<()> {
        let available_names: Vec<String> = entry
            .enumerate_instance_extension_properties(None)?
            .iter()
            .map(|ext| {
                String::from_utf8(
                    ext.extension_name
                        .into_iter()
                        .filter(|&c| c != 0)
                        .map(|c| c as u8)
                        .collect(),
                )
            })
            // only accept valid utf-8 extension names
            .filter_map(|item| item.ok())
            .collect();

        log::debug!("Available Vulkan extensions: {:?}", &available_names);

        let missing_extensions: Vec<String> = required_extensions
            .iter()
            .cloned()
            .filter(|name| !available_names.contains(name))
            .collect();

        if !missing_extensions.is_empty() {
            Err(InstanceError::MissingExtensions(missing_extensions))
        } else {
            Ok(())
        }
    }

    /// Check that all requried layers are available.
    ///
    /// # Params
    ///
    /// * `entry` - the Ash library entry
    /// * `required_layers` - all of the layer names required by the application
    ///
    /// # Returns
    ///
    /// Returns an error if any of the required layers are missing.
    fn check_layers(
        entry: &ash::Entry,
        required_layers: &[String],
    ) -> InstanceResult<()> {
        let available_names: Vec<String> = entry
            .enumerate_instance_layer_properties()?
            .iter()
            .map(|layer| {
                String::from_utf8(
                    layer
                        .layer_name
                        .into_iter()
                        .filter(|&c| c != 0)
                        .map(|c| c as u8)
                        .collect(),
                )
                .unwrap()
            })
            .collect();

        log::debug!("Available Vulkan layers: {:?}", &available_names);

        let missing: Vec<String> = required_layers
            .iter()
            .cloned()
            .filter(|name| !available_names.contains(name))
            .collect();

        if !missing.is_empty() {
            Err(InstanceError::MissingLayers(missing))
        } else {
            Ok(())
        }
    }

    /// Build a vector of CStrings and a matching vector of pointers to those
    /// strings.
    ///
    /// # Safety
    ///
    /// Unsafe because:
    ///   - The pointers are only valid so long as the returned strings are not
    ///     dropped or modified.
    pub(super) unsafe fn to_os_ptrs(
        strings: &[String],
    ) -> (Vec<CString>, Vec<*const c_char>) {
        let cstrings = strings
            .iter()
            .cloned()
            .map(|str| CString::new(str).unwrap())
            .collect::<Vec<CString>>();
        let ptrs = cstrings
            .iter()
            .map(|cstr| cstr.as_ptr())
            .collect::<Vec<*const c_char>>();
        (cstrings, ptrs)
    }
}

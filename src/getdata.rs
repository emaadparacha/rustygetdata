// Allow warnings for non-uppercase global variables.
// This is to match the naming conventions used in the external library.
#![allow(non_upper_case_globals)]

// Import the generated bindings for getdata.
use super::getdata_bindings::*;
use std::ffi::CString;

/// Represents a wrapper for a DIRFILE, providing methods for interacting with Dirfile data.
pub struct Dirfile {
    /// Pointer to the opened DIRFILE instance.
    pub dirfile_open: *mut DIRFILE,
}

impl Dirfile {
    /// Creates a new `Dirfile` instance by opening a specified path.
    ///
    /// # Arguments
    /// * `path` - A string slice that holds the path to the Dirfile.
    ///
    /// # Returns
    /// A new `Dirfile` instance.
    ///
    /// # Panics
    /// This function will panic if the path cannot be converted to a `CString`.
    pub fn new(path: &str) -> Self {
        let dirfile = CString::new(path).expect("CString::new failed");
        let dirfile_ptr = dirfile.as_ptr();
        let dirfile_open = unsafe { gd_open(dirfile_ptr, GD_RDONLY) };
        Self {
            dirfile_open,
        }
    }

    /// Retrieves the number of fields in the Dirfile.
    ///
    /// # Returns
    /// The number of fields as a `u32`.
    pub fn nfields(&self) -> u32 {
        unsafe { gd_nfields(self.dirfile_open) }
    }

    /// Retrieves the total number of frames in the Dirfile.
    ///
    /// # Returns
    /// The number of frames as an `i64`.
    pub fn nframes(&self) -> i64 {
        unsafe { gd_nframes(self.dirfile_open) }
    }

    /// Gets the number of samples per frame for a specified field.
    ///
    /// # Arguments
    /// * `field` - The field name as a string slice.
    ///
    /// # Returns
    /// The number of samples per frame as a `u32`.
    ///
    /// # Panics
    /// This function will panic if the field name cannot be converted to a `CString`.
    pub fn spf(&self, field: &str) -> u32 {
        let field_code = CString::new(field).expect("CString::new failed");
        let field_code_ptr = field_code.as_ptr();
        unsafe { gd_spf(self.dirfile_open, field_code_ptr) }
    }

    /// Retrieves the type of a specified field in the Dirfile.
    ///
    /// # Arguments
    /// * `field` - The field name as a string slice.
    ///
    /// # Returns
    /// The field type as a `u32`.
    ///
    /// # Panics
    /// This function will panic if the field name cannot be converted to a `CString`.
    pub fn field_type(&self, field: &str) -> u32 {
        let field_code = CString::new(field).expect("CString::new failed");
        let field_code_ptr = field_code.as_ptr();
        unsafe { gd_native_type(self.dirfile_open, field_code_ptr) }
    }

    /// Retrieves the data for a specified field in the Dirfile and converts it to a `Vec<f64>`.
    ///
    /// # Arguments
    /// * `field` - The field name as a string slice.
    ///
    /// # Returns
    /// A vector containing the data as `f64` values. This is a general-purpose conversion 
    /// that may not preserve the original precision of some data types.
    ///
    /// # Panics
    /// This function will panic if the field name cannot be converted to a `CString`.
    pub fn get_data(&self, field: &str) -> Vec<f64> {
        let field_type = self.field_type(field);
        let nframes = self.nframes();
        let samples_per_frame = self.spf(field);
        let total_samples = nframes * (samples_per_frame as i64);
    
        let field_code = CString::new(field).expect("CString::new failed");
        let field_code_ptr = field_code.as_ptr();
    
        // Extract the data based on its type and convert to `Vec<f64>`.
        let data: Vec<f64> = match field_type {
            gd_type_t_GD_UINT8 => {
                let mut raw_data = vec![0u8; total_samples as usize];
                unsafe {
                    gd_getdata(
                        self.dirfile_open,
                        field_code_ptr,
                        0, 0,
                        nframes as usize,
                        samples_per_frame as usize,
                        gd_type_t_GD_UINT8,
                        raw_data.as_mut_ptr() as *mut ::std::os::raw::c_void,
                    );
                }
                raw_data.iter().map(|&v| v as f64).collect()
            }
            gd_type_t_GD_INT8 => {
                let mut raw_data = vec![0i8; total_samples as usize];
                unsafe {
                    gd_getdata(
                        self.dirfile_open,
                        field_code_ptr,
                        0, 0,
                        nframes as usize,
                        samples_per_frame as usize,
                        gd_type_t_GD_INT8,
                        raw_data.as_mut_ptr() as *mut ::std::os::raw::c_void,
                    );
                }
                raw_data.iter().map(|&v| v as f64).collect()
            }
            gd_type_t_GD_UINT16 => {
                let mut raw_data = vec![0u16; total_samples as usize];
                unsafe {
                    gd_getdata(
                        self.dirfile_open,
                        field_code_ptr,
                        0, 0,
                        nframes as usize,
                        samples_per_frame as usize,
                        gd_type_t_GD_UINT16,
                        raw_data.as_mut_ptr() as *mut ::std::os::raw::c_void,
                    );
                }
                raw_data.iter().map(|&v| v as f64).collect()
            }
            gd_type_t_GD_INT16 => {
                let mut raw_data = vec![0i16; total_samples as usize];
                unsafe {
                    gd_getdata(
                        self.dirfile_open,
                        field_code_ptr,
                        0, 0,
                        nframes as usize,
                        samples_per_frame as usize,
                        gd_type_t_GD_INT16,
                        raw_data.as_mut_ptr() as *mut ::std::os::raw::c_void,
                    );
                }
                raw_data.iter().map(|&v| v as f64).collect()
            }
            gd_type_t_GD_UINT32 => {
                let mut raw_data = vec![0u32; total_samples as usize];
                unsafe {
                    gd_getdata(
                        self.dirfile_open,
                        field_code_ptr,
                        0, 0,
                        nframes as usize,
                        samples_per_frame as usize,
                        gd_type_t_GD_UINT32,
                        raw_data.as_mut_ptr() as *mut ::std::os::raw::c_void,
                    );
                }
                raw_data.iter().map(|&v| v as f64).collect()
            }
            gd_type_t_GD_INT32 => {
                let mut raw_data = vec![0i32; total_samples as usize];
                unsafe {
                    gd_getdata(
                        self.dirfile_open,
                        field_code_ptr,
                        0, 0,
                        nframes as usize,
                        samples_per_frame as usize,
                        gd_type_t_GD_INT32,
                        raw_data.as_mut_ptr() as *mut ::std::os::raw::c_void,
                    );
                }
                raw_data.iter().map(|&v| v as f64).collect()
            }
            gd_type_t_GD_UINT64 => {
                let mut raw_data = vec![0u64; total_samples as usize];
                unsafe {
                    gd_getdata(
                        self.dirfile_open,
                        field_code_ptr,
                        0, 0,
                        nframes as usize,
                        samples_per_frame as usize,
                        gd_type_t_GD_UINT64,
                        raw_data.as_mut_ptr() as *mut ::std::os::raw::c_void,
                    );
                }
                raw_data.iter().map(|&v| v as f64).collect()
            }
            gd_type_t_GD_INT64 => {
                let mut raw_data = vec![0i64; total_samples as usize];
                unsafe {
                    gd_getdata(
                        self.dirfile_open,
                        field_code_ptr,
                        0, 0,
                        nframes as usize,
                        samples_per_frame as usize,
                        gd_type_t_GD_INT64,
                        raw_data.as_mut_ptr() as *mut ::std::os::raw::c_void,
                    );
                }
                raw_data.iter().map(|&v| v as f64).collect()
            }
            gd_type_t_GD_FLOAT32 => {
                let mut raw_data = vec![0.0f32; total_samples as usize];
                unsafe {
                    gd_getdata(
                        self.dirfile_open,
                        field_code_ptr,
                        0, 0,
                        nframes as usize,
                        samples_per_frame as usize,
                        gd_type_t_GD_FLOAT32,
                        raw_data.as_mut_ptr() as *mut ::std::os::raw::c_void,
                    );
                }
                raw_data.iter().map(|&v| v as f64).collect()
            }
            gd_type_t_GD_FLOAT64 => {
                let mut raw_data = vec![0.0f64; total_samples as usize];
                unsafe {
                    gd_getdata(
                        self.dirfile_open,
                        field_code_ptr,
                        0, 0,
                        nframes as usize,
                        samples_per_frame as usize,
                        gd_type_t_GD_FLOAT64,
                        raw_data.as_mut_ptr() as *mut ::std::os::raw::c_void,
                    );
                }
                raw_data
            }
            gd_type_t_GD_STRING => {
                let mut raw_data = vec![CString::new("").unwrap(); total_samples as usize];
                unsafe {
                    gd_getdata(
                        self.dirfile_open,
                        field_code_ptr,
                        0, 0,
                        nframes as usize,
                        samples_per_frame as usize,
                        gd_type_t_GD_STRING,
                        raw_data.as_mut_ptr() as *mut ::std::os::raw::c_void,
                    );
                }
                raw_data.into_iter()
                    .filter_map(|c| c.into_string().ok())
                    .map(|s| s.parse::<f64>().unwrap_or(0.0)) // Converts to f64 or defaults to 0.0.
                    .collect()
            }
            _ => {
                println!("Unknown field type: {}", field_type);
                Vec::new()
            }
        };
    
        data // Return the processed data.
    }    
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    // Path to the test Dirfile.
    const TEST_DIRFILE_PATH: &str = "/data/test_dirfile";

    // Test field name
    const TEST_FIELD_NAME: &str = "time";

    // Fake field name
    const FAKE_FIELD_NAME: &str = "fake_field";

    #[test]
    fn test_dirfile_creation() {
        // Ensure the Dirfile is created successfully from the provided path.
        if !Path::new(TEST_DIRFILE_PATH).exists() {
            panic!("Test Dirfile path does not exist: {}", TEST_DIRFILE_PATH);
        }

        let dirfile = Dirfile::new(TEST_DIRFILE_PATH);
        assert!(!dirfile.dirfile_open.is_null(), "Failed to open Dirfile");
    }

    #[test]
    fn test_nfields() {
        let dirfile = Dirfile::new(TEST_DIRFILE_PATH);
        let num_fields = dirfile.nfields();
        assert!(num_fields > 0, "Number of fields should be greater than zero");
    }

    #[test]
    fn test_nframes() {
        let dirfile = Dirfile::new(TEST_DIRFILE_PATH);
        let num_frames = dirfile.nframes();
        assert!(num_frames > 0, "Number of frames should be greater than zero");
    }

    #[test]
    fn test_spf() {
        let dirfile = Dirfile::new(TEST_DIRFILE_PATH);
        let field_name = TEST_FIELD_NAME;
        let samples_per_frame = dirfile.spf(field_name);
        assert!(samples_per_frame > 0, "Samples per frame should be greater than zero");
    }

    #[test]
    fn test_field_type() {
        let dirfile = Dirfile::new(TEST_DIRFILE_PATH);
        let field_name = TEST_FIELD_NAME;
        let field_type = dirfile.field_type(field_name);
        assert!(field_type > 0, "Field type should be greater than zero");
    }

    #[test]
    fn test_get_data() {
        let dirfile = Dirfile::new(TEST_DIRFILE_PATH);
        let field_name = TEST_FIELD_NAME;
        let data = dirfile.get_data(field_name);
        assert!(!data.is_empty(), "Data should not be empty");
        assert!(data.iter().all(|&value| value.is_finite()), "All data values should be finite");
    }

    #[test]
    fn test_get_data_unknown_field() {
        let dirfile = Dirfile::new(TEST_DIRFILE_PATH);
        let unknown_field = FAKE_FIELD_NAME;
        let data = dirfile.get_data(unknown_field);
        assert!(data.is_empty(), "Data for an unknown field should be empty");
    }
}

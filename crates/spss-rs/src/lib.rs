use std::ffi::{CStr, CString};
use std::os::raw::c_long;
use std::ptr;
use std::error::Error;
use std::os::raw::{c_char, c_int};
use libspssio_sys::ffi;
use spsserror::SpssioErrorCode;

mod spsserror;

pub struct SPSSFile {
    handle: c_int,
}

impl SPSSFile {
    pub fn open_read(file_name: &str) -> Result<Self, Box<dyn Error>> {
        let file_name_c = CString::new(file_name)?;
        let mut handle: c_int = 0;

        let result = unsafe { ffi::spssOpenRead(file_name_c.as_ptr(), &mut handle) };
        if result != 0 {
            return Err(Box::new(std::io::Error::from_raw_os_error(result)));
        }

        Ok(SPSSFile { handle })
    }

    pub fn close_read(&self) -> Result<(), Box<dyn Error>> {
        let result = unsafe { ffi::spssCloseRead(self.handle) };
        if result != 0 {
            return Err(Box::new(std::io::Error::from_raw_os_error(result)));
        }
        Ok(())
    }

    pub fn get_file_code_page(&self) -> Result<i32, Box<dyn Error>> {
        let mut code_page: c_int = 0;
        let result = unsafe { ffi::spssGetFileCodePage(self.handle, &mut code_page) };
        if result != 0 {
            return Err(Box::new(std::io::Error::from_raw_os_error(result)));
        }
        Ok(code_page)
    }

    pub fn get_file_encoding(&self) -> Result<String, Box<dyn Error>> {
        let mut encoding: [c_char; 256] = [0; 256];
        let result = unsafe { ffi::spssGetFileEncoding(self.handle, encoding.as_mut_ptr()) };
        if result != 0 {
            return Err(Box::new(std::io::Error::from_raw_os_error(result)));
        }
        let encoding_str = unsafe { CStr::from_ptr(encoding.as_ptr()) }.to_str()?.to_string();
        Ok(encoding_str)
    }

    pub fn get_number_of_variables(&self) -> Result<i32, Box<dyn Error>> {
        let mut num_vars: c_int = 0;
        let result = unsafe { ffi::spssGetNumberofVariables(self.handle, &mut num_vars) };
        if result != 0 {
            return Err(Box::new(std::io::Error::from_raw_os_error(result)));
        }
        Ok(num_vars)
    }

    pub fn get_var_names(&self) -> Result<Vec<(String, i32)>, Box<dyn Error>> {
        let mut num_vars: c_int = 0;
        let mut var_names: *mut *mut c_char = ptr::null_mut();
        let mut var_types: *mut c_int = ptr::null_mut();

        let result = unsafe { ffi::spssGetVarNames(self.handle, &mut num_vars, &mut var_names, &mut var_types) };
        if result != 0 {
            return Err(Box::new(std::io::Error::from_raw_os_error(result)));
        }

        let mut variables = Vec::new();
        for i in 0..num_vars {
            let name_ptr = unsafe { *var_names.offset(i as isize) };
            let type_ptr = unsafe { *var_types.offset(i as isize) };

            let name = unsafe { CStr::from_ptr(name_ptr) }.to_str()?.to_string();
            let var_type = type_ptr ;

            variables.push((name, var_type));
        }

        Ok(variables)
    }

    pub fn get_var_label(&self, var_name: &str) -> Result<String, Box<dyn Error>> {
        let var_name_c = CString::new(var_name)?;
        let mut var_label: [c_char; 256] = [0; 256];
        let result = unsafe { ffi::spssGetVarLabel(self.handle, var_name_c.as_ptr(), var_label.as_mut_ptr()) };
        if result != 0 {
            return Err(Box::new(std::io::Error::from_raw_os_error(result)));
        }
        let var_label_str = unsafe { CStr::from_ptr(var_label.as_ptr()) }.to_str()?.to_string();
        Ok(var_label_str)
    }

    pub fn get_var_label_long(&self, var_name: &str) -> Result<String, Box<dyn Error>> {
        let var_name_c = CString::new(var_name)?;
        let mut var_label: [c_char; 256] = [0; 256];
        let mut len_label: c_int = 0;
        let result = unsafe { ffi::spssGetVarLabelLong(self.handle, var_name_c.as_ptr(), var_label.as_mut_ptr(), 256, &mut len_label) };
        if result != 0 {
            return Err(Box::new(std::io::Error::from_raw_os_error(result)));
        }
        let var_label_str = unsafe { CStr::from_ptr(var_label.as_ptr()) }.to_str()?.to_string();
        Ok(var_label_str)
    }

    pub fn get_var_measure_level(&self, var_name: &str) -> Result<i32, Box<dyn Error>> {
        let var_name_c = CString::new(var_name)?;
        let mut measure_level: c_int = 0;
        let result = unsafe { ffi::spssGetVarMeasureLevel(self.handle, var_name_c.as_ptr(), &mut measure_level) };
        if result != 0 {
            return Err(Box::new(std::io::Error::from_raw_os_error(result)));
        }
        Ok(measure_level)
    }

    pub fn get_var_column_width(&self, var_name: &str) -> Result<i32, Box<dyn Error>> {
        let var_name_c = CString::new(var_name)?;
        let mut column_width: c_int = 0;
        let result = unsafe { ffi::spssGetVarColumnWidth(self.handle, var_name_c.as_ptr(), &mut column_width) };
        if result != 0 {
            return Err(Box::new(std::io::Error::from_raw_os_error(result)));
        }
        Ok(column_width)
    }

    pub fn get_var_alignment(&self, var_name: &str) -> Result<i32, Box<dyn Error>> {
        let var_name_c = CString::new(var_name)?;
        let mut alignment: c_int = 0;
        let result = unsafe { ffi::spssGetVarAlignment(self.handle, var_name_c.as_ptr(), &mut alignment) };
        if result != 0 {
            return Err(Box::new(std::io::Error::from_raw_os_error(result)));
        }
        Ok(alignment)
    }

    pub fn get_var_print_format(&self, var_name: &str) -> Result<(i32, i32, i32), Box<dyn Error>> {
        let var_name_c = CString::new(var_name)?;
        let mut print_type: c_int = 0;
        let mut print_dec: c_int = 0;
        let mut print_width: c_int = 0;
        let result = unsafe { ffi::spssGetVarPrintFormat(self.handle, var_name_c.as_ptr(), &mut print_type, &mut print_dec, &mut print_width) };
        if result != 0 {
            return Err(Box::new(std::io::Error::from_raw_os_error(result)));
        }
        Ok((print_type, print_dec, print_width))
    }

    pub fn get_var_handle(&self, var_name: &str) -> Result<f64, Box<dyn Error>> {
        let var_name_c = CString::new(var_name)?;
        let mut var_handle: f64 = 0.0;
        let result = unsafe { ffi::spssGetVarHandle(self.handle, var_name_c.as_ptr(), &mut var_handle) };
        if result != 0 {
            return Err(Box::new(std::io::Error::from_raw_os_error(result)));
        }
        Ok(var_handle)
    }

    pub fn get_value_numeric(&self, var_handle: f64) -> Result<f64, Box<dyn Error>> {
        let mut value: f64 = 0.0;
        let result = unsafe { ffi::spssGetValueNumeric(self.handle, var_handle, &mut value) };
        if result != 0 {
            return Err(Box::new(std::io::Error::from_raw_os_error(result)));
        }
        Ok(value)
    }

    pub fn get_value_char(&self, var_handle: f64) -> Result<String, Box<dyn Error>> {
        let mut value: [c_char; 256] = [0; 256];
        let result = unsafe { ffi::spssGetValueChar(self.handle, var_handle, value.as_mut_ptr(), 256) };
        if result != 0 {
            return Err(Box::new(std::io::Error::from_raw_os_error(result)));
        }
        let value_str = unsafe { CStr::from_ptr(value.as_ptr()) }.to_str()?.to_string();
        Ok(value_str)
    }

    pub fn read_case_record(&self) -> Result<(), Box<dyn Error>> {
        let result = unsafe { ffi::spssReadCaseRecord(self.handle) };
        if result != 0 {
            return Err(Box::new(std::io::Error::from_raw_os_error(result)));
        }
        Ok(())
    }

    pub fn get_number_of_cases(&self) -> Result<i64, Box<dyn Error>> {
        let mut case_count: c_long = 0;
        let result = unsafe { ffi::spssGetNumberofCases(self.handle, &mut case_count) };
        if result != 0 {
            return Err(Box::new(std::io::Error::from_raw_os_error(result)));
        }
        Ok(case_count)
    }

    pub fn get_id_string(&self) -> Result<String, Box<dyn Error>> {
        let mut id: [c_char; 256] = [0; 256];
        let result = unsafe { ffi::spssGetIdString(self.handle, id.as_mut_ptr()) };
        if result != 0 {
            return Err(Box::new(std::io::Error::from_raw_os_error(result)));
        }
        let id_str = unsafe { CStr::from_ptr(id.as_ptr()) }.to_str()?.to_string();
        Ok(id_str)
    }

    pub fn get_compression(&self) -> Result<i32, Box<dyn Error>> {
        let mut comp_switch: c_int = 0;
        let result = unsafe { ffi::spssGetCompression(self.handle, &mut comp_switch) };
        if result != 0 {
            return Err(Box::new(std::io::Error::from_raw_os_error(result)));
        }
        Ok(comp_switch)
    }

    pub fn get_case_weight_var(&self) -> Result<String, Box<dyn Error>> {
        let mut var_name: [c_char; 256] = [0; 256];
        let result = unsafe { ffi::spssGetCaseWeightVar(self.handle, var_name.as_mut_ptr()) };
        if result != 0 {
            return Err(Box::new(std::io::Error::from_raw_os_error(result)));
        }
        let var_name_str = unsafe { CStr::from_ptr(var_name.as_ptr()) }.to_str()?.to_string();
        Ok(var_name_str)
    }

    pub fn get_variable_sets(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut var_sets: *mut c_char = ptr::null_mut();
        let n_sets: c_int = 0;

        let result = unsafe { ffi::spssGetVariableSets(self.handle, &mut var_sets) };
        if result != 0 {
            return Err(Box::new(std::io::Error::from_raw_os_error(result)));
        }

        let mut sets = Vec::new();
        for i in 0..n_sets {
            let set_ptr = unsafe { *var_sets.offset(i as isize) };
            let set = unsafe { CStr::from_ptr(set_ptr as *const c_char) }.to_str()?.to_string();
            sets.push(set);
        }

        Ok(sets)
    }

    pub fn get_var_role(&self, var_name: &str) -> Result<i32, Box<dyn Error>> {
        let var_name_c = CString::new(var_name)?;
        let mut role: c_int = 0;
        let result = unsafe { ffi::spssGetVarRole(self.handle, var_name_c.as_ptr(), &mut role) };
        if result != 0 {
            return Err(Box::new(std::io::Error::from_raw_os_error(result)));
        }
        Ok(role)
    }

    pub fn get_mult_resp_count(&self) -> Result<i32, Box<dyn Error>> {
        let mut n_sets: c_int = 0;
        let result = unsafe { ffi::spssGetMultRespCount(self.handle, &mut n_sets) };
        if result != 0 {
            return Err(Box::new(std::io::Error::from_raw_os_error(result)));
        }
        Ok(n_sets)
    }

    pub fn get_mult_resp_def_by_index(&self, index: i32) -> Result<String, Box<dyn Error>> {
        let mut pp_set: *mut ffi::spssMultRespDef_T = ptr::null_mut();
        let result = unsafe { ffi::spssGetMultRespDefByIndex(self.handle, index, &mut pp_set) };
        if result != 0 {
            return Err(Box::new(std::io::Error::from_raw_os_error(result)));
        }
        let set = unsafe { CStr::from_ptr(pp_set as *const c_char) }.to_str()?.to_string();
        Ok(set)
    }

    pub fn get_mult_resp_defs(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut mresp_defs: *mut c_char = ptr::null_mut();

        let result = unsafe { ffi::spssGetMultRespDefs(self.handle, &mut mresp_defs) };
        if result != 0 {
            return Err(Box::new(std::io::Error::from_raw_os_error(result)));
        }

        let mut defs = Vec::new();
        let mut current = mresp_defs;

        while !current.is_null() {
            let c_str = unsafe { CStr::from_ptr(current) };
            let str_slice = c_str.to_str().unwrap();
            defs.push(str_slice.to_string());

            unsafe {
                current = current.add(c_str.to_bytes().len() + 1);
            }
        }

        Ok(defs)
    }

    pub fn get_mult_resp_defs_ex(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut mresp_defs: *mut c_char = ptr::null_mut();

        let result = unsafe { ffi::spssGetMultRespDefsEx(self.handle, &mut mresp_defs) };
        if result != 0 {
            return Err(Box::new(std::io::Error::from_raw_os_error(result)));
        }

        let mut defs = Vec::new();
        let mut current = mresp_defs;

        while !current.is_null() {
            let c_str = unsafe { CStr::from_ptr(current) };
            let str_slice = c_str.to_str().unwrap();
            defs.push(str_slice.to_string());

            unsafe {
                current = current.add(c_str.to_bytes().len() + 1);
            }
        }

        Ok(defs)
    }
    pub fn open_write(file_name: &str) -> Result<Self, SpssioErrorCode> {
        let file_name_c = CString::new(file_name).unwrap();
        let mut handle: c_int = 0;

        let result = unsafe { ffi::spssOpenWrite(file_name_c.as_ptr(), &mut handle) };
        if result != 0 {
            return Err(SpssioErrorCode::from(result));
        }

        Ok(SPSSFile { handle })
    }

    pub fn close_write(&self) -> Result<(), SpssioErrorCode> {
        let result = unsafe { ffi::spssCloseWrite(self.handle) };
        if result != 0 {
            return Err(SpssioErrorCode::from(result));
        }
        Ok(())
    }

    pub fn set_var_name(&self, var_name: &str, var_type: c_int) -> Result<(), SpssioErrorCode> {
        let var_name_c = CString::new(var_name).unwrap();
        let result = unsafe { ffi::spssSetVarName(self.handle, var_name_c.as_ptr(), var_type) };
        if result != 0 {
            return Err(SpssioErrorCode::from(result));
        }
        Ok(())
    }

    pub fn set_var_label(&self, var_name: &str, var_label: &str) -> Result<(), SpssioErrorCode> {
        let var_name_c = CString::new(var_name).unwrap();
        let var_label_c = CString::new(var_label).unwrap();
        let result = unsafe { ffi::spssSetVarLabel(self.handle, var_name_c.as_ptr(), var_label_c.as_ptr()) };
        if result != 0 {
            return Err(SpssioErrorCode::from(result));
        }
        Ok(())
    }

    pub fn set_value_numeric(&self, var_name: &str, value: f64) -> Result<(), SpssioErrorCode> {
        let var_name_c = CString::new(var_name).unwrap();
        let mut var_handle: f64 = 0.0;
        let result = unsafe { ffi::spssGetVarHandle(self.handle, var_name_c.as_ptr(), &mut var_handle) };
        if result != 0 {
            return Err(SpssioErrorCode::from(result));
        }
        let result = unsafe { ffi::spssSetValueNumeric(self.handle, var_handle, value) };
        if result != 0 {
            return Err(SpssioErrorCode::from(result));
        }
        Ok(())
    }

    pub fn set_value_char(&self, var_name: &str, value: &str) -> Result<(), SpssioErrorCode> {
        let var_name_c = CString::new(var_name).unwrap();
        let value_c = CString::new(value).unwrap();
        let mut var_handle: f64 = 0.0;
        let result = unsafe { ffi::spssGetVarHandle(self.handle, var_name_c.as_ptr(), &mut var_handle) };
        if result != 0 {
            return Err(SpssioErrorCode::from(result));
        }

       // let value_c = CString::new("test");
        let result = unsafe { ffi::spssSetValueChar(self.handle, var_handle, value_c.as_ptr()) };
        if result != 0 {
            return Err(SpssioErrorCode::from(result));
        }
        Ok(())
    }

    pub fn commit_header(&self) -> Result<(), SpssioErrorCode> {
        let result = unsafe { ffi::spssCommitHeader(self.handle) };
        if result != 0 {
            return Err(SpssioErrorCode::from(result));
        }
        Ok(())
    }

    pub fn commit_case_record(&self) -> Result<(), SpssioErrorCode> {
        let result = unsafe { ffi::spssCommitCaseRecord(self.handle) };
        if result != 0 {
            return Err(SpssioErrorCode::from(result));
        }
        Ok(())
    }

    pub fn get_time_stamp(&self) -> Result<(String, String), SpssioErrorCode> {
        let mut file_date: [c_char; 9] = [0; 9];
        let mut file_time: [c_char; 9] = [0; 9];
        let result = unsafe { ffi::spssGetTimeStamp(self.handle, file_date.as_mut_ptr(), file_time.as_mut_ptr()) };
        if result != 0 {
            return Err(SpssioErrorCode::from(result));
        }
        let file_date_str = unsafe { CStr::from_ptr(file_date.as_ptr()) }.to_str().unwrap().to_string();
        let file_time_str = unsafe { CStr::from_ptr(file_time.as_ptr()) }.to_str().unwrap().to_string();
        Ok((file_date_str, file_time_str))
    }

    pub fn set_compression(&self, comp_switch: i32) -> Result<(), SpssioErrorCode> {
        let result = unsafe { ffi::spssSetCompression(self.handle, comp_switch) };
        if result != 0 {
            return Err(SpssioErrorCode::from(result));
        }
        Ok(())
    }

    pub fn set_var_c_value_label(&self, var_name: &str, value: &str, label: &str) -> Result<(), SpssioErrorCode> {
        let var_name_c = CString::new(var_name).unwrap();
        let value_c = CString::new(value).unwrap();
        let label_c = CString::new(label).unwrap();
        let result = unsafe { ffi::spssSetVarCValueLabel(self.handle, var_name_c.as_ptr(), value_c.as_ptr(), label_c.as_ptr()) };
        if result != 0 {
            return Err(SpssioErrorCode::from(result));
        }
        Ok(())
    }

    pub fn get_var_n_value_label(&self, var_name: &str, value: f64) -> Result<String, SpssioErrorCode> {
        let var_name_c = CString::new(var_name).unwrap();
        let mut label: [c_char; 256] = [0; 256];
        let result = unsafe { ffi::spssGetVarNValueLabel(self.handle, var_name_c.as_ptr(), value, label.as_mut_ptr()) };
        if result != 0 {
            return Err(SpssioErrorCode::from(result));
        }
        let label_str = unsafe { CStr::from_ptr(label.as_ptr()) }.to_str().unwrap().to_string();
        Ok(label_str)
    }

    pub fn get_var_n_value_label_long(&self, var_name: &str, value: f64) -> Result<String, SpssioErrorCode> {
        let var_name_c = CString::new(var_name).unwrap();
        let mut label: [c_char; 256] = [0; 256];
        let mut len_label: c_int = 0;
        let result = unsafe { ffi::spssGetVarNValueLabelLong(self.handle, var_name_c.as_ptr(), value, label.as_mut_ptr(), 256, &mut len_label) };
        if result != 0 {
            return Err(SpssioErrorCode::from(result));
        }
        let label_str = unsafe { CStr::from_ptr(label.as_ptr()) }.to_str().unwrap().to_string();
        Ok(label_str)
    }

    pub fn get_var_n_missing_values(&self, var_name: &str) -> Result<(i32, f64, f64, f64), SpssioErrorCode> {
        let var_name_c = CString::new(var_name).unwrap();
        let mut missing_format: c_int = 0;
        let mut missing_val1: f64 = 0.0;
        let mut missing_val2: f64 = 0.0;
        let mut missing_val3: f64 = 0.0;
        let result = unsafe { ffi::spssGetVarNMissingValues(self.handle, var_name_c.as_ptr(), &mut missing_format, &mut missing_val1, &mut missing_val2, &mut missing_val3) };
        if result != 0 {
            return Err(SpssioErrorCode::from(result));
        }
        Ok((missing_format, missing_val1, missing_val2, missing_val3))
    }
}
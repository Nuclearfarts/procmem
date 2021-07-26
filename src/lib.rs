use process_memory::*;
use std::slice::{from_raw_parts, from_raw_parts_mut};

#[no_mangle]
pub unsafe extern "C" fn write_mem(pid: i64, offset: usize, buf_ptr: *const u8, buf_size: usize) -> i32 {
	let pid = pid as Pid;
	let handle_r = pid.try_into_process_handle();
	if let Ok(handle) = handle_r {
		let buf = from_raw_parts(buf_ptr, buf_size);
		if handle.put_address(offset, buf).is_err() {
			return -2;
		}
	} else {
		return -1;
	}
	return 0;
}

#[no_mangle]
pub unsafe extern "C" fn read_mem(pid: i64, offset: usize, buf_ptr: *mut u8, buf_size: usize) -> i32 {
	let pid = pid as Pid;
	let handle_r = pid.try_into_process_handle();
	if let Ok(handle) = handle_r {
		let buf = from_raw_parts_mut(buf_ptr, buf_size);
		if handle.copy_address(offset, buf).is_err() {
			return -2;
		}
	} else {
		return -1;
	}
	return 0;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

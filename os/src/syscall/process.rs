//! Process management syscalls
use core::{cmp::min, mem::size_of};

use crate::{
    mm::translated_byte_buffer,
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next, get_current_trace_time,
        suspend_current_and_run_next,
    },
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    // get the time as usize
    let us = get_time_us();
    // build timeval
    let ts = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    let length = size_of::<TimeVal>();
    // reinterpret the ts to a byte array
    let ts_bytes =
        unsafe { core::slice::from_raw_parts(&ts as *const TimeVal as *const u8, length) };
    // get buffer in the user space
    let buffers = translated_byte_buffer(current_user_token(), _tz as *const u8, length);
    let mut u8_copied = 0;
    // copy data to user space
    for buffer in buffers {
        // calculate byte to copy this page
        let len_to_copy = min(ts_bytes.len() - u8_copied, buffer.len());
        buffer[..len_to_copy].copy_from_slice(&ts_bytes[u8_copied..u8_copied + len_to_copy]);
        u8_copied += len_to_copy;
    }
    // not all data copied, maybe the address is invalid
    if u8_copied != ts_bytes.len() {
        trace!("kernel: sys_get_time: invalid address");
        -1
    } else {
        0
    }
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    match _trace_request {
        0 => {
            // read memory from user space
            let buffer =
                translated_byte_buffer(current_user_token(), _id as *const u8, size_of::<u8>());
            // return the first byte of the buffer
            // each [u8] in buffer should have at least one byte
            if !buffer.is_empty() {
                buffer[0][0] as isize
            // buffer is empty, maybe the address is invalid
            } else {
                trace!("kernel: sys_trace: invalid address");
                -1
            }
        }
        1 => {
            // get access of memory in user space
            let src =
                translated_byte_buffer(current_user_token(), _data as *const u8, size_of::<u8>());
            let mut dst =
                translated_byte_buffer(current_user_token(), _id as *const u8, size_of::<u8>());
            // copy data from src to dst
            // each [u8] in buffer should have at least one byte
            if !src.is_empty() && !dst.is_empty() {
                dst[0][0] = src[0][0];
                0
            // buffer is empty, maybe the address is invalid
            } else {
                trace!("kernel: sys_trace: invalid address");
                -1
            }
        }
        2 => get_current_trace_time(_id),
        _ => -1,
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    -1
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    -1
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}

//! Syscall IDs for the kernel

/// write syscall
pub const SYSCALL_WRITE: usize = 64;
/// exit syscall
pub const SYSCALL_EXIT: usize = 93;
/// yield syscall
pub const SYSCALL_YIELD: usize = 124;
/// gettime syscall
pub const SYSCALL_GET_TIME: usize = 169;
/// trace syscall
pub const SYSCALL_TRACE: usize = 410;
/// syscall count
pub const SYSCALL_COUNT: usize = 5;

/// map syscall id to array index
pub fn find_syscall_number(id: usize) -> usize {
    match id {
        SYSCALL_WRITE => 0,
        SYSCALL_EXIT => 1,
        SYSCALL_YIELD => 2,
        SYSCALL_GET_TIME => 3,
        SYSCALL_TRACE => 4,
        _ => panic!("Unsupported syscall id: {}", id),
    }
}

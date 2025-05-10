//! Syscall IDs for the kernel

/// write syscall
pub const SYSCALL_WRITE: usize = 64;
/// exit syscall
pub const SYSCALL_EXIT: usize = 93;
/// yield syscall
pub const SYSCALL_YIELD: usize = 124;
/// gettime syscall
pub const SYSCALL_GET_TIME: usize = 169;
/// sbrk syscall
pub const SYSCALL_SBRK: usize = 214;
/// munmap syscall
pub const SYSCALL_MUNMAP: usize = 215;
/// mmap syscall
pub const SYSCALL_MMAP: usize = 222;
/// trace syscall
pub const SYSCALL_TRACE: usize = 410;

/// syscall count
pub const SYSCALL_COUNT: usize = 8;
/// map syscall id to array index
pub fn find_syscall_number(id: usize) -> usize {
    match id {
        SYSCALL_WRITE => 0,
        SYSCALL_EXIT => 1,
        SYSCALL_YIELD => 2,
        SYSCALL_GET_TIME => 3,
        SYSCALL_SBRK => 4,
        SYSCALL_MUNMAP => 5,
        SYSCALL_MMAP => 6,
        SYSCALL_TRACE => 7,
        _ => panic!("Invalid syscall id"),
    }
}

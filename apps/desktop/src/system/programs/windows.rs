//! The Windows side: one system call lists the processes.

use super::Process;

/// Windows writes the process list in `buffer`. The buffer keeps the size that the previous list
/// needed.
#[derive(Default)]
pub(super) struct Lister {
    buffer: Vec<u64>,
}

impl Lister {
    pub(super) fn processes(&mut self) -> Vec<Process> {
        processes(&mut self.buffer)
    }
}

/// All processes with their CPU time, from one system call. The call needs no access to the
/// processes, so it includes services and elevated processes, as Task Manager does.
fn processes(buffer: &mut Vec<u64>) -> Vec<Process> {
    use windows::Wdk::System::SystemInformation::{
        NtQuerySystemInformation, SystemProcessInformation,
    };
    use windows::Win32::Foundation::{STATUS_INFO_LENGTH_MISMATCH, UNICODE_STRING};

    /// The start of each entry. `SYSTEM_PROCESS_INFORMATION` in the SDK keeps the times in a
    /// reserved block. This is the layout that all process tools use.
    #[repr(C)]
    struct Entry {
        next_entry_offset: u32,
        number_of_threads: u32,
        working_set_private_size: i64,
        hard_fault_count: u32,
        number_of_threads_high_watermark: u32,
        cycle_time: u64,
        create_time: i64,
        user_time: i64,
        kernel_time: i64,
        image_name: UNICODE_STRING,
        base_priority: i32,
        unique_process_id: usize,
    }

    // The buffer holds u64 values for the 8-byte alignment of the entries. It grows untill the
    // list fits, with space for processes that start before the next read.
    loop {
        let mut needed = 0;
        let status = unsafe {
            NtQuerySystemInformation(
                SystemProcessInformation,
                buffer.as_mut_ptr().cast(),
                size_of_val(buffer.as_slice()) as u32,
                &mut needed,
            )
        };
        if status == STATUS_INFO_LENGTH_MISMATCH {
            buffer.resize(needed as usize / 8 * 5 / 4, 0);
        } else if status.is_ok() {
            break;
        } else {
            return Vec::new();
        }
    }

    let mut processes = Vec::new();
    let mut offset = 0;
    loop {
        let entry = unsafe { &*buffer.as_ptr().cast::<u8>().add(offset).cast::<Entry>() };
        // Id 0 is the idle process. Its CPU time is the time when the processors were idle.
        if entry.unique_process_id != 0 {
            let UNICODE_STRING { Length, Buffer, .. } = entry.image_name;
            let name = unsafe { std::slice::from_raw_parts(Buffer.0, Length as usize / 2) };
            processes.push(Process {
                key: (entry.unique_process_id, entry.create_time),
                name: String::from_utf16_lossy(name),
                time: (entry.user_time + entry.kernel_time) as u64,
            });
        }
        if entry.next_entry_offset == 0 {
            return processes;
        }
        offset += entry.next_entry_offset as usize;
    }
}

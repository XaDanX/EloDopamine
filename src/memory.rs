
use winapi::{
    shared::minwindef::DWORD,
    ctypes::c_void,
    um::{
        winuser::{
            FindWindowA,
            GetWindowThreadProcessId },
        processthreadsapi::OpenProcess,
        memoryapi::{
            ReadProcessMemory
        },
        tlhelp32::{
            MODULEENTRY32,
            CreateToolhelp32Snapshot,
            Module32First,
            Module32Next }
    }
};
use std::{
    ptr::null_mut as nullptr,
    ffi::CString
};
use std::os::raw::c_longlong;
use std::os::windows::raw::HANDLE;
use winapi::shared::minwindef::LPVOID;
use winapi::shared::windef::HWND;

#[derive(Clone, Copy)]
pub struct Memory {
    pub pid: DWORD,
    pub base_address: DWORD,
    pub handle: HANDLE
}
impl Memory {
    pub fn new(title: &str) -> Memory {
        unsafe {
            let pid = find_window(title);

            return if pid != 0 {
                let handle = OpenProcess(0x1f0ff as DWORD, 0, pid);
                let base_address = find_module(pid, "");
                Memory { pid, base_address, handle: handle as HANDLE }
            } else {
                Memory { pid: 0, base_address: 0, handle: 0 as HANDLE }
            }
        }

    }

    pub fn read<T: Default>(&mut self, address: u32) -> T {
        let mut ret: T = Default::default();

        unsafe {
            ReadProcessMemory(
                self.handle as _,
                address as *const c_void,
                &mut ret as *mut T as *mut c_void,
                std::mem::size_of::<T>(),
                nullptr()
            );}

        return ret;
    }

    pub fn read_t<T: Default>(&mut self, address: DWORD) -> T {
        let mut ret: T = Default::default();

        unsafe {
            ReadProcessMemory(
                self.handle as _,
                address as *const c_void,
                &mut ret as *mut T as *mut c_void,
                std::mem::size_of::<T>(),
                nullptr()
            );}

        return ret;
    }

    pub fn read_i<T: Default>(&mut self, address: i32) -> T {
        let mut ret: T = Default::default();

        unsafe {
            ReadProcessMemory(
                self.handle as _,
                address as *const c_void,
                &mut ret as *mut T as *mut c_void,
                std::mem::size_of::<T>(),
                nullptr()
            );}

        return ret;
    }

    pub fn read_string(&mut self, address: u32) -> String {
        let string_address = self.read::<u32>(address);
        let string_size = self.read::<u32>(address + 4);


        let buff: Vec<u8> = vec![0; string_size as usize];
        unsafe {
            ReadProcessMemory(
                self.handle as _,
                string_address as LPVOID,
                buff.as_ptr() as LPVOID,
                string_size as _,
                nullptr()
            );
        }
        return String::from_utf8(buff).expect("Found invalid UTF-8").parse().unwrap();
    }

    pub fn read_template(&mut self, address: u32) -> Vec<u32> {
        let template_address = self.read::<u32>(self.base_address + address);
        let template_array = self.read::<u32>(template_address + 0x4);
        let template_array_length = self.read::<u32>(template_address + 0x08);
        let mut object_pointer_list: Vec<u32> = vec![0; template_array_length as _];
        object_pointer_list.clear();
        for index in 0..template_array_length {
            let object_pointer: u32 = self.read::<u32>(template_array + (index*4));
            object_pointer_list.push(object_pointer);
        }
        return object_pointer_list
    }
    
}

impl std::fmt::Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Memory:\n\t-Pid: {}\n\t-BaseAddress: {:#08x}", self.pid, self.base_address)
    }
}

fn find_window(title: &str) -> u32 {
    let _title = CString::new(title).unwrap();
    let wnd_name = _title.as_ptr();

    unsafe {
        let h_wnd = FindWindowA(nullptr(), wnd_name);

        let mut pid: u32 = 0;
        GetWindowThreadProcessId(h_wnd, &mut pid);
        return pid
    }

}

pub fn get_hwnd(title: &str) -> HWND{
    let _title = CString::new(title).unwrap();
    let wnd_name = _title.as_ptr();

    unsafe {
        let h_wnd = FindWindowA(nullptr(), wnd_name);
        return h_wnd;
    }
}

fn find_module(pid: DWORD, name: &str) -> DWORD {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(0x8 as DWORD, pid);

        let mut module = MODULEENTRY32 {
            dwSize: std::mem::size_of::<MODULEENTRY32>() as u32,
            th32ModuleID: 0,
            th32ProcessID: 0,
            GlblcntUsage: 0,
            ProccntUsage: 0,
            modBaseAddr: nullptr(),
            modBaseSize: 0,
            hModule: nullptr(),
            szModule: [0; 256],
            szExePath: [0; 260]
        };

        Module32First(snapshot, &mut module);

        loop {
            let _u8slice = &*(&mut module.szModule[..] as *mut [i8] as *mut [u8]);
            let module_name = std::str::from_utf8(_u8slice).unwrap();

            if module_name.find(name) == Some(0) {
                return module.modBaseAddr as DWORD;
            }
            if Module32Next(snapshot, &mut module) == 0 {
                return 0 as DWORD;
            }
        }
    }
}
use alloc::collections::BTreeMap;
use alloc::sync::Arc;

use spin::Mutex;

fn init_prgrams() -> BTreeMap<&'static str, &'static [u8]> {
    extern "C" {
        fn app_initproc_start();
        fn app_initproc_end();
        fn app_hello_world_start();
        fn app_hello_world_end();
        fn app_hello_world_a_start();
        fn app_hello_world_a_end();
        fn app_hello_world_b_start();
        fn app_hello_world_b_end();
    }

    let intiproc_data: &'static [u8] = unsafe {
        core::slice::from_raw_parts(app_initproc_start as usize as *const u8, app_initproc_end as usize - app_initproc_start as usize)
    };
    let hello_world_data: &'static [u8] = unsafe {
        core::slice::from_raw_parts(app_hello_world_start as usize as *const u8, app_hello_world_end as usize - app_hello_world_start as usize)
    };
    let hello_world_a_data: &'static [u8] = unsafe {
        core::slice::from_raw_parts(app_hello_world_a_start as usize as *const u8, app_hello_world_a_end as usize - app_hello_world_a_start as usize)
    };
    let hello_world_b_data: &'static [u8] = unsafe {
        core::slice::from_raw_parts(app_hello_world_b_start as usize as *const u8, app_hello_world_b_end as usize - app_hello_world_b_start as usize)
    };

    let mut programs = BTreeMap::new();
    programs.insert("initproc", intiproc_data);
    programs.insert("hello_word", intiproc_data);
    programs.insert("hello_word_a", intiproc_data);
    programs.insert("hello_word_b", intiproc_data);
    programs
}

lazy_static! {
    pub static ref PROGRAMS: Arc<Mutex<BTreeMap<&'static str, &'static [u8]>>> = {
        let programs = init_prgrams();
        Arc::new(Mutex::new(programs))
    };
}


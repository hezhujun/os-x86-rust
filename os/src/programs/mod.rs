use alloc::collections::BTreeMap;
use alloc::sync::Arc;

use spin::Mutex;

fn init_prgrams() -> BTreeMap<&'static str, &'static [u8]> {
    extern "C" {
        fn app_initproc_start();
        fn app_initproc_end();
        fn app_user_shell_start();
        fn app_user_shell_end();
        fn app_help_start();
        fn app_help_end();
        fn app_hello_world_start();
        fn app_hello_world_end();
        fn app_hello_world_a_start();
        fn app_hello_world_a_end();
        fn app_forktest_simple_start();
        fn app_forktest_simple_end();
        fn app_forktest_start();
        fn app_forktest_end();
        fn app_forktest2_start();
        fn app_forktest2_end();
        fn app_threads_start();
        fn app_threads_end();
        fn app_threads_arg_start();
        fn app_threads_arg_end();
    }

    let intiproc_data: &'static [u8] = unsafe {
        core::slice::from_raw_parts(app_initproc_start as usize as *const u8, app_initproc_end as usize - app_initproc_start as usize)
    };
    let user_shell_data: &'static [u8] = unsafe {
        core::slice::from_raw_parts(app_user_shell_start as usize as *const u8, app_user_shell_end as usize - app_user_shell_start as usize)
    };
    let help_data: &'static [u8] = unsafe {
        core::slice::from_raw_parts(app_help_start as usize as *const u8, app_help_end as usize - app_help_start as usize)
    };
    let hello_world_data: &'static [u8] = unsafe {
        core::slice::from_raw_parts(app_hello_world_start as usize as *const u8, app_hello_world_end as usize - app_hello_world_start as usize)
    };
    let hello_world_a_data: &'static [u8] = unsafe {
        core::slice::from_raw_parts(app_hello_world_a_start as usize as *const u8, app_hello_world_a_end as usize - app_hello_world_a_start as usize)
    };
    let forktest_simple_data: &'static [u8] = unsafe {
        core::slice::from_raw_parts(app_forktest_simple_start as usize as *const u8, app_forktest_simple_end as usize - app_forktest_simple_start as usize)
    };
    let forktest_data: &'static [u8] = unsafe {
        core::slice::from_raw_parts(app_forktest_start as usize as *const u8, app_forktest_end as usize - app_forktest_start as usize)
    };
    let forktest2_data: &'static [u8] = unsafe {
        core::slice::from_raw_parts(app_forktest2_start as usize as *const u8, app_forktest2_end as usize - app_forktest2_start as usize)
    };
    let threads_data: &'static [u8] = unsafe {
        core::slice::from_raw_parts(app_threads_start as usize as *const u8, app_threads_end as usize - app_threads_start as usize)
    };
    let threads_arg_data: &'static [u8] = unsafe {
        core::slice::from_raw_parts(app_threads_arg_start as usize as *const u8, app_threads_arg_end as usize - app_threads_arg_start as usize)
    };

    let mut programs = BTreeMap::new();
    programs.insert("initproc", intiproc_data);
    programs.insert("user_shell", user_shell_data);
    programs.insert("help", help_data);
    programs.insert("hello_world", hello_world_data);
    programs.insert("hello_world_a", hello_world_a_data);
    programs.insert("forktest_simple", forktest_simple_data);
    programs.insert("forktest", forktest_data);
    programs.insert("forktest2", forktest2_data);
    programs.insert("threads", threads_data);
    programs.insert("threads_arg", threads_arg_data);
    programs
}

lazy_static! {
    pub static ref PROGRAMS: Arc<Mutex<BTreeMap<&'static str, &'static [u8]>>> = {
        let programs = init_prgrams();
        Arc::new(Mutex::new(programs))
    };
}


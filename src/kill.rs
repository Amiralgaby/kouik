use libc::{kill,EINVAL,EPERM,ESRCH};
use libc;
use crate::lib::Proc;

pub fn kill_proc_by_name(progname: &str, liste_procs: &Vec<Proc>) -> Result<u32,String> {

    let mut compteur : u32 = 0;
    let mut error_desc_trace : String = String::new();
	
    for process in liste_procs {
        if process.name == *progname {
			match kill_proc(process) {
                None => compteur += 1,
                Some(error_description) => {
                    error_desc_trace.push_str("process : ");
                    error_desc_trace.push_str(&process.pid.to_string());
                    error_desc_trace.push(' ');
                    error_desc_trace.push_str(&error_description);
                    error_desc_trace.push('\n');
                },
            }
        }
    }

    if !error_desc_trace.is_empty() {
        return Err(error_desc_trace);
    }
    Ok(compteur)
}

pub fn kill_proc(processus: &Proc) -> Option<String> {
    let result_of_kill : i32  = unsafe { kill(processus.pid,libc::SIGTERM) };
    
    if result_of_kill == 0 {
        return None;
    }

    let errno = unsafe {
        libc::__errno_location() as i32
    };

    let mystr = match errno {
        EINVAL => "An invalid signal was specified.",
        EPERM => "The calling process does not have permission to send the
              signal to any of the target processes.",
        ESRCH => "The target process or process group does not exist.  Note
              that an existing process might be a zombie, a process that
              has terminated execution, but has not yet been waited for.",
        _ => "Unimplemented error",
    };

    Some(mystr.to_string())
}


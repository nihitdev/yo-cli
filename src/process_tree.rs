//! Platform process containment, with ownership of the child until it is reaped.
use std::{
    io,
    process::{Child, Command, ExitStatus},
};

pub struct ProcessTree {
    child: Child,
    #[cfg(unix)]
    group: libc::pid_t,
    #[cfg(windows)]
    job: windows::Handle,
    finished: bool,
}

impl ProcessTree {
    pub fn spawn(command: &mut Command) -> io::Result<Self> {
        #[cfg(target_os = "linux")]
        enable_subreaper()?;
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            command.process_group(0);
        }
        #[cfg(windows)]
        let job = {
            use std::os::windows::process::CommandExt;
            command.creation_flags(windows_sys::Win32::System::Threading::CREATE_SUSPENDED);
            windows::new_job()?
        };
        let child = command.spawn()?;
        let tree = Self {
            #[cfg(unix)]
            group: child.id() as libc::pid_t,
            child,
            #[cfg(windows)]
            job,
            finished: false,
        };
        #[cfg(windows)]
        {
            // The child cannot run/spawn descendants before assignment. On every
            // setup failure, the tree's Drop kills and reaps the suspended child.
            windows::assign_and_resume(&tree.job, &tree.child)?;
        }
        Ok(tree)
    }

    pub fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
        #[cfg(unix)]
        {
            use std::os::unix::process::ExitStatusExt;
            // Observe without reaping: the leader's PID cannot be reused before
            // we signal its process group, even if it has already exited.
            let mut info: libc::siginfo_t = unsafe { std::mem::zeroed() };
            let result = unsafe {
                libc::waitid(
                    libc::P_PID,
                    self.child.id() as libc::id_t,
                    &mut info,
                    libc::WEXITED | libc::WNOHANG | libc::WNOWAIT,
                )
            };
            if result < 0 {
                return Err(io::Error::last_os_error());
            }
            if unsafe { info.si_pid() } == 0 {
                return Ok(None);
            }
            let code = unsafe { info.si_status() };
            let raw = if info.si_code == libc::CLD_EXITED {
                code << 8
            } else {
                code | if info.si_code == libc::CLD_DUMPED {
                    0x80
                } else {
                    0
                }
            };
            Ok(Some(ExitStatus::from_raw(raw)))
        }
        #[cfg(not(unix))]
        self.child.try_wait()
    }

    pub fn finish(&mut self) -> io::Result<()> {
        if self.finished {
            return Ok(());
        }
        #[cfg(unix)]
        let kill = {
            // The group is private to this invocation, including inherited descendants.
            let result = unsafe { libc::kill(-self.group, libc::SIGKILL) };
            let error = io::Error::last_os_error();
            if result == 0 || error.raw_os_error() == Some(libc::ESRCH) {
                Ok(())
            } else {
                Err(error)
            }
        };
        #[cfg(windows)]
        let kill = windows::terminate(&self.job);
        #[cfg(not(any(unix, windows)))]
        let kill = self.child.kill();
        // Also handles a failure to assign the child to its Windows job.
        let _ = self.child.kill();
        let wait = self.child.wait();
        self.finished = wait.is_ok();
        #[cfg(target_os = "linux")]
        let descendants = reap_group(self.group);
        #[cfg(not(target_os = "linux"))]
        let descendants: io::Result<()> = Ok(());
        kill.and(wait.map(|_| ())).and(descendants)
    }
}

impl Drop for ProcessTree {
    fn drop(&mut self) {
        let _ = self.finish();
    }
}

#[cfg(windows)]
mod windows {
    use std::{
        io,
        mem::{size_of, zeroed},
        os::windows::io::AsRawHandle,
        process::Child,
        ptr,
    };
    use windows_sys::Win32::{
        Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE},
        System::{
            Diagnostics::ToolHelp::{
                CreateToolhelp32Snapshot, TH32CS_SNAPTHREAD, THREADENTRY32, Thread32First,
                Thread32Next,
            },
            JobObjects::{
                AssignProcessToJobObject, CreateJobObjectW, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
                JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JobObjectExtendedLimitInformation,
                SetInformationJobObject, TerminateJobObject,
            },
            Threading::{OpenThread, ResumeThread, THREAD_SUSPEND_RESUME},
        },
    };

    pub struct Handle(HANDLE);
    impl Handle {
        fn new(raw: HANDLE) -> io::Result<Self> {
            if raw.is_null() || raw == INVALID_HANDLE_VALUE {
                Err(io::Error::last_os_error())
            } else {
                Ok(Self(raw))
            }
        }
    }
    impl Drop for Handle {
        fn drop(&mut self) {
            unsafe {
                CloseHandle(self.0);
            }
        }
    }

    pub fn new_job() -> io::Result<Handle> {
        let job = Handle::new(unsafe { CreateJobObjectW(ptr::null(), ptr::null()) })?;
        let mut info: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = unsafe { zeroed() };
        info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
        if unsafe {
            SetInformationJobObject(
                job.0,
                JobObjectExtendedLimitInformation,
                &info as *const _ as *const _,
                size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
            )
        } == 0
        {
            return Err(io::Error::last_os_error());
        }
        Ok(job)
    }

    pub fn assign_and_resume(job: &Handle, child: &Child) -> io::Result<()> {
        if unsafe { AssignProcessToJobObject(job.0, child.as_raw_handle()) } == 0 {
            return Err(io::Error::last_os_error());
        }
        let snapshot = Handle::new(unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0) })?;
        let mut entry: THREADENTRY32 = unsafe { zeroed() };
        entry.dwSize = size_of::<THREADENTRY32>() as u32;
        let mut available = unsafe { Thread32First(snapshot.0, &mut entry) };
        while available != 0 {
            if entry.th32OwnerProcessID == child.id() {
                let thread = Handle::new(unsafe {
                    OpenThread(THREAD_SUSPEND_RESUME, 0, entry.th32ThreadID)
                })?;
                if unsafe { ResumeThread(thread.0) } == u32::MAX {
                    return Err(io::Error::last_os_error());
                }
                return Ok(());
            }
            available = unsafe { Thread32Next(snapshot.0, &mut entry) };
        }
        Err(io::Error::other(
            "could not find the suspended child's main thread",
        ))
    }

    pub fn terminate(job: &Handle) -> io::Result<()> {
        if unsafe { TerminateJobObject(job.0, 1) } == 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

// Linux containers do not always have a PID 1 that reaps orphans. Adopt orphaned
// descendants and reap only our private groups. This flag is process-wide and
// intentionally stays enabled for this CLI's lifetime; it is set once before
// any managed child is spawned. Other children (e.g. the editor) are untouched.
#[cfg(target_os = "linux")]
fn enable_subreaper() -> io::Result<()> {
    static RESULT: std::sync::OnceLock<Result<(), i32>> = std::sync::OnceLock::new();
    RESULT
        .get_or_init(|| {
            if unsafe { libc::prctl(libc::PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0) } == 0 {
                Ok(())
            } else {
                Err(io::Error::last_os_error()
                    .raw_os_error()
                    .unwrap_or(libc::EINVAL))
            }
        })
        .as_ref()
        .map(|_| ())
        .map_err(|code| io::Error::from_raw_os_error(*code))
}

#[cfg(target_os = "linux")]
fn reap_group(group: libc::pid_t) -> io::Result<()> {
    loop {
        // SIGKILL was sent to the whole group before this kernel-level reap.
        if unsafe { libc::waitpid(-group, std::ptr::null_mut(), 0) } >= 0 {
            continue;
        }
        let error = io::Error::last_os_error();
        match error.raw_os_error() {
            Some(libc::ECHILD) => return Ok(()),
            Some(libc::EINTR) => continue,
            _ => return Err(error),
        }
    }
}

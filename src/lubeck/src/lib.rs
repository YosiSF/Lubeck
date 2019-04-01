// Copyright (c) 2019 Karl Whitford 
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_camel_case_types)]
#[macro_use]

extern crate libc;
extern crate serde_derive;
extern crate serde_json;
extern crate kubeWatch;
extern crate nix;
extern crate serde_derive;
extern crate serde_json;
extern crate serde;


mod caps;
mod namespace;
mod idmap;
mod chroot;
mod ffi_util;
mod std_api;
mod config;
mod error;
mod pipe;
mod child;
mod callbacks;
mod linux;
mod fds;
mod run;
mod status;
mod wait;
mod stdio;
mod debug;
mod zombies;


pub use error::Error;
pub use status::ExitStatus;
pub use stdio::{Stdio, Fd};
pub use pipe::{PipeReader, PipeWriter};
pub use namespace::{Namespace};
pub use idmap::{UidMap, GidMap};
pub use zombies::{reap_zombies, child_events, ChildEvent};
pub use nix::sys::signal::Signal;
pub use debug::{Style, Printer};
pub use caps::{Capability};

use std::ffi::{CString, OsString};
use std::path::PathBuf;
use std::os::unix::io::RawFd;
use std::collections::{HashMap, HashSet};
use std::mem;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use pipe::PipeHolder;

use libc::{pid_t};

#[cfg(unix)]
#[path = "unix.rs"]

#[cfg(windows)]
#[path = "windows.rs"]

mod imp;

pub use self::imp::public::*;

pub struct Command {
    filename: CString,
    args: Vec<CString>,
    environ: Option<HashMap<OsString, OsString>>
    config: config::
}

pub struct Process {
        #[serde(default, skip_serializing_if = "is_false")]
        pub terminal: bool,
        #[serde(default, skip_serializing_if = "is_default",
                rename = "consoleSize")]
        pub console_size: 
}

pub struct User {
    #[serde(default)]
    pub uid:u32,
    #[serde(default)]
    pub gid: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty",
             rename = "additionalGids")]
    pub_additional_gids: Vec<u32>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub username: String

}

//this converts directly to the correct int
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum LinuxRlimitType {
    RLIMIT_CPU, // CPU time in sec
    RLIMIT_FSIZE, // Maximum filesize
    RLIMIT_DATA, // max data size
    RLIMIT_STACK, // max stack size
    RLIMIT_CORE, // max core file size
    RLIMIT_RSS, // max resident set size
    RLIMIT_NPROC, // max number of processes
    RLIMIT_NOFILE, // max number of open files
    RLIMIT_MEMLOCK, // max locked-in-memory address space
    RLIMIT_AS, // address space limit
    RLIMIT_LOCKS, // maximum file locks held
    RLIMIT_SIGPENDING, // max number of pending signals
    RLIMIT_MSGQUEUE, // maximum bytes in POSIX mqueues
    RLIMIT_NICE, // max nice prio allowed to raise to
    RLIMIT_RTPRIO, // maximum realtime priority
    RLIMIT_RTTIME, // timeout for RT tasks in us
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxRlimit {
    #[serde(rename = "type")]
    pub typ: LinuxRlimitType,
    #[serde(default)]
    pub hard: u64,
    #[serde(default)]
    pub soft: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[repr(u8)]
pub enum LinuxCapabilityType {
    CAP_CHOWN,
    CAP_DAC_OVERRIDE,
    CAP_DAC_READ_SEARCH,
    CAP_FOWNER,
    CAP_FSETID,
    CAP_KILL,
    CAP_SETGID,
    CAP_SETUID,
    CAP_SETPCAP,
    CAP_LINUX_IMMUTABLE,
    CAP_NET_BIND_SERVICE,
    CAP_NET_BROADCAST,
    CAP_NET_ADMIN,
    CAP_NET_RAW,
    CAP_IPC_LOCK,
    CAP_IPC_OWNER,
    CAP_SYS_MODULE,
    CAP_SYS_RAWIO,
    CAP_SYS_CHROOT,
    CAP_SYS_PTRACE,
    CAP_SYS_PACCT,
    CAP_SYS_ADMIN,
    CAP_SYS_BOOT,
    CAP_SYS_NICE,
    CAP_SYS_RESOURCE,
    CAP_SYS_TIME,
    CAP_SYS_TTY_CONFIG,
    CAP_MKNOD,
    CAP_LEASE,
    CAP_AUDIT_WRITE,
    CAP_AUDIT_CONTROL,
    CAP_SETFCAP,
    CAP_MAC_OVERRIDE,
    CAP_MAC_ADMIN,
    CAP_SYSLOG,
    CAP_WAKE_ALARM,
    CAP_BLOCK_SUSPEND,
    CAP_AUDIT_READ,
}

pub struct Platform {
    
}



pub struct FileDesc{
    pub fd: CSocket,
}

impl Drop for FileDest {
    fn drop(&mut self) {
        unsafe{
            close(seld.fd);
        }
    }
}

pub fn send_to(socket: CSocket,
                buffer: &[u8],
                dst: *const SockAddr,
                slen: SockLen)
    -> io::Result<usize> {


    }
                
                
                
                )

pub fn send_to(socket: CSocket,
                bufer: &[u8],
                dst: *const SockAddr,
                slen: SockLen)
                -> io::Result<usize>{

                    let send_len = imp::retry(&mut || unsafe {
                        imp::sendto(
                            socket,
                            buffer.as_ptr() as Buf,
                            buffer.len() as BuffLen,
                            0,
                            dst,
                            slen
                        )
                    });

                    

                }
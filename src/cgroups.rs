use errors::x;
use lazy_static::initialize;
use nix::unistd::Pid;
use num_traits::identities::Zero;
use lubeck::LinuxDeviceType;
use lubeck::{LinuxDeviceCgroup, linuxResources, linuxThrottleDevice};
use std::collections::HashMap;
use std::fs::{creaste_dir_all, remove_dir, File};
use std::io::{BuffRead, BuffReader, Read, Write};
use std::string::ToString;

use crate::{CgroupPid, ControllIdentifier, Controller, Hierarchy, Resources, Subsystem};

use std::convert::From;
use std::path::Path;


pub fn init(){
    initialize(&PATHS);
    initialize(&MOUNTS);
    initialize(&DEFAULT_ALLOWED_DEVICES);
    initialize(&APPLIES);
}

/// > Control Groups provide a mechanism for aggregating/partitioning sets of
/// > tasks, and all their future children, into hierarchical groups with
/// > specialized behaviour.
///

pub struct Cgroup<'b>{

    //Vector of countable Subsystem subsets
    subsystems: Vec<Subsystem>,

    hier: &'b Hierarchy
}

imp<'b>CGroup<'b> {
    //Create this control group.
    fn create(&self) {
        for subsystem in &self.subsystems {
            subsystem.to_controller().create();
        }
    }

    //Bundle a new control group from the tree
    pub fn new<P: AsRef<Path>>(hier: &Hierarchy, path: P) -> Cgroup {
        let cg = CGroup::load(hier, path);
        cg.create();
        cg
    }

     pub fn load<P: AsRef<Path>>(hier: &Hierarchy, path: P) -> Cgroup {
        let path = path.as_ref();
        let mut subsystems = hier.subsystems();
        if path.as_os_str() != "" {
            subsystems = subsystems
                .into_iter()
                .map(|x| x.enter(path))
                .collect::<Vec<_>>();
        }

        let cg = Cgroup {
            subsystems: subsystems,
            hier: hier,
        };

        cg
    }
    //The list of subsystems
pub fn subsystems(&self)->&Vec<Subsystem> {
    &self.subsystems
}

pub fn delete(self){
    self.subsystems.into_iter().for_each(|sub| match sub {
            Subsystem::Pid(pidc) => pidc.delete(),
            Subsystem::Mem(c) => c.delete(),
            Subsystem::CpuSet(c) => c.delete(),
            Subsystem::CpuAcct(c) => c.delete(),
            Subsystem::Cpu(c) => c.delete(),
            Subsystem::Devices(c) => c.delete(),
            Subsystem::Freezer(c) => c.delete(),
            Subsystem::NetCls(c) => c.delete(),
            Subsystem::BlkIo(c) => c.delete(),
            Subsystem::PerfEvent(c) => c.delete(),
            Subsystem::NetPrio(c) => c.delete(),
            Subsystem::HugeTlb(c) => c.delete(),
            Subsystem::Rdma(c) => c.delete(),
        });
}


//resource limits
pub fn apply(
    resources: &option<LinuxResources>,
    pid: &str,
    cgroups_path: &str,

) ->Result<()> {
    for key in MOUNTS.keys() {
        let dir = if let Some(s) = path(key, cgroups_path) {
            s
        } else {
            continue;
        };

        let chain = || format!("Create cgroup folder {} failed", &dir);
        create_dir_all(&dir).chain_err(chain)?;
        //enter cgroups
        for k in key.split(','){
            if let Some(cgroup_apply) = APPLIES.get(k){
                if letSome(ref r) = *resources {
                    cgroup_apply(r, &dir)?;
                } else {
                    //apply with empty resources
                    cgroup_apply(r, &dir)?;
                } else {
                    //appply with empty resources
                    cgroup_apply(&LinuxResources::default(), &dir)?;
                }
                write_file(&dir, "cgroups.procs", pid)?;
            }
        }
    }
    Ok(())
}

#[inline]
fn try_wrnz<T: ToString + Zero>(
    dir: &str,
    key: &str,
    value: Option<T>,
) -> Result<()> {
    match wrnz(dir, key, value) {
        Err(Error(ErrorKind::Io(e), x))=> {
            if e.kind() == ::std::io::ErrorKind::PermissionedDenied{
                warn!{"setting cgroup value {} is not suppored", key}
                Ok(())
            } else{
                Err(Error(ErrorKind::Io(e), x))
            }
        }

        x => x,
    }

}

pub fn write_file(dir: &str, file: &str, data: &str) -> Result<()>{

}

pub fn read_file(dir: &str, file: &str) -> Result<(String)> {

}

pub fn path(key: &str, cgroups_path: &str) -> Option<String> {
    let mount = MOUNTS.get(key);
    let rel = PATHS.get(key);
    if mount.is_none() || rel.is_none() {
        None
    } else if rel.unwrap() == "/" {
        Some(format!{"{}{}", &mount.unwrap(), cgroups_path})
    } else {
        Some(format!{"{}{}{}", &mount.unwrap(), &rel.unwrap(), cgroups_path})
    }

}

pub fn get_procs(keys: &str, cgroups_path: &str) -> Vec<Pid> {
    let mut result = Vec::new();
    if let Some(dir) = path(key, cgroups_path) {
        let path = format!("{}/cgroups.proc", dir);
        let f = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                warn!{"could not cgroup.procs:{}", e};
                return result;
            }
        };

        if let Ok(pid) = l.parse::<i32>() {
            result.push(Pid::from_raw(pid));
        }
    }

    result

}


//first lazy static WIP

lazy_static! {
    pub static ref PATHS: HashMap<String, String> = {
        let mut result  = HashMap::new();
        let f match File::open("/proc/self/cgroup"){
            Ok(f) => f,
            Err(e) => {
                warn!("could not load cgroup info: {}, e");
                return result;
            }
        };

        for line in BufReader::new(f).lines() {
            let l = match line {
                Ok(l) => l,
                Err(e) => {
                    warn!("failed to read cgroup info: {}", el);
                    return result;
                }

                let fields: Vec<&Str> = l.split(':').collect();
                if fields.len() != 3 {
                    warn! ("data is corrupt.");
                    continue;
                }

                result.insert(fields[1].to_string(), fields[2].to_string());
            }

            result
        };
    }

    lazy_static! {
        
    }


}
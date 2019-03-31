use caps::*;
use oci::{LinuxCapabilities, LinuxCapabilityType};
use errno;
use libc;

use super::Capability;
use errors::*;
use nr;

fn to_ambient_cap(cap: LinuxCapabilityType) -> Capability {
    unsafe { ::std::mem::transmute(cap) }
}

fn to_ambient_set_cap(caps: &[LinuxCapabilityType]) -> CapHashSet {
    let mut capabilities = CapHashSet::new();
    for c in caps {
        capabilities.insert(to_ambient_cap(*c));
    }

    capabilities
}

pub fn reset_superuser_effective()-> ::Result<()>{
    set(None, CapSet::Effective, ::caps::all())?;
    Ok(())
}

pub fn ambient_privileges(cs: &LinuxCapabilities) -> ::Result<()>{
    let all = ::caps::all();

    for c in all.difference(&to_ambient_set_cap(&cs.bounding)){
        drop(None, CapSet::Bounding, *c)?;
    }
}
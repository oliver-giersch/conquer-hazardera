#![feature(const_generics)]

mod global;
mod guard;
mod local;

use conquer_reclaim::Reclaim;

use core::sync::atomic::AtomicU64;

#[derive(Eq, PartialEq)]
pub enum RetireStrategy {
    Global,
    Local,
}

pub struct HazEra<const S: RetireStrategy> {
    global_era: AtomicU64,
    hazard_eras: HazardEraList,
    global_state: GlobalRetireState,
}

impl<const S: RetireStrategy> Reclaim for HazEra<S> {
    type Header = ();
    type LocalState = ();

    unsafe fn build_local_state(&self) -> Self::LocalState {
        unimplemented!()
    }
}

struct Header {
    create_era: u64,
    retire_era: u64,
}

#[repr(C)]
struct HeaderNode {
    next: *mut Self,
    header: Header,
}

struct HazardEraList;
struct HazardEra(AtomicU64);

enum GlobalRetireState {
    GlobalRetire(()),
    LocalRetire(()),
}

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

impl RetireStrategy {
    fn global_state(&self) -> GlobalRetireState {
        match self {
            RetireStrategy::Global => GlobalRetireState::GlobalRetire(()),
            RetireStrategy::Local => GlobalRetireState::LocalRetire(()),
        }
    }
}

pub struct HazEra<const S: RetireStrategy> {
    global_era: AtomicU64,
    hazard_eras: HazardEraList,
    global_state: GlobalRetireState,
}

impl<const S: RetireStrategy> Default for HazEra<S> {
    fn default() -> Self {
        todo!()
    }
}

impl Reclaim for HazEra<{ RetireStrategy::Global }> {
    type Header = ();
    type LocalState = ();

    unsafe fn build_local_state(&self) -> Self::LocalState {
        unimplemented!()
    }
}

impl Reclaim for HazEra<{ RetireStrategy::Local }> {
    type Header = ();
    type LocalState = ();

    unsafe fn build_local_state(&self) -> Self::LocalState {
        unimplemented!()
    }
}

struct HazardEraList;
struct HazardEra(AtomicU64);

enum GlobalRetireState {
    GlobalRetire(()),
    LocalRetire(()),
}

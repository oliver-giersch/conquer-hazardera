use conquer_reclaim::{LocalState, Retired};

use crate::global::GlobalRef;
use crate::{HazEra, RetireStrategy};

pub struct LocalRef<'local, 'global, const S: RetireStrategy>(Ref<'local, 'global, S>);

unsafe impl<'local, 'global, const S: RetireStrategy> LocalState for LocalRef<'local, 'global, S> {
    type Guard = ();
    type Reclaimer = HazEra<S>;

    fn build_guard(&self) -> Self::Guard {
        unimplemented!()
    }

    unsafe fn retire_record(&self, retired: Retired<Self::Reclaimer>) {
        unimplemented!()
    }
}

pub struct Local<'global, const S: RetireStrategy> {
    global: GlobalRef<'global, S>,
}

enum Ref<'local, 'global, const S: RetireStrategy> {
    Ref(&'local Local<'global, S>),
    Raw(*const Local<'global, S>),
}

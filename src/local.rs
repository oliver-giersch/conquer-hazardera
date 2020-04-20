use core::marker::PhantomData;

use conquer_reclaim::{LocalState, Reclaim, Retired};

use crate::global::GlobalRef;
use crate::{HazEra, RetireStrategy};

pub struct LocalRef<'local, 'global, R> {
    local: Ref<'local, 'global>,
    _marker: PhantomData<R>,
}

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

pub struct Local<'global> {
    global: GlobalRef<'global>,
}

enum Ref<'local, 'global> {
    Ref(&'local Local<'global>),
    Raw(*const Local<'global>),
}

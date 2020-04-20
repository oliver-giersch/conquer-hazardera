use crate::local::LocalRef;
use crate::{HazardEra, RetireStrategy};

pub struct Guard<'local, 'global, const S: RetireStrategy> {
    era: *const HazardEra,
    local: LocalRef<'local, 'global, S>,
}

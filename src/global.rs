use crate::{HazEra, RetireStrategy};

pub struct GlobalRef<'global, const S: RetireStrategy>(Ref<'global, S>);

enum Ref<'global, const S: RetireStrategy> {
    Ref(&'global HazEra<S>),
    Raw(*const HazEra<S>),
}

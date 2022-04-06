/// # Safety
/// Same invariants as [`stuff::StuffingStrategy]
pub unsafe trait PtrStuffingStrategy: sealed::Sealed {
    fn stuff_ptr(addr: usize) -> usize;
    fn extract_ptr(addr: usize) -> usize;
}

mod sealed {
    pub trait Sealed {}
}

pub struct Identity;

impl sealed::Sealed for Identity {}

unsafe impl PtrStuffingStrategy for Identity {
    fn stuff_ptr(addr: usize) -> usize {
        addr
    }

    fn extract_ptr(addr: usize) -> usize {
        addr
    }
}

pub struct Mask1<const MASK: usize>;

impl<const MASK: usize> sealed::Sealed for Mask1<MASK> {}

unsafe impl<const MASK: usize> PtrStuffingStrategy for Mask1<MASK> {
    fn stuff_ptr(addr: usize) -> usize {
        addr & MASK
    }

    fn extract_ptr(addr: usize) -> usize {
        addr & !MASK
    }
}

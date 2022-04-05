use core::marker::PhantomData;

use stuff::Backend;

pub trait SafeStuffingStrategy<B> {
    type Extra;

    fn stuff_extra(extra: &Self::Extra) -> B;

    fn extract_extra(addr: B) -> Option<Self::Extra>;
}

/// An internal type to convert from the safe to the unsafe trait
pub(crate) struct SafeStrategyAdaptor<S>(PhantomData<S>);

// SAFETY: `extract_extra` panics if an invalid extra is returned
//         `stuff_extra` never drops the extra
//         pointer stuffing is just identity
unsafe impl<S, B> stuff::StuffingStrategy<B> for SafeStrategyAdaptor<S>
where
    S: SafeStuffingStrategy<B>,
{
    type Extra = S::Extra;

    fn is_extra(data: B) -> bool {
        S::extract_extra(data).is_some()
    }

    fn stuff_extra(inner: Self::Extra) -> B {
        let b = S::stuff_extra(&inner);
        core::mem::forget(inner);
        b
    }

    unsafe fn extract_extra(data: B) -> Self::Extra {
        S::extract_extra(data).unwrap()
    }

    fn stuff_ptr(addr: usize) -> B {
        addr
    }

    fn extract_ptr(inner: B) -> usize {
        inner
    }
}

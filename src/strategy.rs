use core::marker::PhantomData;

use crate::ptr::PtrStuffingStrategy;

pub trait SafeStuffingStrategy<B> {
    type Other;

    fn stuff_other(extra: &Self::Other) -> B;

    fn extract_other(addr: B) -> Option<Self::Other>;
}

/// An internal type to convert from the safe to the unsafe trait
pub(crate) struct SafeStrategyAdaptor<S>(PhantomData<S>);

// SAFETY: `extract_extra` panics if an invalid extra is returned
//         `stuff_extra` never drops the extra
//         pointer stuffing is just identity
unsafe impl<S, B> stuff::StuffingStrategy<B> for SafeStrategyAdaptor<S>
where
    S: SafeStuffingStrategy<B>,
    B: TryInto<usize>,
    usize: TryInto<B>,
{
    type Other = S::Other;

    fn is_other(data: B) -> bool {
        S::extract_other(data).is_some()
    }

    fn stuff_other(inner: Self::Other) -> B {
        let b = S::stuff_other(&inner);
        core::mem::forget(inner);
        b
    }

    unsafe fn extract_other(data: B) -> Self::Other {
        S::extract_other(data).unwrap()
    }

    fn stuff_ptr(addr: usize) -> B {
        addr.try_into().unwrap_or_else(|_| panic!())
    }

    fn extract_ptr(inner: B) -> usize {
        inner.try_into().unwrap_or_else(|_| panic!())
    }
}

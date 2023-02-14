use std::{rc::Rc, sync::Arc};

trait PointerFamily {
    type PointerType<T>;
}

struct RcPointer;

impl PointerFamily for RcPointer {
    type PointerType<T> = Rc<T>;
}

struct ArcPointer;

impl PointerFamily for ArcPointer {
    type PointerType<T> = Arc<T>;
}

struct MyDataStructure<T, PointerSel: PointerFamily> {
    data: PointerSel::PointerType<T>,
}

////------------

trait FunctorFamily {
    type Type<T>;
    fn fmap<T, U, F>(value: Self::Type<T>, f: F) -> Self::Type<U>
    where
        F: FnMut(T) -> U;
}

trait ApplicativeFamily: FunctorFamily {
    fn pure<T>(inner: T) -> Self::Type<T>;
    fn apply<T, U, F>(value: Self::Type<T>, f: Self::Type<F>) -> Self::Type<U>
    where
        F: FnMut(T) -> U;
}

trait MonadFamily: ApplicativeFamily {
    fn bind<T, U, F>(value: Self::Type<T>, f: F) -> Self::Type<U>
    where
        F: FnMut(T) -> Self::Type<U>;
}

struct OptionType;

impl FunctorFamily for OptionType {
    type Type<T> = Option<T>;

    fn fmap<T, U, F>(value: Self::Type<T>, f: F) -> Self::Type<U>
    where
        F: FnMut(T) -> U,
    {
        value.map(f)
    }
}

impl ApplicativeFamily for OptionType {
    fn pure<T>(inner: T) -> Self::Type<T> {
        Some(inner)
    }

    fn apply<T, U, F>(value: Self::Type<T>, f: Self::Type<F>) -> Self::Type<U>
    where
        F: FnMut(T) -> U,
    {
        value.zip(f).map(|(v, mut f)| f(v))
    }
}

impl MonadFamily for OptionType {
    fn bind<T, U, F>(value: Self::Type<T>, f: F) -> Self::Type<U>
    where
        F: FnMut(T) -> Self::Type<U>,
    {
        value.and_then(f)
    }
}
////------------

fn main() {}

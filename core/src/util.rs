// Code from: https://stackoverflow.com/questions/57454887/how-do-i-append-to-a-tuple

pub trait TupleExtend<T> {
    type ResultType;

    fn extend(self, t: T) -> Self::ResultType;
}

impl<T> TupleExtend<T> for () {
    type ResultType = (T,);

    fn extend(self, t: T) -> Self::ResultType {
        (t,)
    }
}

macro_rules! impl_tuple_append {
    ( () ) => {};
    ( ( $t0:ident $(, $types:ident)* ) ) => {
        impl<$t0, $($types,)* T> TupleExtend<T> for ($t0, $($types,)*) {
            // Trailing comma, just to be extra sure we are dealing
            // with a tuple and not a parenthesized type/expr.
            type ResultType = ($t0, $($types,)* T,);

            fn extend(self, t: T) -> Self::ResultType {
                // Reuse the type identifiers to destructure ourselves:
                let ($t0, $($types,)*) = self;
                // Create a new tuple with the original elements, plus the new one:
                ($t0, $($types,)* t,)
            }
        }

        // Recurse for one smaller size:
        impl_tuple_append! { ($($types),*) }
    };
}

impl_tuple_append! {
    // Supports tuples up to size 10:
    (_1, _2, _3, _4, _5, _6, _7, _8, _9, _10)
}

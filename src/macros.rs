#[macro_export]
macro_rules! hash_trait_object {
    ($($path:tt)+) => {
        $crate::__internal_hash_trait_object!(begin $($path)+);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __internal_hash_trait_object {
    // Invocation started with `<`, parse generics.
    (begin < $($rest:tt)*) => {
        $crate::__internal_hash_trait_object!(generics () () $($rest)*);
    };

    // Invocation did not start with `<`.
    (begin $first:tt $($rest:tt)*) => {
        $crate::__internal_hash_trait_object!(path () ($first) $($rest)*);
    };

    // End of generics.
    (generics ($($generics:tt)*) () > $($rest:tt)*) => {
        $crate::__internal_hash_trait_object!(path ($($generics)*) () $($rest)*);
    };

    // Generics open bracket.
    (generics ($($generics:tt)*) ($($brackets:tt)*) < $($rest:tt)*) => {
        $crate::__internal_hash_trait_object!(generics ($($generics)* <) ($($brackets)* <) $($rest)*);
    };

    // Generics close bracket.
    (generics ($($generics:tt)*) (< $($brackets:tt)*) > $($rest:tt)*) => {
        $crate::__internal_hash_trait_object!(generics ($($generics)* >) ($($brackets)*) $($rest)*);
    };

    // Token inside of generics.
    (generics ($($generics:tt)*) ($($brackets:tt)*) $first:tt $($rest:tt)*) => {
        $crate::__internal_hash_trait_object!(generics ($($generics)* $first) ($($brackets)*) $($rest)*);
    };

    // End with `where` clause.
    (path ($($generics:tt)*) ($($path:tt)*) where $($rest:tt)*) => {
        $crate::__internal_hash_trait_object!(impl ($($generics)*) ($($path)*) ($($rest)*));
    };

    // End without `where` clause.
    (path ($($generics:tt)*) ($($path:tt)*)) => {
        $crate::__internal_hash_trait_object!(impl ($($generics)*) ($($path)*) ());
    };

    // Token inside of path.
    (path ($($generics:tt)*) ($($path:tt)*) $first:tt $($rest:tt)*) => {
        $crate::__internal_hash_trait_object!(path ($($generics)*) ($($path)* $first) $($rest)*);
    };

    // The impl.
    (impl ($($generics:tt)*) ($($path:tt)*) ($($bound:tt)*)) => {
        impl<'hash, $($generics)*> $crate::__private::Hash for dyn $($path)* + 'hash where $($bound)* {
            fn hash<H: $crate::__private::Hasher>(&self, state: &mut H) {
                $crate::DynHash::hash(self, state);
            }
        }
        impl<'hash, $($generics)*> $crate::__private::Hash for dyn $($path)* + $crate::__private::Send + 'hash where $($bound)* {
            fn hash<H: $crate::__private::Hasher>(&self, state: &mut H) {
                $crate::DynHash::hash(self, state);
            }
        }
        impl<'hash, $($generics)*> $crate::__private::Hash for dyn $($path)* + $crate::__private::Sync + 'hash where $($bound)* {
            fn hash<H: $crate::__private::Hasher>(&self, state: &mut H) {
                $crate::DynHash::hash(self, state);
            }
        }
        impl<'hash, $($generics)*> $crate::__private::Hash for dyn $($path)* + $crate::__private::Send + $crate::__private::Sync + 'hash where $($bound)* {
            fn hash<H: $crate::__private::Hasher>(&self, state: &mut H) {
                $crate::DynHash::hash(self, state);
            }
        }
    };
}

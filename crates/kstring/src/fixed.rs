macro_rules! fixed_string {
    ($name:ident, $len:literal) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub(crate) struct $name {
            array: [u8; $len],
        }

        impl $name {
            pub(crate) fn new(s: &str) -> Self {
                assert_eq!(s.as_bytes().len(), $len);
                let mut array = [0; $len];
                array.copy_from_slice(&s.as_bytes()[0..$len]);
                Self { array }
            }

            #[inline]
            pub(crate) fn into_boxed_str(&self) -> Box<str> {
                Box::from(self.as_str())
            }

            #[inline]
            pub(crate) fn as_str(&self) -> &str {
                unsafe { std::str::from_utf8_unchecked(&self.array) }
            }
        }
    };
}

fixed_string!(FixedString1, 1);
fixed_string!(FixedString2, 2);
fixed_string!(FixedString3, 3);
fixed_string!(FixedString4, 4);
fixed_string!(FixedString5, 5);
fixed_string!(FixedString6, 6);
fixed_string!(FixedString7, 7);
fixed_string!(FixedString8, 8);
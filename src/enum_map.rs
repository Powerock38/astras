#[macro_export]
macro_rules! enum_map {
    ($enum_name:ident => $data_type:ty {
        $first_name:ident = $first_data:expr,
        $( $name:ident = $data:expr ),* $(,)?
    }) => {
        #[derive(PartialEq, Eq, Hash, Clone, Copy, Reflect, Default, Debug)]
        pub enum $enum_name {
            #[default]
            $first_name,
            $(
                $name,
            )*
        }

        impl $enum_name {
            #[allow(dead_code)]
            pub const ALL: &'static [$enum_name] = &[
                $enum_name::$first_name,
                $(
                    $enum_name::$name,
                )*
            ];

            #[inline]
            pub fn data(&self) -> $data_type {
                match self {
                    Self::$first_name => $first_data,
                    $(
                        Self::$name => $data,
                    )*
                }
            }

            #[allow(dead_code)]
            pub fn to_str(self) -> &'static str {
                match self {
                    Self::$first_name => stringify!($first_name),
                    $(
                        Self::$name => stringify!($name),
                    )*
                }
            }

            #[allow(dead_code)]
            pub fn from_str(s: &str) -> Option<Self> {
                match s {
                    stringify!($first_name) => Some(Self::$first_name),
                    $(
                        stringify!($name) => Some(Self::$name),
                    )*
                    _ => None,
                }
            }
        }
    }
}

/// Creates a struct that resembles a record.
/// A record automatically has the members of the argument, generates a `::new()` function and
/// sets the variables.
///
/// # Arguments
///
/// * `$i:ident` - An identifier which is the struct/record name
/// * `$($var:ident: $t:ty),*` - The variables/members to be set in the struct/record
/// * **Optional:** `{ $($impl:tt)* }` - An added `impl` of the struct
///
/// **Note:** The struct and each member are automatically public (through the use of `pub`
/// keyword).
///
/// # Examples
///
/// ```
/// record!(Card, name: String, id: String);
/// ```
/// Resulting in:
/// ```
/// pub struct Card {
///     pub name: String,
///     pub id: String,
/// }
///
/// impl Card {
///     pub fn new(name: String, id: String) -> Self {
///         Self {
///             name,
///             id,
///         }
///     }
/// }
/// ```
///
/// The extra `impl` argument just adds another `impl Card` with your own choice of `impl`.
///
/// # Example with impl
///
/// ```
/// record!(Card, name: String, id: String, {
///     fn print(self) { println!("{} {}", self.name, self.id); }
/// });
/// ```
/// Resulting in:
/// ```
/// pub struct Card {
///     pub name: String,
///     pub id: String,
/// }
///
/// impl Card {
///     pub fn new(name: String, id: String) -> Self {
///         Self {
///             name,
///             id,
///         }
///     }
/// }
///
/// impl Card {
///     fn print(self) { println!("{} {}", self.name, self.id); }
/// }
/// ```
#[macro_export]
macro_rules! record {
    ($i:ident, $($var:ident: $t:ty),*, { $($impl:tt)* }) => {
        pub struct $i {
            $(pub $var: $t),*
        }

        impl $i {
            pub fn new($($var: $t),*) -> Self {
                Self {
                    $($var),*
                }
            }
        }

        impl $i { $($impl)* }
    };

    ($i:ident, $($var:ident: $t:ty),*) => {
        record!($i, $($var: $t),*, {});
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn new_fn() {
        record!(Card, name: String, id: String);
        let card = Card::new("Robert".to_string(), "12345".to_string());
        assert_eq!(card.name, "Robert".to_string());
        assert_eq!(card.id, "12345".to_string());
    }

    #[test]
    fn impl_works() {
        record!(Card, name: String, id: String, {
            fn become_a_string_to_work(self) -> String {
                format!("{}, {}", self.name, self.id)
            }
        });
        let card = Card::new("Robert".to_string(), "12345".to_string());
        assert_eq!(card.become_a_string_to_work(), "Robert, 12345".to_string());
    }
}

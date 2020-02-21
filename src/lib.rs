extern crate proc_macro;

use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, DeriveInput, Ident};

#[derive(Debug, FromMeta)]
struct MacroArgs {
    #[darling(default)]
    serialization: Option<Ident>,
}

/// A macro attribute that indicates a type that needs to be tracked and implements
/// [Trackable](https://docs.rs/track/trait.Trackable.html) and [TrackableMarker](https://docs.rs/track/trait.TrackableMarker.html).
///
/// # Examples
///
/// Add `track` attribute to mark your struct as trackable.
///
/// ```rust
/// // imports all necessarily types for the `track` attribute.
/// use track::preclude::*;
///
/// #[track]
/// #[derive(Debug)]
/// pub struct Position {
///     pub x: u32,
///     pub y: u32,
/// }
/// ```
///
/// You can specify a serialization method for the track macro.
/// Give the name of the type that implements [SerializationStrategy](https://docs.rs/track/serialization/trait.SerializationStrategy.html), and make sure it is in scope for the macro.
/// Such as:
///
/// ```rust
/// use track::serialization::bincode::Bincode;
///
/// #[track(serialization = "Bincode")]
/// struct Postition ...
/// ```
///
/// For a more in-depth example checkout the [examples](https://github.com/entity-sync-rs/track/tree/master/examples) on github.
#[proc_macro_attribute]
pub fn track(_attr: TokenStream, unparsed_input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(_attr as AttributeArgs);
    let input: DeriveInput = parse_macro_input!(unparsed_input as syn::DeriveInput);

    let _args = match MacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return proc_macro::TokenStream::from(e.write_errors());
        }
    };

    let serialization = if let Some(ser) = _args.serialization {
        ser
    } else {
        Ident::from_string("Bincode").unwrap()
    };

    let name = input.ident.clone();

    let implementation = quote! {
        #[derive(Clone, SerdeDiff, Serialize, Deserialize)]
        #input

        impl Trackable<#name, #serialization> for #name {
            fn track<'notifier, I: Identifier>(&mut self, sender: &'notifier Sender<ModificationEvent<I>>, identifier: I) -> Tracker<'_, 'notifier, #name,  #serialization, I> {
                Tracker::new(self, sender, #serialization, identifier)
            }
        }

        impl TrackableMarker for #name { }
    };

    proc_macro::TokenStream::from(implementation)
}

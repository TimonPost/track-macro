extern crate proc_macro;

use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, DeriveInput, Ident};

#[derive(Debug, FromMeta)]
struct MacroArgs {
    #[darling(default)]
    serialisation: Option<Ident>,
}

/// A macro attribute that indicates a type that needs to be tracked and implements
/// [Trackable](LINK) and [TrackableMarker](LINK).
///
/// To detect, track and test changes, the structure values must be serialized.
/// You can add a custom serializer with the parameter `serialization`.
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

    let serialisation = if let Some(ser) = _args.serialisation {
        ser
    } else {
        Ident::from_string("Bincode").unwrap()
    };

    let name = input.ident.clone();

    let implementation = quote! {
        #[derive(Clone, SerdeDiff, Serialize, Deserialize)]
        #input

        impl Trackable<#name, #serialisation> for #name {
            fn track<'notifier, I: Identifier>(&mut self, sender: &'notifier Sender<ModificationEvent<I>>, identifier: I) -> Tracker<'_, 'notifier, #name,  #serialisation, I> {
                Tracker::new(self, sender, #serialisation, identifier)
            }
        }

        impl TrackableMarker for #name { }
    };

    proc_macro::TokenStream::from(implementation)
}

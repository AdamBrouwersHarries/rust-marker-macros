/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/. */

// #![deny(warnings)]
#![allow(unused_variables, dead_code, non_camel_case_types)]

//! A procedural macro as a syntactical sugar to `gecko_profiler_label!` macro.
//! You can use this macro on top of functions to automatically append the
//! label frame to the function.
//!
//! Example usage:
//! ```rust
//! #[gecko_profiler_fn_label(DOM)]
//! fn foo(bar: u32) -> u32 {
//!     bar
//! }
//!
//! #[gecko_profiler_fn_label(Javascript, IonMonkey)]
//! pub fn bar(baz: i8) -> i8 {
//!     baz
//! }
//! ```
//!
//! See the documentation of `gecko_profiler_label!` macro to learn more about
//! its parameters.

use syn::Path;
use proc_macro2::Span;
use syn::spanned::Spanned;
extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote_spanned;
use quote::{quote, ToTokens};
use syn::Fields;
use syn::{parse_macro_input, DeriveInput, Ident};
use syn::{Data, Error};

// We want to try and derive this:
// pub trait ProfilerMarker: Serialize + DeserializeOwned {
//     /// A static method that returns the name of the marker type.
//     fn marker_type_name() -> &'static str;
//     /// A static method that returns a `MarkerSchema`, which contains all the
//     /// information needed to stream the display schema associated with a
//     /// marker type.
//     fn marker_type_display() -> MarkerSchema;
//     /// A method that streams the marker payload data as JSON object properties.
//     /// Please see the [JSONWriter] struct to see its methods.
//     fn stream_json_marker_data(&self, json_writer: &mut JSONWriter);
// }

static LOCATIONS: &[&str] = &[
    "MarkerChart",
    "MarkerTable",
    "TimelineOverview",
    "TimelineMemory",
    "TimelineIPC",
    "TimelineFileIO",
    "StackChart",
];

static FORMATS: &[&str] = &[
    "Url",
    "FilePath",
    "SanitizedString",
    "String",
    "UniqueString",
    "Duration",
    "Time",
    "Seconds",
    "Milliseconds",
    "Microseconds",
    "Nanoseconds",
    "Bytes",
    "Percentage",
    "Integer",
    "Decimal",
];

fn is_valid_marker_location(ident: &syn::Ident) -> bool {
    let ident_as_string = ident.to_string();
    LOCATIONS.iter().any(|e| *e == ident_as_string.as_str())
}

fn is_valid_format_string(ident: &syn::Ident) -> bool {
    let ident_as_string = ident.to_string();
    FORMATS.iter().any(|e| *e == ident_as_string.as_str())
}

#[proc_macro_derive(
    ProfilerMarker,
    attributes(marker_display, MarkerChart, searchable, format)
)]
pub fn derive_profiler_marker(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Step 1: Parse the input into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    let mut marker_locations: Vec<syn::Ident> = vec![];

    // Step 2: Check the attributes of the input, look for marker specific ones.
    // This could be done better in terms of error reporting and how we check for
    // ill-formed attributes (e.g. #[marker_display()])
    for attr in input.attrs {
        // Ignore inner attributes, as they're not from us
        match attr.style {
            syn::AttrStyle::Inner(_) => {
                continue;
            }
            _ => {}
        }

        // look at the path: we're expecting `marker_display`
        if attr.path().is_ident("marker_display") {
            match attr.parse_nested_meta(|meta| {
                match meta.path.get_ident() {
                    Some(i) => {
                        if is_valid_marker_location(i) {
                            marker_locations.push(i.clone());
                            Ok(())
                        } else {
                            Err(meta.error("Unsupported marker display location"))
                        }
                    }
                    None => {
                        // No need to defer this.
                        Err(meta.error(
                            "Expected a marker display location as argument to 'marker_display'",
                        ))
                    }
                }
            }) {
                Err(e) => return e.into_compile_error().into(),
                Ok(_) => {} // continue safely.
            };
        }
    }

    println!("Found marker locations: {:?}", marker_locations);

    // Get the name of the input
    let name = &input.ident;
    // Get generic type accoutremonts
    let (_impl_generics, _ty_generics, _where_clause) = input.generics.split_for_impl();

    // Step 3: We need to generate three methods:
    // marker_type_name (For which we'll use the name of the struct)
    // marker_type_display (For which we'll use the fields of the struct)
    // stream_json_marker_data (Fro which we'll use the fields of the struct)
    let marker_type_name_fn = marker_type_name_impl(&name);
    let marker_type_display_fn = marker_type_display_impl(name, &marker_locations, &input.data);
    let stream_json_marker_data_fn = stream_json_marker_data_impl();

    let total_impl = quote! {

        impl ProfilerMarker for #name {
            #marker_type_name_fn
            #marker_type_display_fn
            #stream_json_marker_data_fn
        }
    };

    println!("Total generation: {}", total_impl);

    proc_macro::TokenStream::from(total_impl)
}

fn marker_type_name_impl(name: &Ident) -> TokenStream {
    let name_str = name.to_token_stream().to_string();
    let ts = quote! {
        fn marker_type_name() -> &'static str {
            #name_str
        }
    }
    .into();

    println!("Generated type name impl: {}", ts);

    ts
}

fn marker_type_display_impl(_name: &Ident, _marker_locations: &Vec<syn::Ident>, data: &Data) -> TokenStream {
    let key_label_formats = match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let displays = fields.named.iter().map(|f| {
                    let fname = &f.ident;
                    let fname_str = fname.to_token_stream().to_string();
                    let attrs = &f.attrs;
                    let mut format: Option<Ident> = None;
                    let mut searchable: bool = false;
                    for attr in attrs {
                        match attr.style {
                            syn::AttrStyle::Outer => {
                                if attr.path().is_ident("searchable") {
                                    searchable = true;
                                } else if attr.path().is_ident("format") {
                                    if format.is_some() {
                                        return Error::new(attr.span(),"Too many format arguments").into_compile_error().into();
                                    }
                                    match attr.parse_nested_meta(|meta| {
                                        match meta.path.get_ident() {
                                            Some(i) => {
                                                if is_valid_format_string(i) {
                                                    format = Some(i.clone());
                                                    Ok(())
                                                } else {
                                                    Err(meta.error("Unsupported format specifier"))
                                                }
                                            }
                                            None => {
                                                Err(meta.error(
                                                    "Expected a marker format specifier as argument to 'format'",
                                                ))
                                            }
                                        }
                                    }) {
                                        Err(e) => return e.into_compile_error().into(),
                                        Ok(_) => {} // continue safely.
                                    };
                                }
                            }
                            syn::AttrStyle::Inner(_) => {
                            },
                        }
                    }

                    let fstring = match format {
                        Some(ident) => format!("Format::{}", ident.to_string()),
                        None => "Format::String".to_string(),
                    };
                    let format_type = syn::parse_str::<Path>(fstring.as_str()).unwrap();

                    // Ident::new(fname.as_str(), Span::call_site());
                    // println!("Format_type: {:?}", format_type.into_token_stream());

                    if searchable { 
                        quote! {
                            schema.add_key_label_format_searchable(#fname_str, #fname_str, #format_type, Searchable::Searchable);
                        }
                    } else {
                        quote! {
                            schema.add_key_label_format(#fname_str, #fname_str, #format_type);
                        }
                    }
                });

                quote! {
                    #(; #displays)*
                }
            }
            Fields::Unnamed(ref _fields) => {
                todo!()
            }
            Fields::Unit => {
                todo!()
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    };

    let ts = quote! {
        fn marker_type_display() -> MarkerSchema {
            let mut schema = MarkerSchema::new(&[Location::MarkerChart]);
            schema.set_chart_label("Name: {marker.name}");

            #key_label_formats

            schema
        }
    }
    .into();
    // use gecko_profiler::marker::schema::*;

    //         schema.set_tooltip_label("{marker.data.a}");
    //         schema.add_key_label_format("a", "A Value", Format::Integer);
    //         schema.add_key_label_format("b", "B Value", Format::String);
    //         schema
    println!("Generated type display impl: {}", ts);

    ts
}

fn stream_json_marker_data_impl() -> TokenStream {
    let ts = quote! {
        fn stream_json_marker_data(&self, json_writer: &mut JSONWriter) -> () {
        }
    }
    .into();
    println!("Generated streaming json marker: {}", ts);

    ts
    // fn stream_json_marker_data(&self, json_writer: &mut gecko_profiler::JSONWriter) {
    //         json_writer.int_property("a", self.a.into());
    //         json_writer.string_property("b", self.b.as_ref());
}

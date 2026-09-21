use proc_macro::TokenStream;

use quote::{format_ident, quote};
use syn::{
    Attribute, Data, DeriveInput, Error, Fields, GenericArgument, LitInt, PathArguments, Result,
    Type, parse_macro_input,
};

/// Derives decoding for a tagged TLV, dispatching enum, or frame payload struct.
///
/// A tagged struct requires `#[tlv(type = <u32>)]`. An untagged struct collects
/// TLVs into `Option<T>` and `Vec<T>` fields. Every enum variant must contain one
/// tagged TLV type.
#[proc_macro_derive(Tlv, attributes(tlv))]
pub fn derive_tlv(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_tlv_impl(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Derives `titlv::TlvReader` for a struct with consecutively encoded fields.
#[proc_macro_derive(TlvReader)]
pub fn derive_tlv_reader(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_reader_impl(&input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn derive_tlv_impl(input: DeriveInput) -> Result<proc_macro2::TokenStream> {
    match &input.data {
        Data::Struct(_) => {
            if let Some(type_id) = tlv_type(&input.attrs)? {
                let reader = derive_reader_impl(&input)?;
                let name = &input.ident;

                Ok(quote! {
                    #reader

                    impl ::titlv::Tlv for #name {
                        const TYPE: ::titlv::types::Tag = ::titlv::types::Tag::const_new::<#type_id>();

                        fn from_packet(packet: ::titlv::types::TlvPacket<'_>) -> ::titlv::error::Result<Self> {
                            if packet.header.r#type != Self::TYPE {
                                return Err(::titlv::error::Error::UnexpectedTlvType);
                            }

                            <Self as ::titlv::TlvReader>::read(&mut &packet.payload[..])
                        }
                    }
                })
            } else {
                derive_frame_payload_impl(&input)
            }
        }
        Data::Enum(data) => {
            let name = &input.ident;
            let variants = data
                .variants
                .iter()
                .map(|variant| {
                    let variant_name = &variant.ident;
                    let Fields::Unnamed(fields) = &variant.fields else {
                        return Err(Error::new_spanned(
                            variant,
                            "TLV enum variants must contain exactly one root TLV type",
                        ));
                    };
                    if fields.unnamed.len() != 1 {
                        return Err(Error::new_spanned(
                            variant,
                            "TLV enum variants must contain exactly one root TLV type",
                        ));
                    }
                    let variant_type = &fields.unnamed[0].ty;

                    Ok(quote! {
                        tag if tag == <#variant_type as ::titlv::Tlv>::TYPE => Ok(Self::#variant_name(
                            <#variant_type as ::titlv::Tlv>::from_packet(packet)?,
                        )),
                    })
                })
                .collect::<Result<Vec<_>>>()?;

            Ok(quote! {
                impl ::titlv::Tlv for #name {
                    const TYPE: ::titlv::types::Tag = ::titlv::types::Tag::UNKNOWN;

                    fn from_packet(packet: ::titlv::types::TlvPacket<'_>) -> ::titlv::error::Result<Self> {
                        match packet.header.r#type {
                            #(#variants)*
                            _ => Err(::titlv::error::Error::UnexpectedTlvType),
                        }
                    }
                }
            })
        }
        Data::Union(_) => Err(Error::new_spanned(
            input.ident,
            "Tlv can only be derived for structs and enums",
        )),
    }
}

fn derive_frame_payload_impl(input: &DeriveInput) -> Result<proc_macro2::TokenStream> {
    let Data::Struct(data) = &input.data else {
        unreachable!();
    };
    let Fields::Named(fields) = &data.fields else {
        return Err(Error::new_spanned(
            &data.fields,
            "frame payload must be a struct with named Option<T> or Vec<T> fields",
        ));
    };
    let name = &input.ident;
    let mut initializers = Vec::new();
    let mut matches = Vec::new();

    for field in &fields.named {
        let field_name = field.ident.as_ref().unwrap();
        let (collection, inner_type) = collection_type(&field.ty)?;

        match collection {
            "Option" => {
                initializers.push(quote! { #field_name: None });
                matches.push(quote! {
                    tag if tag == <#inner_type as ::titlv::Tlv>::TYPE => {
                        if let Ok(value) = <#inner_type as ::titlv::Tlv>::from_packet(packet) {
                            self.#field_name = Some(value);
                        }
                    }
                });
            }
            "Vec" => {
                initializers.push(quote! { #field_name: ::std::vec::Vec::new() });
                matches.push(quote! {
                    tag if tag == <#inner_type as ::titlv::Tlv>::TYPE => {
                        if let Ok(value) = <#inner_type as ::titlv::Tlv>::from_packet(packet) {
                            self.#field_name.push(value);
                        }
                    }
                });
            }
            _ => unreachable!(),
        }
    }

    Ok(quote! {
        impl ::titlv::types::FramePayload for #name {
            fn with_capacity(_capacity: usize) -> Self {
                Self { #(#initializers),* }
            }

            fn push(&mut self, packet: ::titlv::types::TlvPacket<'_>) {
                match packet.header.r#type {
                    #(#matches,)*
                    _ => {}
                }
            }
        }
    })
}

fn collection_type(ty: &Type) -> Result<(&str, &Type)> {
    let Type::Path(path) = ty else {
        return Err(Error::new_spanned(ty, "expected Option<T> or Vec<T>"));
    };
    let Some(segment) = path.path.segments.last() else {
        return Err(Error::new_spanned(ty, "expected Option<T> or Vec<T>"));
    };
    let collection = match segment.ident.to_string().as_str() {
        "Option" => "Option",
        "Vec" => "Vec",
        _ => return Err(Error::new_spanned(ty, "expected Option<T> or Vec<T>")),
    };
    let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return Err(Error::new_spanned(ty, "expected Option<T> or Vec<T>"));
    };
    let [GenericArgument::Type(inner_type)] = arguments.args.iter().collect::<Vec<_>>()[..] else {
        return Err(Error::new_spanned(ty, "expected Option<T> or Vec<T>"));
    };

    Ok((collection, inner_type))
}

fn derive_reader_impl(input: &DeriveInput) -> Result<proc_macro2::TokenStream> {
    let Data::Struct(data) = &input.data else {
        return Err(Error::new_spanned(
            &input.ident,
            "TlvReader can only be derived for structs",
        ));
    };
    let name = &input.ident;

    let body = match &data.fields {
        Fields::Named(fields) => {
            let names = fields
                .named
                .iter()
                .map(|field| field.ident.as_ref().unwrap());
            let reads = fields.named.iter().map(|field| {
                let name = field.ident.as_ref().unwrap();
                let ty = &field.ty;
                quote! { let #name = <#ty as ::titlv::TlvReader>::read(buf)?; }
            });
            quote! {
                #(#reads)*
                Ok(Self { #(#names),* })
            }
        }
        Fields::Unnamed(fields) => {
            let names = (0..fields.unnamed.len())
                .map(|index| format_ident!("field_{index}"))
                .collect::<Vec<_>>();
            let reads = fields.unnamed.iter().zip(&names).map(|(field, name)| {
                let ty = &field.ty;
                quote! { let #name = <#ty as ::titlv::TlvReader>::read(buf)?; }
            });
            quote! {
                #(#reads)*
                Ok(Self(#(#names),*))
            }
        }
        Fields::Unit => quote! { Ok(Self) },
    };

    Ok(quote! {
        impl ::titlv::TlvReader for #name {
            fn read<R: ::std::io::BufRead>(buf: &mut R) -> ::titlv::error::Result<Self> {
                #body
            }
        }
    })
}

fn tlv_type(attributes: &[Attribute]) -> Result<Option<LitInt>> {
    let mut type_id = None;

    for attribute in attributes {
        if attribute.path().is_ident("tlv") {
            attribute.parse_nested_meta(|meta| {
                if !meta.path.is_ident("type") {
                    return Err(meta.error("expected `type = <u32>`"));
                }
                if type_id.is_some() {
                    return Err(meta.error("TLV type is already specified"));
                }
                type_id = Some(meta.value()?.parse()?);
                Ok(())
            })?;
        }
    }

    Ok(type_id)
}

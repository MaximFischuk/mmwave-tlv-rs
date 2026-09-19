use proc_macro::TokenStream;

use quote::{format_ident, quote};
use syn::{Attribute, Data, DeriveInput, Error, Fields, LitInt, Result, parse_macro_input};

/// Derives `titlv::Tlv` for a tagged TLV struct or dispatching enum.
///
/// A struct requires `#[tlv(type = <u32>)]`. Every enum variant must contain
/// one tagged TLV type.
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
            let reader = derive_reader_impl(&input)?;
            let type_id = tlv_type(&input.attrs)?;
            let name = &input.ident;

            Ok(quote! {
                #reader

                impl ::titlv::Tlv for #name {
                    fn from_packet(packet: ::titlv::types::TlvPacket<'_>) -> ::titlv::error::Result<Self> {
                        if packet.header.r#type != #type_id {
                            return Err(::titlv::error::Error::UnexpectedTlvType);
                        }

                        <Self as ::titlv::TlvReader>::read(&mut &packet.payload[..])
                    }
                }
            })
        }
        Data::Enum(data) => {
            let name = &input.ident;
            let variants = data
                .variants
                .iter()
                .map(|variant| {
                    let type_id = tlv_type(&variant.attrs)?;
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
                        #type_id => Ok(Self::#variant_name(
                            <#variant_type as ::titlv::Tlv>::from_packet(packet)?,
                        )),
                    })
                })
                .collect::<Result<Vec<_>>>()?;

            Ok(quote! {
                impl ::titlv::Tlv for #name {
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

fn tlv_type(attributes: &[Attribute]) -> Result<LitInt> {
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

    type_id.ok_or_else(|| {
        Error::new(
            proc_macro2::Span::call_site(),
            "missing #[tlv(type = <u32>)]",
        )
    })
}

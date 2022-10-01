//! Generates extensions.rs and many globally accessible symbols.

use convert_case::{Case, Casing};
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::api_parser::*;

struct Tokens {
    opaque_types: Vec<TokenStream>,
    variant_enumerators: Vec<TokenStream>,
    variant_fn_decls: Vec<TokenStream>,
    variant_fn_inits: Vec<TokenStream>,
}

struct TypeNames {
    /// "PackedVector2Array"
    pascal_case: String,

    /// "packed_vector2_array"
    snake_case: String,

    /// "PACKED_VECTOR2_ARRAY"
    shout_case: String,

    /// GDNativeVariantType_GDNATIVE_VARIANT_TYPE_PACKED_VECTOR2_ARRAY
    sys_variant_type: Ident,
}

pub fn generate_central_file(
    api: &ExtensionApi,
    build_config: &str,
    gen_path: &Path,
    out_files: &mut Vec<PathBuf>,
) {
    let tokens = load_extension_api(api, build_config);
    let Tokens {
        opaque_types,
        variant_enumerators,
        variant_fn_decls,
        variant_fn_inits,
    } = tokens;

    let tokens = quote! {
        #![allow(dead_code)]
        use crate::{GDNativeVariantPtr, GDNativeTypePtr};

        pub mod types {
            #(#opaque_types)*
        }

        pub struct InterfaceCache {
            #(#variant_fn_decls)*
        }

        impl InterfaceCache {
            pub(crate) unsafe fn new(interface: &crate::GDNativeInterface) -> Self {
                Self {
                    #(#variant_fn_inits)*
                }
            }
        }

        pub enum VariantType {
            #(#variant_enumerators),*
        }
    };

    let string = tokens.to_string();

    let _ = std::fs::create_dir(gen_path);
    let out_path = gen_path.join("extensions.rs");
    std::fs::write(&out_path, string).expect("failed to write extension file");

    out_files.push(out_path);
}

fn load_extension_api(model: &ExtensionApi, build_config: &str) -> Tokens {
    let mut opaque_types = vec![];
    let mut variant_enumerators = vec![];
    let mut variant_fn_decls = vec![];
    let mut variant_fn_inits = vec![];

    for class in &model.builtin_class_sizes {
        if &class.build_configuration == build_config {
            for ClassSize { name, size } in &class.sizes {
                opaque_types.push(make_opaque_type(name, *size));
            }

            break;
        }
    }

    // Find variant types, for which `variant_get_ptr_destructor` returns a non-null function pointer.
    // List is directly sourced from extension_api.json (information would also be in variant_destruct.cpp).
    let mut class_map = HashMap::new();
    for class in &model.builtin_classes {
        let normalized_name = class.name.to_lowercase();

        class_map.insert(normalized_name, class);
    }

    let class_map = class_map;

    for enum_ in &model.global_enums {
        if &enum_.name == "Variant.Type" {
            for ty in &enum_.values {
                let shout_case = ty
                    .name
                    .strip_prefix("TYPE_")
                    .expect("Enum name begins with 'TYPE_'");

                if shout_case == "NIL" || shout_case == "MAX" {
                    continue;
                }

                // Lowercase without underscore, to map SHOUTY_CASE to shoutycase
                let normalized = shout_case.to_lowercase().replace("_", "");

                let pascal_case: String;
                let has_destructor: bool;
                let constructors: Option<&Vec<Constructor>>;
                let methods: Option<&Vec<BuiltinMethod>>;
                if let Some(class) = class_map.get(&normalized) {
                    pascal_case = class.name.clone();
                    has_destructor = class.has_destructor;
                    methods = class.methods.as_ref();
                    constructors = Some(&class.constructors);
                } else {
                    assert_eq!(normalized, "object");
                    pascal_case = "Object".to_string();
                    has_destructor = false;
                    methods = None;
                    constructors = None;
                }

                let type_names = TypeNames {
                    pascal_case,
                    snake_case: shout_case.to_lowercase(),
                    shout_case: shout_case.to_string(),
                    sys_variant_type: format_ident!(
                        "GDNativeVariantType_GDNATIVE_VARIANT_TYPE_{}",
                        shout_case
                    ),
                };

                let value = ty.value;
                variant_enumerators.push(make_enumerator(&type_names, value));

                let (decl, init) =
                    make_variant_fns(&type_names, has_destructor, methods, constructors);

                variant_fn_decls.push(decl);
                variant_fn_inits.push(init);
            }

            break;
        }
    }

    Tokens {
        opaque_types,
        variant_enumerators,
        variant_fn_decls,
        variant_fn_inits,
    }
}

fn make_enumerator(type_names: &TypeNames, value: i32) -> TokenStream {
    let enumerator = format_ident!("{}", type_names.shout_case);
    let value = proc_macro2::Literal::i32_unsuffixed(value);

    quote! {
       #enumerator = #value
    }
}

fn make_opaque_type(name: &str, size: usize) -> TokenStream {
    // Capitalize: "int" -> "Int"
    let (first, rest) = name.split_at(1);
    let ident = format_ident!("Opaque{}{}", first.to_uppercase(), rest);
    //let upper = format_ident!("SIZE_{}", name.to_uppercase());
    quote! {
        pub type #ident = crate::opaque::Opaque<#size>;
        //pub const #upper: usize = #size;
    }
}

fn make_variant_fns(
    type_names: &TypeNames,
    has_destructor: bool,
    methods: Option<&Vec<BuiltinMethod>>,
    constructors: Option<&Vec<Constructor>>,
) -> (TokenStream, TokenStream) {
    let (destroy_decls, destroy_inits) = make_destroy_fns(&type_names, has_destructor);

    let (construct_decls, construct_inits) = make_construct_fns(&type_names, constructors);

    let (method_decls, method_inits) = make_builtin_methods(&type_names, methods);

    let to_variant = format_ident!("{}_to_variant", type_names.snake_case);
    let from_variant = format_ident!("{}_from_variant", type_names.snake_case);
    let to_variant_error = format_load_error(&to_variant);
    let from_variant_error = format_load_error(&from_variant);
    let variant_type = &type_names.sys_variant_type;

    // Field declaration
    let decl = quote! {
        pub #to_variant: unsafe extern "C" fn(GDNativeVariantPtr, GDNativeTypePtr),
        pub #from_variant: unsafe extern "C" fn(GDNativeTypePtr, GDNativeVariantPtr),
        #construct_decls
        #destroy_decls
        #method_decls
    };

    // Field initialization in new()
    let init = quote! {
        #to_variant: {
            let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
            ctor_fn(crate:: #variant_type).expect(#to_variant_error)
        },
        #from_variant:  {
            let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
            ctor_fn(crate:: #variant_type).expect(#from_variant_error)
        },
        #construct_inits
        #destroy_inits
        #method_inits
    };

    (decl, init)
}

fn make_builtin_methods(
    type_names: &TypeNames,
    methods: Option<&Vec<BuiltinMethod>>,
) -> (TokenStream, TokenStream) {
    let methods = match methods {
        Some(c) => c,
        None => return (TokenStream::new(), TokenStream::new()),
    };

    let variant_type = &type_names.sys_variant_type;
    let mut decls = Vec::with_capacity(methods.len());
    let mut inits = Vec::with_capacity(methods.len());
    for method in methods {
        if method.is_vararg {
            continue;
        }; // TODO: vararg fns don't support ptrcalls, use varcall
        let ident = format_ident!("{}_{}", type_names.snake_case, method.name);
        let hash = method.hash;
        let name_str = Literal::byte_string(format!("{}\0", method.name).as_bytes());
        let error_msg = format_load_error(&ident);

        // TODO: use arg_count if provided, but calculate if varargs
        let _arg_count = method.arguments.as_ref().map_or(0, |args| args.len()) as i32;

        decls.push(quote! {
            pub #ident: unsafe extern "C" fn(GDNativeTypePtr,  *const GDNativeTypePtr, GDNativeTypePtr, i32),
        });
        inits.push(quote! {
            #ident : {
                (interface.variant_get_ptr_builtin_method.unwrap())(
                    crate:: #variant_type,
                    ::std::ffi::CStr::from_bytes_with_nul( #name_str ).unwrap().as_ptr(),
                    #hash,
                ).expect(#error_msg)
            },
        });
    }

    let decl = quote! {
        #(#decls)*
    };
    let init = quote! {
        #(#inits)*
    };

    (decl, init)
}

fn make_construct_fns(
    type_names: &TypeNames,
    constructors: Option<&Vec<Constructor>>,
) -> (TokenStream, TokenStream) {
    let constructors = match constructors {
        Some(c) => c,
        None => return (TokenStream::new(), TokenStream::new()),
    };

    // Constructor vec layout:
    //   [0]: default constructor
    //   [1]: copy constructor
    //  rest: type-specific

    // Sanity checks -- ensure format is as expected
    for (i, c) in constructors.iter().enumerate() {
        assert_eq!(i, c.index);
    }

    assert!(constructors[0].arguments.is_none());

    if let Some(args) = &constructors[1].arguments {
        assert_eq!(args.len(), 1);
        assert_eq!(args[0].name, "from");
        assert_eq!(args[0].type_, type_names.pascal_case);
    } else {
        panic!(
            "type {}: no constructor args found for copy constructor",
            type_names.pascal_case
        );
    }

    let construct_default = format_ident!("{}_construct_default", type_names.snake_case);
    let construct_copy = format_ident!("{}_construct_copy", type_names.snake_case);
    let construct_default_error = format_load_error(&construct_default);
    let construct_copy_error = format_load_error(&construct_copy);
    let variant_type = &type_names.sys_variant_type;

    let mut more_decls = Vec::with_capacity(constructors.len() - 2);
    let mut more_inits = Vec::with_capacity(constructors.len() - 2);
    for i in 2..constructors.len() {
        let ctor = &constructors[i];
        if let Some(args) = &ctor.arguments {
            let ident = if args.len() == 1 && args[0].name == "from" {
                let arg_type = &args[0].type_.to_case(Case::Snake);
                format_ident!("{}_from_{arg_type}", type_names.snake_case)
            } else {
                let arg_names = args.iter().map(|arg| &*arg.name).collect::<Vec<_>>();
                format_ident!("{}_from_{}", type_names.snake_case, arg_names.join("_"))
            };
            more_decls.push(quote! {
                pub #ident: unsafe extern "C" fn(GDNativeTypePtr, *const GDNativeTypePtr),
            });
            let i = i as i32;
            more_inits.push(quote! {
               #ident: {
                    let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                    ctor_fn(crate:: #variant_type, #i).expect(#construct_default_error)
                },
            });
        }
    }

    // Generic signature:  fn(base: GDNativeTypePtr, args: *const GDNativeTypePtr)
    let decls = quote! {
        pub #construct_default: unsafe extern "C" fn(GDNativeTypePtr, *const GDNativeTypePtr),
        pub #construct_copy: unsafe extern "C" fn(GDNativeTypePtr, *const GDNativeTypePtr),
        #(#more_decls)*
    };

    let inits = quote! {
        #construct_default: {
            let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
            ctor_fn(crate:: #variant_type, 0i32).expect(#construct_default_error)
        },
        #construct_copy: {
            let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
            ctor_fn(crate:: #variant_type, 1i32).expect(#construct_copy_error)
        },
        #(#more_inits)*
    };

    (decls, inits)
}

fn make_destroy_fns(type_names: &TypeNames, has_destructor: bool) -> (TokenStream, TokenStream) {
    if !has_destructor {
        return (TokenStream::new(), TokenStream::new());
    }

    let destroy = format_ident!("{}_destroy", type_names.snake_case);
    let variant_type = &type_names.sys_variant_type;

    let decls = quote! {
        pub #destroy: unsafe extern "C" fn(GDNativeTypePtr),
    };

    let inits = quote! {
        #destroy: {
            let dtor_fn = interface.variant_get_ptr_destructor.unwrap();
            dtor_fn(crate:: #variant_type).unwrap()
        },
    };
    (decls, inits)
}

fn format_load_error(ident: &Ident) -> String {
    format!("failed to load GDExtension function `{}`", ident)
}

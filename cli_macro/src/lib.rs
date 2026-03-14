//! 命令行参数解析过程宏 - 简化版 clap 风格

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Type, LitStr, Expr};

/// 为结构体派生命令行参数解析器
#[proc_macro_derive(Parser, attributes(command, arg))]
pub fn derive_parser(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => &data.fields,
        _ => panic!("Parser can only be derived for structs"),
    };

    // 解析 #[command(...)] 属性
    let mut command_name = String::new();
    let mut command_about = String::new();

    for attr in &input.attrs {
        if attr.path().is_ident("command") {
            attr.parse_nested_meta(|meta| {
                let ident = meta.path.get_ident().unwrap();
                match ident.to_string().as_str() {
                    "name" => {
                        let value: LitStr = meta.value()?.parse()?;
                        command_name = value.value();
                    }
                    "about" => {
                        let value: LitStr = meta.value()?.parse()?;
                        command_about = value.value();
                    }
                    _ => {}
                }
                Ok(())
            }).unwrap();
        }
    }

    let mut field_parse_branches = Vec::new();
    let mut field_assignments = Vec::new();
    let mut help_entries = Vec::new();
    let mut default_value_inserts = Vec::new();
    let mut conflict_checks = Vec::new();
    let mut field_decls = Vec::new();

    for field in fields.iter() {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        // 解析字段文档注释作为帮助文本
        let mut help = String::new();
        for attr in &field.attrs {
            if attr.path().is_ident("doc") {
                if let Ok(name_value) = attr.meta.require_name_value() {
                    if let Expr::Lit(expr_lit) = &name_value.value {
                        if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                            if !help.is_empty() {
                                help.push(' ');
                            }
                            help.push_str(&lit_str.value());
                        }
                    }
                }
            }
        }

        // 解析 #[arg(...)] 属性
        let mut short = None;
        let mut long = None;
        let mut default_value = None;
        let mut conflicts_with = None;

        // 检查是否是 Option 类型，并提取内部类型
        let mut is_option = false;
        let mut inner_type = field_type;
        if let Type::Path(type_path) = field_type {
            if let Some(segment) = type_path.path.segments.last() {
                if segment.ident == "Option" {
                    is_option = true;
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        if let Some(syn::GenericArgument::Type(ty)) = args.args.first() {
                            inner_type = ty;
                        }
                    }
                }
            }
        }

        let mut is_flag = false;
        for attr in &field.attrs {
            if attr.path().is_ident("arg") {
                attr.parse_nested_meta(|meta| {
                    let ident = meta.path.get_ident().unwrap();
                    match ident.to_string().as_str() {
                        "short" => {
                            if meta.input.peek(syn::Token![=]) {
                                let value: LitStr = meta.value()?.parse()?;
                                short = Some(value.value());
                            } else {
                                short = Some(field_name.to_string().chars().next().unwrap().to_string());
                            }
                        }
                        "long" => {
                            if meta.input.peek(syn::Token![=]) {
                                let value: LitStr = meta.value()?.parse()?;
                                long = Some(value.value());
                            } else {
                                long = Some(field_name.to_string().replace('_', "-"));
                            }
                        }
                        "default_value" => {
                            let value: LitStr = meta.value()?.parse()?;
                            default_value = Some(value.value());
                        }
                        "conflicts_with" => {
                            let value: LitStr = meta.value()?.parse()?;
                            conflicts_with = Some(value.value().replace('_', "-"));
                        }
                        "flag" => {
                            is_flag = true;
                        }
                        _ => {}
                    }
                    Ok(())
                }).unwrap();
            }
        }

        // 如果没有指定 long，使用字段名（将下划线转换为连字符）
        if long.is_none() {
            let long_name = field_name.to_string().replace('_', "-");
            long = Some(long_name);
        }

        let long_name = long.as_ref().unwrap();
        let field_name_str = field_name.to_string();

        // 判断是否是 bool 标志
        let is_bool = is_flag || matches!(field_type, Type::Path(p) if p.path.is_ident("bool"));

        // 判断是否是数字类型
        let is_numeric = matches!(inner_type, Type::Path(p) if {
            let id = p.path.segments.last().map(|s| s.ident.to_string());
            matches!(id.as_deref(), Some("u32" | "i32" | "u64" | "i64" | "usize" | "isize" | "f32" | "f64"))
        });

        // 声明字段变量
        if is_bool {
            field_decls.push(quote! { let mut #field_name: bool = false; });
        } else if is_numeric {
            field_decls.push(quote! { let mut #field_name: Option<#inner_type> = None; });
        } else {
            field_decls.push(quote! { let mut #field_name: Option<String> = None; });
        }

        // 生成 --long 解析分支
        let long_lit = proc_macro2::Literal::string(long_name);
        if is_bool {
            field_parse_branches.push(quote! {
                #long_lit => {
                    #field_name = true;
                    parsed_flags.insert(#long_lit.to_string());
                }
            });
        } else if is_numeric {
            // 数字类型：直接 parse 到目标类型
            field_parse_branches.push(quote! {
                #long_lit => {
                    let value = iter.next()
                        .ok_or_else(|| rust_learning::cli_parser::ParseError::MissingArgument(#field_name_str.to_string()))?;
                    #field_name = Some(value.parse::<#inner_type>().map_err(|_| rust_learning::cli_parser::ParseError::InvalidFormat(#field_name_str.to_string(), "invalid number".to_string()))?);
                    parsed_flags.insert(#long_lit.to_string());
                }
            });
        } else {
            // 带值的参数 (String)
            field_parse_branches.push(quote! {
                #long_lit => {
                    let value = iter.next()
                        .ok_or_else(|| rust_learning::cli_parser::ParseError::MissingArgument(#field_name_str.to_string()))?;
                    #field_name = Some(value);
                    parsed_flags.insert(#long_lit.to_string());
                }
            });
        }

        // 生成 -short 解析分支
        if let Some(short_name) = &short {
            let short_lit = proc_macro2::Literal::string(short_name);
            if is_bool {
                field_parse_branches.push(quote! {
                    #short_lit => {
                        #field_name = true;
                        parsed_flags.insert(#long_lit.to_string());
                    }
                });
            } else if is_numeric {
                field_parse_branches.push(quote! {
                    #short_lit => {
                        let value = iter.next()
                            .ok_or_else(|| rust_learning::cli_parser::ParseError::MissingArgument(#field_name_str.to_string()))?;
                        #field_name = Some(value.parse::<#inner_type>().map_err(|_| rust_learning::cli_parser::ParseError::InvalidFormat(#field_name_str.to_string(), "invalid number".to_string()))?);
                        parsed_flags.insert(#long_lit.to_string());
                    }
                });
            } else {
                field_parse_branches.push(quote! {
                    #short_lit => {
                        let value = iter.next()
                            .ok_or_else(|| rust_learning::cli_parser::ParseError::MissingArgument(#field_name_str.to_string()))?;
                        #field_name = Some(value);
                        parsed_flags.insert(#long_lit.to_string());
                    }
                });
            }
        }

        // 冲突检测
        if let Some(conflict) = &conflicts_with {
            let conflict_lit = proc_macro2::Literal::string(conflict);
            let long_lit_check = proc_macro2::Literal::string(long_name);
            conflict_checks.push(quote! {
                if parsed_flags.contains(#long_lit_check) && parsed_flags.contains(#conflict_lit) {
                    return Err(rust_learning::cli_parser::ParseError::ConflictError(#long_lit_check.to_string(), #conflict_lit.to_string()));
                }
            });
        }

        // 默认值处理
        if let Some(def_val) = &default_value {
            if is_option {
                let def_lit = proc_macro2::Literal::string(def_val);
                default_value_inserts.push(quote! {
                    if #field_name.is_none() {
                        #field_name = Some(#def_lit.to_string());
                    }
                });
            } else if is_numeric {
                // 数字类型的默认值需要解析
                let def_num: proc_macro2::TokenStream = def_val.parse().unwrap_or_else(|_| quote! { 0 });
                default_value_inserts.push(quote! {
                    if #field_name.is_none() {
                        #field_name = Some(#def_num);
                    }
                });
            } else if !is_bool {
                let def_lit = proc_macro2::Literal::string(def_val);
                default_value_inserts.push(quote! {
                    if #field_name.is_none() {
                        #field_name = Some(#def_lit.to_string());
                    }
                });
            }
        }

        // 字段赋值
        let field_name_lit = proc_macro2::Literal::string(&field_name_str);
        if is_bool {
            field_assignments.push(quote! {
                #field_name: #field_name
            });
        } else if is_option {
            field_assignments.push(quote! {
                #field_name: #field_name
            });
        } else if is_numeric {
            // 数字类型：Option<T> -> T
            field_assignments.push(quote! {
                #field_name: #field_name.ok_or_else(|| rust_learning::cli_parser::ParseError::MissingArgument(#field_name_lit.to_string()))?
            });
        } else {
            // 非 Option 非 bool 非数字：String 类型
            field_assignments.push(quote! {
                #field_name: #field_name.ok_or_else(|| rust_learning::cli_parser::ParseError::MissingArgument(#field_name_lit.to_string()))?
            });
        }

        // 帮助信息
        let mut opt_str = String::new();
        if let Some(s) = &short {
            opt_str.push_str(&format!("-{}", s));
        }
        if let Some(l) = &long {
            if !opt_str.is_empty() {
                opt_str.push_str(", ");
            }
            opt_str.push_str(&format!("--{}", l));
        }

        let opt_str_lit = proc_macro2::Literal::string(&opt_str);
        let mut full_help = help.clone();
        if let Some(d) = &default_value {
            full_help.push_str(&format!(" [默认: {}]", d));
        }
        let full_help_lit = proc_macro2::Literal::string(&full_help);
        help_entries.push(quote! {
            format!("  {:<25} {}", #opt_str_lit, #full_help_lit)
        });
    }

    let name_lit = proc_macro2::Literal::string(&command_name);
    let about_lit = proc_macro2::Literal::string(&command_about);

    let expanded = quote! {
        impl rust_learning::cli_parser::Parser for #name {
            fn parse_from<I, T>(iter: I) -> Result<Self, rust_learning::cli_parser::ParseError>
            where
                I: IntoIterator<Item = T>,
                T: Into<String>
            {
                let mut iter = iter.into_iter().map(|s| s.into()).peekable();

                // 声明所有字段变量
                #(#field_decls)*

                let mut parsed_flags = std::collections::HashSet::new();

                while let Some(arg) = iter.next() {
                    if arg == "--help" || arg == "-h" {
                        return Err(rust_learning::cli_parser::ParseError::MissingArgument("help".to_string()));
                    }
                    if arg.starts_with("--") {
                        let flag = &arg[2..];
                        match flag {
                            #(#field_parse_branches)*
                            _ => return Err(rust_learning::cli_parser::ParseError::UnknownArgument(arg)),
                        }
                    } else if arg.starts_with('-') && arg.len() > 1 {
                        let short_flag = &arg[1..];
                        match short_flag {
                            #(#field_parse_branches)*
                            _ => return Err(rust_learning::cli_parser::ParseError::UnknownArgument(arg)),
                        }
                    } else {
                        return Err(rust_learning::cli_parser::ParseError::UnknownArgument(arg));
                    }
                }

                // 冲突检测
                #(#conflict_checks)*

                // 默认值
                #(#default_value_inserts)*

                Ok(Self {
                    #(#field_assignments),*
                })
            }

            fn help() -> String {
                let mut help = String::new();

                if !#name_lit.is_empty() {
                    help.push_str(&format!("{}: {}\n\n", #name_lit, #about_lit));
                }

                help.push_str("用法:\n");
                help.push_str(&format!("  {} [选项]\n\n", #name_lit));

                help.push_str("选项:\n");
                #(
                    help.push_str(&#help_entries);
                    help.push('\n');
                )*

                help
            }
        }
    };

    TokenStream::from(expanded)
}

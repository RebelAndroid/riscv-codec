extern crate proc_macro;
use litrs::{IntegerLit};
use proc_macro::{Delimiter, TokenStream, TokenTree};

/// Assembles an i-type instruction
#[proc_macro]
pub fn i_assemble(input: TokenStream) -> TokenStream {
    if let TokenTree::Ident(i) = input.into_iter().next().unwrap() {
        let name = i.to_string();
        let lower = name.to_lowercase();
        format!(
            "
        if operands.len() != 3 {{
            Err(\"{lower} instruction requires 3 operands\".to_owned())
        }} else {{
            Ok(Instruction::{name}{{
                dest: IRegister::from_string(operands[0])?,
                src: IRegister::from_string(operands[1])?,
                imm: IImmediate::try_from(parse_int(operands[2])?)?,
            }})
        }}"
        )
        .parse()
        .unwrap()
    } else {
        panic!("expected identifier");
    }
}

/// Assembles an r-type instruction
#[proc_macro]
pub fn r_assemble(input: TokenStream) -> TokenStream {
    if let TokenTree::Ident(i) = input.into_iter().next().unwrap() {
        let name = i.to_string();
        let lower = name.to_lowercase();
        format!(
            "
        if operands.len() != 3 {{
            Err(\"{lower} instruction requires 3 operands\".to_owned())
        }} else {{
            Ok(Instruction::{name}{{
                dest: IRegister::from_string(operands[0])?,
                src1: IRegister::from_string(operands[1])?,
                src2: IRegister::from_string(operands[2])?,
            }})
        }}"
        )
        .parse()
        .unwrap()
    } else {
        panic!("expected identifier");
    }
}

/// Assembles a load type instruction
#[proc_macro]
pub fn l_assemble(input: TokenStream) -> TokenStream {
    if let TokenTree::Ident(i) = input.into_iter().next().unwrap() {
        let name = i.to_string();
        let lower = name.to_lowercase();
        format!(
            "
        if operands.len() != 2 {{
            Err(\"{lower} instruction requires 2 operands\".to_owned())
        }} else {{
            let (base, offset) = parse_address_expression(operands[1])?;
            Ok(Instruction::{name}{{
                dest: IRegister::from_string(operands[0])?,
                base,
                offset: IImmediate::try_from(offset)?,
            }})
        }}"
        )
        .parse()
        .unwrap()
    } else {
        panic!("expected identifier");
    }
}
/// Assembles a store instruction
#[proc_macro]
pub fn s_assemble(input: TokenStream) -> TokenStream {
    if let TokenTree::Ident(i) = input.into_iter().next().unwrap() {
        let name = i.to_string();
        let lower = name.to_lowercase();
        format!(
            "
        if operands.len() != 2 {{
            Err(\"{lower} instruction requires 2 operands\".to_owned())
        }} else {{
            let (base, offset) = parse_address_expression(operands[1])?;
            Ok(Instruction::{name}{{
                src: IRegister::from_string(operands[0])?,
                base,
                offset: SImmediate::from_val(offset),
            }})
        }}"
        )
        .parse()
        .unwrap()
    } else {
        panic!("expected identifier");
    }
}

/// Assembles a branc instruction
#[proc_macro]
pub fn b_assemble(input: TokenStream) -> TokenStream {
    // if operands.len() != 3 {
    //     Err("blt instruction requires 3 operands".to_owned())
    // } else {
    //     Ok(Instruction::BLT(
    //         IRegister::from_string(operands[0])?,
    //         IRegister::from_string(operands[1])?,
    //         parse_int(operands[2])? as i16,
    //     ))
    // }
    if let TokenTree::Ident(i) = input.into_iter().next().unwrap() {
        let name = i.to_string();
        let lower = name.to_lowercase();
        format!(
            "
        if operands.len() != 3 {{
            Err(\"{lower} instruction requires 3 operands\".to_owned())
        }} else {{
            Ok(Instruction::{name}{{
                src1: IRegister::from_string(operands[0])?,
                src2: IRegister::from_string(operands[1])?,
                offset: BImmediate::from_val(parse_int(operands[2])?),
            }})
        }}"
        )
        .parse()
        .unwrap()
    } else {
        panic!("expected identifier");
    }
}

/// Assembles a shift immediate instruction
#[proc_macro]
pub fn sh_assemble(input: TokenStream) -> TokenStream {
    if let TokenTree::Ident(i) = input.into_iter().next().unwrap() {
        let name = i.to_string();
        let lower = name.to_lowercase();
        format!(
            "
        if operands.len() != 3 {{
            Err(\"{lower} instruction requires 3 operands\".to_owned())
        }} else {{
            Ok(Instruction::{name}{{
                dest: IRegister::from_string(operands[0])?,
                src: IRegister::from_string(operands[1])?,
                shamt: Shamt::from_val(parse_int(operands[2])?),
            }})
        }}"
        )
        .parse()
        .unwrap()
    } else {
        panic!("expected identifier");
    }
}

/// Assembles a shift immediate word instruction
#[proc_macro]
pub fn shw_assemble(input: TokenStream) -> TokenStream {
    if let TokenTree::Ident(i) = input.into_iter().next().unwrap() {
        let name = i.to_string();
        let lower = name.to_lowercase();
        format!(
            "
        if operands.len() != 3 {{
            Err(\"{lower} instruction requires 3 operands\".to_owned())
        }} else {{
            Ok(Instruction::{name}{{
                dest: IRegister::from_string(operands[0])?,
                src: IRegister::from_string(operands[1])?,
                shamt: ShamtW::from_val(parse_int(operands[2])?),
            }})
        }}"
        )
        .parse()
        .unwrap()
    } else {
        panic!("expected identifier");
    }
}

#[proc_macro]
pub fn amo_assemble(input: TokenStream) -> TokenStream {
    if let TokenTree::Ident(i) = input.into_iter().next().unwrap() {
        let name = i.to_string();
        let lower = name.to_lowercase();
        let dname = name.clone() + "D";
        let wname = name.clone() + "W";
        let p = format!(
            "
            if mnemonics.len() == 1 {{
                Err(\"{lower} must have size (w/d)\".to_owned())
            }} else if mnemonics.len() == 2 {{
                if mnemonics[1] == \"w\" {{
                    Ok(Instruction::{wname}{{
                        dest: IRegister::from_string(operands[0])?,
                        addr: IRegister::from_string(operands[1])?,
                        src: IRegister::from_string(operands[2])?,
                        rl: false,
                        aq: false,
                }})
                }} else if mnemonics[1] == \"d\" {{
                    Ok(Instruction::{dname}{{
                        dest: IRegister::from_string(operands[0])?,
                        addr: IRegister::from_string(operands[1])?,
                        src: IRegister::from_string(operands[2])?,
                        rl: false,
                        aq: false,
                    }})
                }} else {{
                    Err(\"size of {lower} instruction must be word (w) or doubleword (d)\".to_owned())
                }}
            }} else if mnemonics.len() == 3 {{
                let (aq, rl) = match mnemonics[2] {{
                    \"\" => (false, false),
                    \"aq\" => (true, false),
                    \"rl\" => (false, true),
                    \"aqrl\" => (true, true),
                    _ => return Err(\"ordering should be (aq)(rl)\".to_owned()),
                }};
                if mnemonics[1] == \"w\" {{
                    Ok(Instruction::{wname}{{
                        dest: IRegister::from_string(operands[0])?,
                        addr: IRegister::from_string(operands[1])?,
                        src: IRegister::from_string(operands[2])?,
                        aq,
                        rl,
                    }})
                }} else if mnemonics[1] == \"d\" {{
                    Ok(Instruction::{dname}{{
                        dest: IRegister::from_string(operands[0])?,
                        addr: IRegister::from_string(operands[1])?,
                        src: IRegister::from_string(operands[2])?,
                        aq,
                        rl,
                    }})
                }} else {{
                    Err(\"size of {lower} isntruction must be word (w) or doubleword (d)\".to_owned())
                }}
            }} else {{
                Err(\"{lower} instruction has too many suffixes, expected {lower}.size.ordering\".to_owned())
            }}
        "
        );
        match p.parse() {
            Ok(t) => t,
            Err(_) => "".parse().unwrap(),
        }
    } else {
        panic!("expected identifier");
    }
}

// assembles an fr type instruction
#[proc_macro]
pub fn fr_assemble(input: TokenStream) -> TokenStream {
    if let TokenTree::Ident(i) = input.into_iter().next().unwrap() {
        let name = i.to_string();
        let sname = name.clone() + "S";
        let _dname = name.clone() + "D";
        let lower = name.to_lowercase();
        format!(
            "
        if operands.len() != 3 {{
                Err(\"{lower} instruction requires 3 operands\".to_owned())
        }} else {{
                if mnemonics.len() == 2 {{
                    Ok(Instruction::{sname}{{
                        dest: FRegister::try_from(operands[0])?,
                        src1: FRegister::try_from(operands[1])?,
                        src2: FRegister::try_from(operands[2])?,
                        rm: RoundingMode::DYN,
                    }})
        }}else if mnemonics.len() == 3 {{
                    Ok(Instruction::{sname}{{
                        dest: FRegister::try_from(operands[0])?,
                        src1: FRegister::try_from(operands[1])?,
                        src2: FRegister::try_from(operands[2])?,
                        rm: RoundingMode::from_str(mnemonics[2])?, 
                    }})
        }}else{{
                    Err(\"fadd instruction requires a suffix {{s,d}}\".to_owned())
        }}
        }}
            "
        )
        .parse()
        .unwrap()
    } else {
        panic!("expected identifier");
    }
}

/// Assembles a ci-type instruction
#[proc_macro]
pub fn ci_assemble(input: TokenStream) -> TokenStream {
    if let TokenTree::Ident(i) = input.into_iter().next().unwrap() {
        let name = i.to_string();
        let lower = name.to_lowercase();
        format!(
            "
        if operands.len() != 2 {{
            Err(\"c.{lower} instruction requires 2 operands\".to_owned())
        }} else {{
            Ok(CInstruction::{name}{{
                dest: IRegister::from_string(operands[0])?,
                imm: CIImmediate::from_val(parse_int(operands[1])?),
            }})
        }}"
        )
        .parse()
        .unwrap()
    } else {
        panic!("expected identifier");
    }
}

/// Assembles a cr-type instruction
#[proc_macro]
pub fn cr_assemble(input: TokenStream) -> TokenStream {
    if let TokenTree::Ident(i) = input.into_iter().next().unwrap() {
        let name = i.to_string();
        let lower = name.to_lowercase();
        format!(
            "
        if operands.len() != 2 {{
            Err(\"c.{lower} instruction requires 2 operands\".to_owned())
        }} else {{
            Ok(CInstruction::{name}{{
                dest: CIRegister::try_from(operands[0])?,
                src: CIRegister::try_from(operands[1])?,
            }})
        }}"
        )
        .parse()
        .unwrap()
    } else {
        panic!("expected identifier");
    }
}

// part of an immediate
#[derive(Clone, Debug, Copy)]
struct ImmPart {
    /// the beginning index of this part in the immediate value
    base: u8,
    /// the size of this immediate part
    size: u8,
    /// the location of this part in the *instruction*
    location: u8,
}

impl ImmPart {
    pub fn from_token_tree(input: TokenTree) -> Self {
        if let TokenTree::Group(g) = input {
            if g.delimiter() != Delimiter::Parenthesis {
                panic!("group delimiter should be Parenthesis");
            }
            let t: Vec<TokenTree> = g.stream().into_iter().collect();
            if t.len() != 3 {
                panic!("group should have 3 elements has {}", t.len());
            }
            let base: u8 = IntegerLit::try_from(t[0].clone()).unwrap().value().unwrap();
            let size: u8 = IntegerLit::try_from(t[1].clone()).unwrap().value().unwrap();
            let location: u8 = IntegerLit::try_from(t[2].clone()).unwrap().value().unwrap();
            ImmPart {
                base,
                size,
                location,
            }
        } else {
            panic!("expected group got {}", input.to_string());
        }
    }

    pub fn extract(&self, index: usize, input: &str) -> String {
        format!(
            "let part{index} = ({input} >> {}) & ((1 << {}) - 1);\n",
            self.location, self.size
        )
    }

    pub fn insert(&self, index: usize) -> String {
        if index == 0 {
            format!("(part{index} << {})", self.base)
        } else {
            format!(" | (part{index} << {})", self.base)
        }
    }
}

#[proc_macro]
pub fn make_immediate(input: TokenStream) -> TokenStream {
    let mut i = input.into_iter();
    if let TokenTree::Ident(ident) = i.next().unwrap() {
        let name = ident.to_string();
        let parts: Vec<ImmPart> = i.map(|t| ImmPart::from_token_tree(t)).collect();
        let struct_string = format!(
            "
            #[derive(Debug, PartialEq)]
            pub struct {name} {{
                val: i32,
            }}
        "
        );

        let impl_string = format!(
            "
        impl TryFrom<i64> for {name} {{
            type Error = String;

            fn try_from(value: i64) -> Result<Self, Self::Error> {{
                if value > 2i64.pow(11) - 1 || value < -2i64.pow(11) {{
                    Err(format!(\"attempted to construct out of range {name}\"))
                }}else {{
                    Ok({name} {{ val: value as i32 }})
                }}
            }}
        }}

        impl Into<i64> for {name} {{
            fn into(self) -> i64 {{
                self.val.into()
            }}
        }}
        "
        );

        //     pub fn from_u32(x: u32) -> Self {
        //         let a = (x >> 12) & 0b1111_1111;
        //         let b = (x >> 20) & 0b1;
        //         let c = (x >> 21) & 0b11_1111_1111;
        //         let d = x >> 31;
        //         let i: i32 = ((c << 1) | (b << 11) | (a << 12) | (d << 20)) as i32;
        //         // sign extend 21 bit value
        //         let i2: i32 = (i << 11) >> 11;
        //         JImmediate { val: i2 }
        //     }

        let extract_fn = {
            let extractions: String = parts
                .iter()
                .enumerate()
                .map(|(i, part)| part.extract(i, "x"))
                .collect();

            let insertions: String = parts
                .iter()
                .enumerate()
                .map(|(i, part)| part.insert(i))
                .collect();

            let insert = format!("let i: i32 = ({insertions}) as i32;");

            let total_size: u8 = parts
                .iter()
                .map(|part| part.size)
                .reduce(|acc, e| acc + e)
                .unwrap();
            let sign_extension = format!(
                "let i2: i32 = (i << {}) >> {};",
                32 - total_size,
                32 - total_size
            );

            let ret = format!("{name} {{ val: i2}}");

            format!(
                "
            impl {name} {{
                pub fn from_u32(x: u32) -> Self {{
                    {extractions}
                    {insert}
                    {sign_extension}
                    {ret}
                }}
            }}
            "
            )
        };

        let display_string = format!(
            "
            impl Display for {name} {{
                fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {{
                    write!(f, \"{{}}\", self.val)
                }}
            }}"
        );

        let final_str = format!(
            "
            {struct_string}
            {impl_string}
            {extract_fn}
            {display_string}
            "
        );


        // println!("{}", final_str);

        final_str.parse().unwrap()
    } else {
        panic!("first token should be an Identifier")
    }
}

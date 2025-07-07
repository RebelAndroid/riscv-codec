extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};

// #[proc_macro]
// pub fn r_print(input: TokenStream) -> TokenStream {
//     // Instruction::ADD(rd, rs1, rs2) => write!(f, "add {rd},{rs1},{rs2}"),
//     if let TokenTree::Ident(i) = input.into_iter().next().unwrap() {
//         let name = i.to_string();
//         let lower = name.to_lowercase();
//         format!("Instruction::{name}(rd, rs1, rs2) => write!(f, \"{{rd}},{{rs1}},{{rs2}}\")")
//             .parse()
//             .unwrap()
//     } else {
//         panic!("expected identifier");
//     }
// }

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
            Ok(Instruction::{name}(
                IRegister::from_string(operands[0])?,
                IRegister::from_string(operands[1])?,
                IImmediate::from_val(parse_int(operands[2])?),
            ))
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
            Ok(Instruction::{name}(
                IRegister::from_string(operands[0])?,
                IRegister::from_string(operands[1])?,
                IRegister::from_string(operands[2])?,
            ))
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
            Ok(Instruction::{name}(
                IRegister::from_string(operands[0])?,
                base,
                IImmediate::from_val(offset),
            ))
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
            Ok(Instruction::{name}(
                base,
                IRegister::from_string(operands[0])?,
                SImmediate::from_val(offset),
            ))
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
            Ok(Instruction::{name}(
                IRegister::from_string(operands[0])?,
                IRegister::from_string(operands[1])?,
                BImmediate::from_val(parse_int(operands[2])?),
            ))
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
            Ok(Instruction::{name}(
                IRegister::from_string(operands[0])?,
                IRegister::from_string(operands[1])?,
                Shamt::from_val(parse_int(operands[2])?),
            ))
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
            Ok(Instruction::{name}(
                IRegister::from_string(operands[0])?,
                IRegister::from_string(operands[1])?,
                ShamtW::from_val(parse_int(operands[2])?),
            ))
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
                    Ok(Instruction::{wname}(
                        IRegister::from_string(operands[0])?,
                        IRegister::from_string(operands[1])?,
                        IRegister::from_string(operands[2])?,
                        false,
                        false,
                    ))
                }} else if mnemonics[1] == \"d\" {{
                    Ok(Instruction::{dname}(
                        IRegister::from_string(operands[0])?,
                        IRegister::from_string(operands[1])?,
                        IRegister::from_string(operands[2])?,
                        false,
                        false,
                    ))
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
                    Ok(Instruction::{wname}(
                        IRegister::from_string(operands[0])?,
                        IRegister::from_string(operands[1])?,
                        IRegister::from_string(operands[2])?,
                        aq,
                        rl,
                    ))
                }} else if mnemonics[1] == \"d\" {{
                    Ok(Instruction::{dname}(
                        IRegister::from_string(operands[0])?,
                        IRegister::from_string(operands[1])?,
                        IRegister::from_string(operands[2])?,
                        aq,
                        rl,
                    ))
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
                    Ok(Instruction::{sname}(
                        FRegister::from_string(operands[0])?,
                        FRegister::from_string(operands[1])?,
                        FRegister::from_string(operands[2])?,
                        RoundingMode::DYN,
                    ))
        }}else if mnemonics.len() == 3 {{
                    Ok(Instruction::{sname}(
                        FRegister::from_string(operands[0])?,
                        FRegister::from_string(operands[1])?,
                        FRegister::from_string(operands[2])?,
                        RoundingMode::from_str(mnemonics[2])?, 
                    ))
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
            Ok(CInstruction::{name}(
                IRegister::from_string(operands[0])?,
                CIImmediate::from_val(parse_int(operands[1])?),
            ))
        }}"
        )
        .parse()
        .unwrap()
    } else {
        panic!("expected identifier");
    }
}
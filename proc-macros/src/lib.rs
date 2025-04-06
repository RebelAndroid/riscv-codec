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
                parse_int(operands[2])? as i16,
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

mod sm83;

use sm83::{RegisterType, Register8, Register16, RegisterValue, RegisterFile, Flag};

fn main() {
    let mut rf = RegisterFile::new();
    println!("{}", rf);

    assert_eq!(rf.get_register(RegisterType::R8(Register8::A)), RegisterValue::Value8(0u8));
    assert_eq!(rf.get_register(RegisterType::R16(Register16::Pc)), RegisterValue::Value16(0u16));
    assert_eq!(rf.get_flag(Flag::Carry), false);
    assert_eq!(rf.get_flag(Flag::Zero), false);

    rf.set_register8(Register8::A, 0x10);
    println!("{}", rf);
    assert_eq!(rf.get_register(RegisterType::R8(Register8::A)), RegisterValue::Value8(0x10u8));
    rf.set_flag(Flag::Carry, true);
    rf.set_flag(Flag::Zero, true);
    println!("{}", rf);
    rf.set_flag(Flag::Zero, false);
    println!("{}", rf);
}

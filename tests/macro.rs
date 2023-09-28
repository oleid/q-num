use q_num::define_q_num;

#[test]
fn test_macro_q12p5() {
    define_q_num!(X, Q12.5);
    assert_eq!(X::TOTAL_BITS, 32);
    assert_eq!(std::mem::size_of::<X>(), 4);
    assert_eq!(X::USED_BITS, 17);
    assert_eq!(X::INT_BITS, 12);
    assert_eq!(X::FRAC_BITS, 5);
    assert_eq!(X::PAD_BITS, 15);
    assert_eq!(X::DENOMINATOR, 32.0);
    assert_eq!(X::CONVERSION_FACTOR, 1_048_576.0);
    assert_eq!(X::USED_MASK as u32, 0xFFFF8000);
    assert_eq!(X::MIN_FLOAT, -2048.0);
    assert_eq!(X::MAX_FLOAT, 2047.96875);
}

#[test]
fn test_macro_add() {
    define_q_num!(X, Q12.5);
    let x1 = X::from(500.25);
    let x2 = X::from(-744.75);
    let x3 = X::from(-244.5);
    let b1 = x1.to_bits();
    let b2 = x2.to_bits();
    let b3 = x3.to_bits();
    assert_eq!(b1 + b2, b3);
    assert_eq!(x1 + x2, x3)
}

#[test]
fn test_shadowed_type() {
    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    type u8 = u16;
    // If the macro is non-hygienic, it will use the shadowed type.
    // If the macro is hygienic, it will use `core::primitive::u8`.
    define_q_num!(X, Q4.2);
    // The correct size is 1 bytes = 8 bits.
    assert_eq!(std::mem::size_of::<X>(), 1);
}

#[test]
fn test_q4p2() {
    define_q_num!(Q, UQ4.2);
    let mut value = 0.0;
    for byte in (0..0b1111_1100).step_by(0b100) {
        let q = Q::from(value);
        assert_eq!(q.to_bits(), byte);
        value += 0.25;
    }
}

#[test]
#[should_panic]
fn test_q4p2_out_of_range() {
    define_q_num!(Q, Q4.2);
    let _ = Q::from(16.00);
}

#[test]
fn test_q5p3() {
    define_q_num!(Q, UQ5.3);
    let mut value = 0.0;
    for byte in 0..0b1111_1111 {
        let q = Q::from(value);
        assert_eq!(q.to_bits(), byte);
        value += 0.125;
    }
}

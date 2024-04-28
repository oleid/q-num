use q_num::define_q_num;

/// Spot check the macro-generated constants for `Q12.5`.
#[test]
fn test_q12p5_constants() {
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

/// Spot check addition for `Q12.5`.
#[test]
fn test_q12p5_add() {
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

/// Spot check the macro-generated constants for `Q1.6`.
#[test]
fn test_q1p6_constants() {
    define_q_num!(X, Q1.6);
    assert_eq!(X::TOTAL_BITS, 8);
    assert_eq!(std::mem::size_of::<X>(), 1);
    assert_eq!(X::USED_BITS, 7);
    assert_eq!(X::INT_BITS, 1);
    assert_eq!(X::FRAC_BITS, 6);
    assert_eq!(X::PAD_BITS, 1);
    assert_eq!(X::DENOMINATOR, 64.0);
    assert_eq!(X::CONVERSION_FACTOR, 128.0);
    assert_eq!(X::USED_MASK as u8, 0xFE);
    assert_eq!(X::MIN_FLOAT, -1.0);
    assert_eq!(X::MAX_FLOAT, 0.984375);
}

/// Spot check the macro-generated constants for `Q1.7`.
#[test]
fn test_q1p7_constants() {
    define_q_num!(X, Q1.7);
    assert_eq!(X::TOTAL_BITS, 8);
    assert_eq!(std::mem::size_of::<X>(), 1);
    assert_eq!(X::USED_BITS, 8);
    assert_eq!(X::INT_BITS, 1);
    assert_eq!(X::FRAC_BITS, 7);
    assert_eq!(X::PAD_BITS, 0);
    assert_eq!(X::DENOMINATOR, 128.0);
    assert_eq!(X::CONVERSION_FACTOR, 128.0);
    assert_eq!(X::USED_MASK as u8, 0xFF);
    assert_eq!(X::MIN_FLOAT, -1.0);
    assert_eq!(X::MAX_FLOAT, 0.9921875);
}

/// Test that the `define_q_num` macro is hygienic.
///
/// - If the macro is non-hygienic, it may refer to an unqualified `u8`. This is
///   a problem because `u8` could be shadowed. In the test below, it is
///   shadowed to refer to `u16`, which would result in an incorrect size of 2
///   bytes.
/// - If the macro is hygienic, it will correctly refer to the full qualified
///   type: `::core::primitive::u8`.
#[test]
fn test_q4p2_hygienic_inner_type() {
    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    type u8 = u16;
    define_q_num!(X, Q4.2);
    // The correct size is 1 bytes = 8 bits.
    assert_eq!(std::mem::size_of::<X>(), 1);
}

/// Exhaustively test all valid inner values for `UQ5.3`.
#[test]
fn test_q5p3_inner_values() {
    define_q_num!(Q, UQ5.3);
    let mut value = 0.0;
    for byte in 0..0b1111_1111 {
        let q = Q::from(value);
        assert_eq!(q.to_bits(), byte);
        value += 0.125;
    }
}

/// Exhaustively test all valid inner values for `UQ4.2`. In this case where
/// only 6 of 8 bits are used, the inner values are left-aligned, leaving the
/// rightmost 2 bits unused.
#[test]
fn test_q4p2_inner_values() {
    define_q_num!(Q, UQ4.2);
    let mut value = 0.0;
    for byte in (0..0b1111_1100).step_by(0b100) {
        let q = Q::from(value);
        assert_eq!(q.to_bits(), byte);
        value += 0.25;
    }
}

/// Spot check an out-of-range input for `Q4.2`.
#[test]
#[should_panic]
fn test_q4p2_out_of_range() {
    define_q_num!(Q, Q4.2);
    let _ = Q::from(16.00);
}

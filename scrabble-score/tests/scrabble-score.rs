extern crate scrabble_score;

use scrabble_score::*;

#[test]
fn a_is_worth_one_point() {
    assert_eq!(score("a"), 1);
}

#[test]
fn scoring_is_case_insensitive() {
    assert_eq!(score("A"), 1);
}

#[test]
fn f_is_worth_four() {
    assert_eq!(score("f"), 4);
}

#[test]
fn two_one_point_letters_make_a_two_point_word() {
    assert_eq!(score("at"), 2);
}

#[test]
fn three_letter_word() {
    assert_eq!(score("zoo"), 12);
}

#[test]
fn medium_word() {
    assert_eq!(score("street"), 6);
}

#[test]
fn longer_words_with_valuable_letters() {
    assert_eq!(score("quirky"), 22);
}

#[test]
fn long_mixed_case_word() {
    assert_eq!(score("OxyphenButazone"), 41);
}

#[test]
fn non_english_scrabble_letters_do_not_score() {
    assert_eq!(score("pinata"), 8);
    assert_eq!(score("piñata"), 7);
}

#[test]
fn empty_words_are_worth_zero() {
    assert_eq!(score(""), 0);
}

#[test]
fn all_letters_work() {
    assert_eq!(score("abcdefghijklmnopqrstuvwxyz"), 87);
}

// =============================================
//  Extensions
//
//  The tests below correspond to the optional
//  extensions described in the README
// =============================================

#[test]
fn single_letter_double_score() {
    assert_eq!(score("a:double"), 2);
}

#[test]
fn other_single_letter_double_score() {
    assert_eq!(score("g:double"), 4);
}

#[test]
fn double_scoring_letter_in_a_word() {
    assert_eq!(score("zo:doubleo"), 13);
}

#[test]
fn double_letters_at_word_boundary() {
    assert_eq!(score("fabulous"), 13);
    assert_eq!(score("f:doubleabulous"), 17);
    assert_eq!(score("fabulous:double"), 14);
}

#[test]
fn single_letter_triple_score() {
    assert_eq!(score("a:triple"), 3);
}

#[test]
fn other_single_letter_triple_score() {
    assert_eq!(score("g:triple"), 6);
}

#[test]
fn triple_scoring_letter_in_a_word() {
    assert_eq!(score("zo:tripleo"), 14);
}

#[test]
fn triple_letters_at_word_boundary() {
    assert_eq!(score("fabulous"), 13);
    assert_eq!(score("f:tripleabulous"), 21);
    assert_eq!(score("fabulous:triple"), 15);
}

#[test]
fn zero_scoring_letters_ignored() {
    assert_eq!(score("pinata"), 8);
    assert_eq!(score("piñata"), 7);
    assert_eq!(score("piñ:doubleata"), 7);
    assert_eq!(score("piñ:tripleata"), 7);
}

extern crate pangram;

use pangram::*;

#[test]
fn empty_strings_are_not_pangrams() {
    let sentence = "";
    assert!(!is_pangram(&sentence));
}

#[test]
fn classic_pangram_is_a_pangram() {
    let sentence = "the quick brown fox jumps over the lazy dog";
    assert!(is_pangram(&sentence));
}

#[test]
fn pangrams_must_have_all_letters() {
    let sentence = "a quick movement of the enemy will jeopardize five gunboats";
    assert!(!is_pangram(&sentence));
}

#[test]
fn pangrams_must_have_all_letters_two() {
    let sentence = "the quick brown fish jumps over the lazy dog";
    assert!(!is_pangram(&sentence));
}

#[test]
fn pangrams_must_include_z() {
    let sentence = "the quick brown fox jumps over the lay dog";
    assert!(!is_pangram(&sentence));
}

#[test]
fn underscores_do_not_affect_pangrams() {
    let sentence = "the_quick_brown_fox_jumps_over_the_lazy_dog";
    assert!(is_pangram(&sentence));
}

#[test]
fn numbers_do_not_affect_pangrams() {
    let sentence = "the 1 quick brown fox jumps over the 2 lazy dogs";
    assert!(is_pangram(&sentence));
}

#[test]
fn numbers_can_not_replace_letters() {
    let sentence = "7h3 qu1ck brown fox jumps ov3r 7h3 lazy dog";
    assert!(!is_pangram(&sentence));
}

#[test]
fn capitals_and_punctuation_can_be_in_pangrams() {
    let sentence = "\"Five quacking Zephyrs jolt my wax bed.\"";
    assert!(is_pangram(&sentence));
}

#[test]
fn non_ascii_characters_can_be_in_pangrams() {
    let sentence = "Victor jagt zwölf Boxkämpfer quer über den großen Sylter Deich.";
    assert!(is_pangram(&sentence));
}

#[test]
fn every_single_alphabet_letter_should_be_present() {
    assert!(!is_pangram(" bcdefghijklmnopqrstuvwxyz"));
    assert!(!is_pangram("a cdefghijklmnopqrstuvwxyz"));
    assert!(!is_pangram("ab defghijklmnopqrstuvwxyz"));
    assert!(!is_pangram("abc efghijklmnopqrstuvwxyz"));
    assert!(!is_pangram("abcd fghijklmnopqrstuvwxyz"));
    assert!(!is_pangram("abcde ghijklmnopqrstuvwxyz"));
    assert!(!is_pangram("abcdef hijklmnopqrstuvwxyz"));
    assert!(!is_pangram("abcdefg ijklmnopqrstuvwxyz"));
    assert!(!is_pangram("abcdefgh jklmnopqrstuvwxyz"));
    assert!(!is_pangram("abcdefghi klmnopqrstuvwxyz"));
    assert!(!is_pangram("abcdefghij lmnopqrstuvwxyz"));
    assert!(!is_pangram("abcdefghijk mnopqrstuvwxyz"));
    assert!(!is_pangram("abcdefghijkl nopqrstuvwxyz"));
    assert!(!is_pangram("abcdefghijklm opqrstuvwxyz"));
    assert!(!is_pangram("abcdefghijklmn pqrstuvwxyz"));
    assert!(!is_pangram("abcdefghijklmno qrstuvwxyz"));
    assert!(!is_pangram("abcdefghijklmnop rstuvwxyz"));
    assert!(!is_pangram("abcdefghijklmnopq stuvwxyz"));
    assert!(!is_pangram("abcdefghijklmnopqr tuvwxyz"));
    assert!(!is_pangram("abcdefghijklmnopqrs uvwxyz"));
    assert!(!is_pangram("abcdefghijklmnopqrst vwxyz"));
    assert!(!is_pangram("abcdefghijklmnopqrstu wxyz"));
    assert!(!is_pangram("abcdefghijklmnopqrstuv xyz"));
    assert!(!is_pangram("abcdefghijklmnopqrstuvw yz"));
    assert!(!is_pangram("abcdefghijklmnopqrstuvwx z"));
    assert!(!is_pangram("abcdefghijklmnopqrstuvwxy "));
}

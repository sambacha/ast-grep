#![cfg(test)]
//! Standalone Solidity file to test syntax.
//! Solidity does not need special processing and can be a stub lang.

use ast_grep_core::source::TSParseError;

use super::*;

fn test_replace(src: &str, pattern: &str, replacer: &str) -> Result<String, TSParseError> {
    use crate::test::test_replace_lang;
    test_replace_lang(src, pattern, replacer, Solidity)
}

fn test_match(query: &str, source: &str) {
  use crate::test::test_match_lang;
  test_match_lang(query, source, Solidity);
}

fn test_non_match(query: &str, source: &str) {
  use crate::test::test_non_match_lang;
  test_non_match_lang(query, source, Solidity);
}


#[test]
fn test_solidity_replace() -> Result<(), TSParseError> {
    let ret = test_replace(
      "token.transfer(recipient)",
      "$CONTRACT.transfer($ADDR)",
      "$CONTRACT.safeTransfer($ADDR)",
    )?;
    assert_eq!(ret, "token.safeTransfer(recipient)");
    Ok(())
}


#[test]
fn test_solidity_basic() {
  // Basic patterns
  test_match("uint256 $VAR", "uint256 x");
  test_match("address $VAR", "address recipient");
  test_match("$VAR = msg.sender", "owner = msg.sender");
  test_match("$VAR = block.timestamp", "time = block.timestamp");
}

#[test]
fn test_solidity_functions() {
  test_match(
      "function $NAME() public { $BODY }",
      "function getValue() public { return x; }"
  );
  test_match(
      "function $NAME() public returns (uint256)",
      "function getValue() public returns (uint256)"
  );
}

#[test]
fn test_solidity_non_matches() {
  test_non_match("uint256 $VAR", "uint128 value");
  test_non_match("function $NAME() public", "function getValue() private");
}
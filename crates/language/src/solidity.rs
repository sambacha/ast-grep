#![cfg(test)]
use super::*;
use ast_grep_core::source::TSParseError;

// Helper functions for testing, following the pattern from other language modules
fn test_match(query: &str, source: &str) {
  use crate::test::test_match_lang;
  test_match_lang(query, source, Solidity);
}

fn test_non_match(query: &str, source: &str) {
  use crate::test::test_non_match_lang;
  test_non_match_lang(query, source, Solidity);
}

fn test_replace(src: &str, pattern: &str, replacer: &str) -> Result<String, TSParseError> {
  use crate::test::test_replace_lang;
  test_replace_lang(src, pattern, replacer, Solidity)
}

#[test]
fn test_solidity_basic() {
  // Test basic contract structure
  test_match(
    "contract $NAME { $$ }",
    "contract Token { uint256 totalSupply; }",
  );

  // Test function declarations
  test_match(
    "function $NAME($PARAMS) $VISIBILITY $MUTABILITY { $$ }",
    "function transfer(address to, uint256 amount) public pure { return true; }",
  );
}

#[test]
fn test_solidity_expressions() {
  // Test variable declarations
  test_match("$TYPE $NAME = $VALUE;", "uint256 balance = 100;");

  // Test modifiers
  test_match(
    "modifier $NAME($PARAMS) { $$ }",
    "modifier onlyOwner() { require(msg.sender == owner); _; }",
  );
}

#[test]
fn test_solidity_replace() -> Result<(), TSParseError> {
  // Test replacing require statements with custom errors
  let ret = test_replace(
    "require(balance >= amount, 'Insufficient balance');",
    "require($CONDITION, $MESSAGE);",
    "if (!$CONDITION) revert InsufficientBalance();",
  )?;

  assert_eq!(ret, "if (!balance >= amount) revert InsufficientBalance();");

  // Test replacing old-style constructor with new syntax
  let ret = test_replace(
    "function Token() public { owner = msg.sender; }",
    "function $NAME() $VISIBILITY { $$ }",
    "constructor() $VISIBILITY { $$ }",
  )?;

  assert_eq!(ret, "constructor() public { owner = msg.sender; }");

  Ok(())
}

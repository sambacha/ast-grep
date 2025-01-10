#![cfg(test)]
//! Standalone Solidity file to test syntax.
//! Solidity does not need special processing and can be a stub lang.

use ast_grep_core::source::TSParseError;

use super::*;

fn test_replace(src: &str, pattern: &str, replacer: &str) -> Result<String, TSParseError> {
    use crate::test::test_replace_lang;
    test_replace_lang(src, pattern, replacer, Solidity)
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
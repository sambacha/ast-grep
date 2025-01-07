#![cfg(test)]
use ast_grep_core::source::TSParseError;

use super::*;

fn test_match(query: &str, source: &str) {
    use crate::test::test_match_lang;
    test_match_lang(query, source, Solidity);
}

fn test_non_match(query: &str, source: &str) {
    use crate::test::test_non_match_lang;
    test_non_match_lang(query, source, Solidity);
}

#[test]
fn test_solidity_basic() {
    // Test contract declaration matching
    test_match("contract $NAME {}", "contract Token {}");
    test_match("contract $NAME is $BASE {}", "contract Token is ERC20 {}");
    
    // Test function declaration matching
    test_match(
        "function $NAME() public {}",
        "function transfer() public {}"
    );
    test_non_match(
        "function transfer() public {}",
        "function approve() public {}"
    );
}

#[test]
fn test_solidity_modifiers() {
    test_match(
        "modifier $NAME() { _; }",
        "modifier onlyOwner() { _; }"
    );
    test_match(
        "modifier $NAME($PARAM) { _; }",
        "modifier costs(uint price) { _; }"
    );
}

#[test]
fn test_solidity_events() {
    test_match(
        "event $NAME($PARAMS);",
        "event Transfer(address indexed from, address indexed to, uint256 value);"
    );
    test_non_match(
        "event Transfer($PARAMS);",
        "event Approval(address owner, address spender, uint256 value);"
    );
}

#[test]
fn test_solidity_state_variables() {
    test_match(
        "uint256 public $NAME;",
        "uint256 public totalSupply;"
    );
    test_match(
        "mapping($KEY => $VALUE) public $NAME;",
        "mapping(address => uint256) public balanceOf;"
    );
}

fn test_replace(src: &str, pattern: &str, replacer: &str) -> Result<String, TSParseError> {
    use crate::test::test_replace_lang;
    test_replace_lang(src, pattern, replacer, Solidity)
}

#[test]
fn test_solidity_replace() -> Result<(), TSParseError> {
    // Test replacing require with custom error
    let ret = test_replace(
        "require(msg.sender == owner);",
        "require($CONDITION);",
        "if (!$CONDITION) revert Unauthorized();"
    )?;
    assert_eq!(ret, "if (!msg.sender == owner) revert Unauthorized();");

    // Test replacing function modifier
    let ret = test_replace(
        "function transfer() public {",
        "function $NAME() public {",
        "function $NAME() public nonReentrant {"
    )?;
    assert_eq!(ret, "function transfer() public nonReentrant {");
    
    Ok(())
}
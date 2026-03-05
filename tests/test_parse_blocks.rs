use mineclaw::tools::filesystem::parse_search_replace_blocks;

fn main() {
    // Test 1: Simple string (no blocks)
    let search1 = "foo";
    let replace1 = "FOO";
    let blocks1 = parse_search_replace_blocks(search1, replace1);
    println!("Test 1 - Simple string:");
    println!("  Blocks: {:?}", blocks1);
    assert_eq!(blocks1.len(), 1);
    assert_eq!(blocks1[0], ("foo".to_string(), "FOO".to_string()));

    // Test 2: SEARCH/REPLACE blocks
    let search2 = r#"
------- SEARCH
Line A: foo
=======
Line A: FOO
+++++++ REPLACE

------- SEARCH
Line C: foo
=======
Line C: FOO
+++++++ REPLACE
"#;
    let blocks2 = parse_search_replace_blocks(search2, "");
    println!("\nTest 2 - SEARCH/REPLACE blocks:");
    println!("  Blocks: {:?}", blocks2);
    assert_eq!(blocks2.len(), 2);
    assert_eq!(blocks2[0], ("Line A: foo".to_string(), "Line A: FOO".to_string()));
    assert_eq!(blocks2[1], ("Line C: foo".to_string(), "Line C: FOO".to_string()));

    println!("\n✅ All tests passed!");
}
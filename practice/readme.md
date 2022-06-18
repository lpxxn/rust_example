cargo run --bin r3array

cargo test -- --help
cargo test --bin r7trait d1::t1::test_pari -- --show-output 

Using cargo test test-name filters tests that contain test-name. It is possible that it can run multiple tests. It doesn't matter if the test function is in some mod or not, it still able to execute multiple test.

You can avoid this by adding -- --exact as argument.

If your test is not in any mod you can simply execute like this:

cargo test test_fn_name -- --exact
Otherwise you need to provide test with full namespace:

cargo test test_mod_name::test_fn_name -- --exact
For your case the solution will be :

cargo test --package school_info repeat_students_should_not_get_full_marks -- --exact
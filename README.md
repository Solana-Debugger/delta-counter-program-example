# Solana Debugger: Delta counter program example

This is a simple counter program, meant as an example program for the Solana Debugger.

The `debug_input` folder contains sample inputs that can be used to test the debugger.

The program includes a `save_input.rs` module which allows you to use integration tests to generate debugger inputs. For example, like this:

```
cd delta-counter

cargo-test-sbf test_create_counter --test test_counter -- --exact --nocapture

cargo-test-sbf test_increase_counter --test test_counter -- --exact --nocapture
```

The generated inputs are stored in `delta-counter/debug_input`

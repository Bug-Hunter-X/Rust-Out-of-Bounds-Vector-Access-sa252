# Rust Out-of-Bounds Vector Access Example

This repository demonstrates a common error in Rust: accessing an element in a vector using an index that is out of bounds.  The `bug.rs` file shows the erroneous code, while `bugSolution.rs` provides a corrected version.

The error occurs because the program attempts to access an index beyond the valid range of the vector, which results in a runtime panic.

The solution involves carefully checking the vector's length before accessing elements or using safer methods like `get()` which returns an `Option`. 
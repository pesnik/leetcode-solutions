# CLI

```sh
cargo test p1071
cargo test p1071 -- --nocapture
```

## 11. Container With Most Water
My first approach was to select the next bar on the following condition:
```rust
    // capacity when left + 1 bar selected
    // capacity when right - 1 bar selected
    match left_moving_cap.cmp(right_moving_cap) {
        std::cmp::Ordering::Greater => move_left,
        _ => move_right
    }
```
> This algorithm makes locally optimal decisions that might miss the globally optimal solution.

## 1679. Max Number of K-Sum Pairs
Sort + Two Pointer. Too Slow! Beats only 22% ac submissions.

## 643. Maximum Average Subarray I
A simple sliding window problem, but solution isn't so idiomatic rusty.

## 1456. Maximum Number of Vowels in a Substring of Given Length
A non-optimized solution based on sliding window, redundent code and operations. Beats 37.71%!

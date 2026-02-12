# 2. <a href="https://leetcode.com/problems/add-two-numbers/description/"> Add Two Numbers </a>

## 📝 Description

You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.

## 🧠 How I solved the problem 

I reversed both input lists, then iterated through the elements and converted them to **strings**. After concatenating them, I parsed the final **string** back into an **i32** to get the full number. I did this for both lists, added the two numbers together, and finally converted the sum back to a reversed list of characters.

## ➗ Complexity

* **Time complexity**: *O(n)* – Each element is compared with every other element.
* **Space complexity**: *O(1)* – No extra data structures are used.

## 📊 Benchmark

I made it in release mode for more accurate results:
```bash
cargo run --release
```

Hardware: *Apple Mac Mini M4*

### 🤏 Small Input Test 

* **Execution Time**: *39.333µs*
* **Memory Delta**:   *0 bytes*
* **Current Memory**: *1441792 bytes*

### 😖 Stress Test (Large Input)

***This is the problem with this algorithm***

### 🤬 The Problem

* I didn't used the LeetCode provided code spiece

```
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>
    }

    impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
            }
        }
    }  
```
That's why this program, collapses when I add a bigger list.
My program is always overflowing because of the <a href="https://doc.rust-lang.org/book/ch03-02-data-types.html">Rust integer limitations. </a>


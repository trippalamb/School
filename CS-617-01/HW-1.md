# Homework 1

Tripp Lamb

2026 Jan 22

CS 617-01

## Problem 1

Page 41, Problem 2.4 Inversion numbers. you only need to present the pseudo-code part. Please refer to https://www.cs.mcgill.ca/~akroit/math/compsci/Cormen%20Introduction%20to%20Algorithms.pdf

## Problem 2

Present a recursive pseudo-code for insertion sort

```
insertion_sort_iter(A)
  // `A` is an index-1 based array of numbers
  // `n` is len(A)
  
  for(ki=2; ki<=n; ki++) //key index
    key = A[ki]
    si = ki //sort index
    while si > 1 && key < A[si-1]
        A[si] = A[si-1]
        si -= 1
    A[si] = key

```

```
// `A` is an index-1 based array of numbers
// `n` is len(A)
top-level call: insertion_sort_rec(A, n)

insertion_sort_rec(A, si)

  if si < n
    key = A[si]
    insertion_sort_rec(A, si+1)
    i = si
    while i < n && key > A[i+1]
      A[i] = A[i+1]
      i += 1
    A[i] = key
```



## Problem 3

Consider the Hanoi's Tower pseudocode presented in class, write a piece of pseudocode to print the k-th step when moving n plates from the Peg Origin to the Peg Destination with help from the Peg Auxiliary. Here, 1<=k<=2^n-1.

## Problem 4

Consider a strictly convex array A where the numbers in A first strictly increase, then the numbers strictly decrease. Write a recursive and iterative pseudo-code to find the unique largest number in A. 
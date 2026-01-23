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
  for(ki=2; ki<=n; ki++) //key index
    key = A[ki]
    si = ki //sort index
    while si > 1 && key < A[si-1]
        A[si] = A[si-1]
        si -= 1
    A[si] = key

```



## Problem 3

Consider the Hanoi's Tower pseudocode presented in class, write a piece of pseudocode to print the k-th step when moving n plates from the Peg Origin to the Peg Destination with help from the Peg Auxiliary. Here, 1<=k<=2^n-1.

## Problem 4

Consider a strictly convex array A where the numbers in A first strictly increase, then the numbers strictly decrease. Write a recursive and iterative pseudo-code to find the unique largest number in A. 
# Homework 1

Tripp Lamb

2026 Jan 22

CS 617-01

## Problem 1

Page 41, Problem 2.4 Inversion numbers. you only need to present the pseudo-code part. Please refer to https://www.cs.mcgill.ca/~akroit/math/compsci/Cormen%20Introduction%20to%20Algorithms.pdf

## Problem 2

**Present a recursive pseudo-code for insertion sort**

(Iterative version for reference)

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

Recursive version:

```
// `A` is an index-1 based array of numbers
// `n` is len(A)
top-level call: insertion_sort_rec(A, n)

insertion_sort_rec(A, ei)

  if ei > 1
    key = A[ei]
    insertion_sort_rec(A, ei-1)
    i = ei
    
    while i > 1 && key < A[i-1]
      A[i] = A[i-1]
      i -= 1
      
    A[i] = key
```



## Problem 3

**Consider the Hanoi's Tower pseudocode presented in class, write a piece of pseudocode to print the k-th step when moving n plates from the Peg Origin to the Peg Destination with help from the Peg Auxiliary. Here, 1<=k<=2^n-1.**

This is not the most efficient way as it continues to solve the entire Hanoi problem, but it is the simplest change to the code in class to give the answer. 
```
// according to the psuedo code language this is pass-by-value the 
// algorithm returns the incremented step number, changing `curr`
// internally does not change the passed value

// `n` is the number of the current plate, larger number = bigger plate
// `from` is the peg number the current plate is moving from
// `to` is the peg number the current plate is moving to
// `hold` is the peg number not part of this move
// `curr` is the step number of the current move
// `k` is the constant number, at which step to output

top-level call: hanoi_solve(n, 1, 3, 2, 0, k)

hanoi_solve(n, from, to, hold, curr, k) -> next
  
  if(n > 1)
    curr = hanoi_solve(n-1, from, hold, to, curr, k)
    
  curr += 1
  if (curr == k)
    print from, '->', to
    
  
  if(n > 1)
    curr = hanoi_solve(n-1, hold, to, from, curr, k)
    
  return curr
```



## Problem 4

Consider a strictly convex array A where the numbers in A first strictly increase, then the numbers strictly decrease. Write a recursive and iterative pseudo-code to find the unique largest number in A. 

```
convex_find_max_iter(A)

  si = 1
  ei = n
	
  while (si != ei)
	  
	

```
























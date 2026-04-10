# Homework 3

## Problem 1: Longest Increasing Subsequence

### Variables

- $A[1..n]$: the input array of integers
- $c[i]$: the length of the longest increasing subsequence **ending at** index $i$
- $s[i]$: the index of the previous element in the LIS ending at index $i$ (0 if $i$ is the first element in its subsequence)

$c$ and $s$ are the same length a $A$

### Recurrence Relation

$$
c[i] = \begin{cases} 
  1 & \text{if no } k < i \text{ with } A[k] < A[i] \\ 
  1 + \max{c[k] \mid 1 \leq k < i,\ A[k] < A[i]} & \text{otherwise} 
\end{cases}
$$

The overall answer is $\max{c[i] \mid 1 \leq i \leq n}$.

### Pseudocode

```
LIS(A, c, s, n)

  maxlen = -∞
  maxidx = 0

  for (i = 1; i <= n; i++)

    c[i] = 1
    s[i] = 0

    for (k = 1; k < i; k++)

      if (A[k] < A[i])
        ck1 = c[k] + 1
        if (ck1 > c[i])
          c[i] = ck1
          s[i] = k

    if (c[i] > maxlen)
      maxlen = c[i]
      maxidx = i

  return maxlen, maxidx
```

**Reconstruction:**

```
Top-level call: reconstructLIS(A, s, maxidx)

reconstructLIS(A, s, i)

  if (s[i] != 0)
    reconstructLIS(A, s, s[i])

  print A[i]
  return
```

### Time and Space Complexity

- **Time:** $O(n^2)$ — two nested loops over the array.
- **Space:** $O(n)$ — arrays $c$ and $s$ each of length $n$.

## Problem 2: Longest Palindromic Subsequence

### Variables

- $A[1..n] $: the input array of characters
- $c[i,j] $: the length of the longest palindromic subsequence of elements $i $ through $j $
- $s[i,j] $: reconstruction direction — $0 $ if $A_i = A_j $ (matched), $1 $ if shrunk from the left, $-1 $ if shrunk from the right

### Recurrence Relation

$$
c[i,j] = \begin{cases} 0 & \text{if } i > j \\ 1 & \text{if } i = j \\ 2 + c[i+1,\, j-1] & \text{if } A_i = A_j \\ \max\{c[i+1,\, j],\; c[i,\, j-1]\} & \text{otherwise} \end{cases}
$$

### Pseudocode

```
c[:,:] = 0
s[:,:] = -1

LPS(A, c, s, n) -> maxlen

  for (j = 1; j <= n; j++)

    c[j,j] = 1

    for (i = j-1; i > 0; i--)

      if (A[i] == A[j])
        c[i,j] = 2 + c[i+1, j-1]
        s[i,j] = 0
      else
        if (c[i+1, j] > c[i, j-1])
          c[i,j] = c[i+1, j]
          s[i,j] = 1
        else
          c[i,j] = c[i, j-1]
          s[i,j] = -1

  return c[1, n]
```

**Reconstruction:**

```
reconstructLPS(A, s, i, j)

  if i > j
    return
  else if i == j
    print A[i]
    return
  else
    if s[i,j] == -1
      reconstructLPS(A, s, i, j-1)
    else if s[i,j] == 0
      print A[i]
      reconstructLPS(A, s, i+1, j-1)
      print A[j] // A[i] == A[j], but this is easier for me to read
    else
      reconstructLPS(A, s, i+1, j)
```

Top-level call: `reconstructLPS(A, s, 1, n)`

### Time and Space Complexity

- **Time:** $O(n^2) $ — two nested loops over the range of subproblems.
- **Space:** $O(n^2)$ — the $c$ and $s$ tables are each $n \times n $.

## Problem 3: Ball Meeting Conviviality Problem

### Variables

```
Employee = {
  id:string, //identifier of employee
  conviviality: real, //conviviality score of individual employee
  children: Employee[], //references to employees managed by this employee
  parent: Employee, //reference to supervisor
  ecs_w: real = 0.0, //maximum conviviality score of subtree including this employee [default: 0.0]
  ecs_wo: real = 0.0, //maximum conviviality score of subtree not including this employee [default: 0.0]
  invited: bool = false //true denotes that the employee is invited
}
```

- $S$: root employee of the complete tree
- $e$: root employee of the current subtree
- $\text{mcs}(e)$: the maximum conviviality score of employee $e$
- $\text{mcs}_w(e)$: the maximum conviviality score of sub-tree with root employee $e$ explicitly including $e$ (with)
- $\text{mcs}_{wo}(e)$: the maximum conviviality score of sub-tree with root employee $e$ explicitly not including $e$ (without)
- $c(e)$: the conviviality score of employee $e$ exclusively

### Recurrence Relation

$$ \text{mcs}*w(e) = \begin{cases} c(e) & \text{if } e \text{ has no children} \ c(e) + \displaystyle\sum*{c ,\in, e.\text{children}} \text{mcs}_{wo}(c) & \text{otherwise} \end{cases} $$

$$ \text{mcs}*{wo}(e) = \begin{cases} 0 & \text{if } e \text{ has no children} \ \displaystyle\sum*{c ,\in, e.\text{children}} \max{\text{mcs}_{wo}(c),; \text{mcs}_w(c)} & \text{otherwise} \end{cases} $$

$$ \text{mcs}(e) = \max{\text{mcs}*w(e),; \text{mcs}*{wo}(e)} $$

$$ \text{answer} = \text{mcs}(S) $$

### Pseudocode

```
top-level: BMC(S)

BMC(e) -> maximum_conviviality_score

  e.ecs_w = e.conviviality
  e.ecs_wo = 0

  for c in e.children
      
    BMC(c)

    e.ecs_w += c.ecs_wo
    e.ecs_wo += max{c.ecs_wo, c.ecs_w}

  return max{e.ecs_w, e.ecs_wo}
```

**Reconstruction:**

```
top-level call: reconstructBMC(S, S.ecs_w > S.ecs_wo)

reconstructBMC(e:Employee, with:bool)

  if (with)
    print e.id

  for c in e.children
    reconstructBMC(c, !with && c.ecs_w > c.ecs_wo)
```

### Time and Space Complexity

- **Time:** $O(n)$ — each node is visited exactly once.
- **Space:** $O(n)$ — two values stored per node, plus $O(n)$ recursion stack in the worst case.
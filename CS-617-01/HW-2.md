# Homework 2

(Tripp) Milton Lamb

2026 Feb 24

## Problem 1

3-3 Ordering by asymptotic growth rates 
a. Rank the following functions by order of growth.
<img src=".\HW2-P1.png" alt="HW2-P1" />

1. $2^{2^{n+1}}$
2. $2^{2^n}$
3. $(n + 1)!$ 
4. $n!$
5. $e^n$
6. $n\cdot2^n$
7. $2^n$
8. $(\frac{3}{2})^{n}$
9. $\lg{n}^{\lg{n}} = n^{\lg{\lg{n}}}$
10. $(\ln{n})!$
11. $2^{2\cdot\lg{n}}$, $4^{\lg{n}}$
12. $2^{\lg{n}}$
13. $(\sqrt{2})^{\lg{n}}$
14. $2^\sqrt{2\cdot\lg{n}}$
15. $2^{lg^{*}{n}}$
16. $n^3$
17. $n^2$
18. $n\cdot\lg{n}$
19. $\lg{n!}$
20. $n$
21. $\lg^{2}{n}$
22. $\ln{n}$
23. $\sqrt{\lg{n}}$
24. $\ln{\ln{n}}$
25. $\lg^{*}{n} = \lg^{*}{(\lg{n})}$
26. $\lg{(\lg^{*}{n})}$
27. $n^{\frac{1}{\lg{n}}} = 1$

## Problem 2



### (a) $T(n) = 2T(n/2) + n^4$

$$
a = 2, \quad b = 2, \quad c_{\text{crit}} = \log_2 2 = 1, f(n) = n^4
$$



**Regularity condition check:**

$$
af!\left(\frac{n}{b}\right) \leq c \cdot f(n) \\

2\left(\frac{n}{2}\right)^4 \leq c \cdot n^4 \\

2 \cdot \frac{n^4}{16} \leq c \cdot n^4 \\

\frac{1}{8} \cdot n^4 \leq c \cdot n^4 \\

\frac{1}{8} \leq c
$$

The regularity condition is satisfied with $c = \frac{1}{8}$.

Since $f(n) = n^4 = \Omega(n^{c_{\text{crit}} + \varepsilon})$ with $\varepsilon = 3$:

$$
\boxed{T(n) = \Theta(n^4)}
$$


------

### (c) $T(n) = 16T(n/4) + n^2$

$$
a = 16, \quad b = 4, \quad c_{\text{crit}} = \log_4 16 = 2 \\

f(n) = n^2 = \Theta(n^{c_{\text{crit}}} \lg^0 n) \\
$$

Since $f(n) = \Theta(n^{c_{\text{crit}}})$: 

$$
\boxed{T(n) = \Theta(n^2 \lg n)}
$$

------

### (f) $T(n) = 2T(n/4) + \sqrt{n}$

$$
a = 2, \quad b = 4, \quad c_{\text{crit}} = \log_4 2 = 0.5 \\

f(n) = \sqrt{n} = \Theta(n^{c_{\text{crit}}} \lg^0 n)
$$

Since $f(n) = \Theta(n^{c_{\text{crit}}})$:

$$
\boxed{T(n) = \Theta(\sqrt{n} \lg n)}
$$

## Problem 3

Solve $T(n) = T(\frac{n}{4})+ T(\frac{2n}{3}) + \Theta(n)$


$$
\begin{array}{ccccccccc c}
& & & &  \boxed{cn} & & & & & & cn \\[12pt]
& &  \boxed{c\tfrac{n}{4}} & & & & \boxed{c\tfrac{2n}{3}} & & & & \tfrac{11}{12}\,cn \\[12pt]
& \boxed{c\tfrac{n}{16}} & \quad \boxed{c\tfrac{n}{6}} & & & \boxed{c\tfrac{n}{6}} & & & \boxed{c\tfrac{4n}{9}} & & \left(\tfrac{11}{12}\right)^{\!2} cn \\[12pt]
& & & & \vdots & & & & & & \vdots \\[12pt]
\boxed{\Theta(1)} & \cdots & \boxed{\Theta(1)} & & & & & && & \leftarrow \text{depth } \log_4 n \\[12pt]
& & & & \vdots & & & & & & \vdots \\[12pt]
& & & & & & &\boxed{\Theta(1)} & \cdots & \boxed{\Theta(1)} & \leftarrow \text{depth } \log_{3/2} n \\
\end{array}
$$



The work per level is $\frac{11}{12}^k$ where k is the current level. This will yield the geometric series upon summing all levels. Since $\frac{11}{12} < 1$ the series will converge which also means the level sums will shrink and the root will dominate. This will yield $T(n) = \Theta(n)$. To prove this we can create an upper and lower bound on the growth function. For the upper bound we can extend $ n \rightarrow \infin$ and solve the geometric series creating a definitive upper bound of the total work.
$$
\begin{align}
\sum_{k=0}^{\text{n}} r^k &= \frac{(1 - r^{(n+1)})}{1-r} \\
\lim_{n\to \infin}\frac{(1 - r^{(n+1)})}{1-r} &=  \frac{1}{1-r} \text{when} \ r \lt 1 \\
\sum_{k=0}^{\infin} cn\left(\frac{11}{12}\right)^k &= cn\frac{1}{1-\left(\frac{11}{12}\right)} = 12cn = O(n)\\

\end{align}
$$
Now for the lower bound we can take just the root which is $cn$ or:
$$
T(n) \ge cn = \Omega(n)
$$
Combining
$$
\Omega \le T(n) \le O(n) \\ \\
\boxed{T(n) = \Theta(n)}
$$









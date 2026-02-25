# Homework 2

## Problem 1

3-3 Ordering by asymptotic growth rates 
a. Rank the following functions by order of growth.
![HW2-P1](C:\Users\tripp.lamb\Projects\School\CS-617-01\HW2-P1.png)

#NOTE: use stirlings algorithm to compare factorials and double exponents

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

(a) $T(n) = 2T(n/2) + n^4$

a = 2

b = 2

c_crit = lg_b_a

f(x) = n^4

af(n/b) <= c*f(n)

2(n/2)^4 <= c(n^4)
2(n^4/16) <= c(n^4)
1/8*(n^4) <= c (n^4)
1/8 <= c

the above equation is satisfied with c = 1/8

epsilon = +3 therefore T(n) = \Theta(n^4)

(c) $T(n) = 16T(n/4) + n^2$

(f) $T(n) = 2T(n/4) + \sqrt{n}$


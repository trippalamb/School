# CS 517 Midterm 2

(Tripp) Milton Lamb

2025 April, 13th

A#:25002371

Disclaimer: I often think more naturally in psuedocode/code than algorithm so I've included the psuedocode for many problems even though it isn't required. I hope that is alright.



## Problem 1





### Question 1

Design an algorithm to find all the common elements in two sorted lists of numbers. For example, for the lists [2, 5, 5, 5] and [2, 2, 3, 5, 5, 7] the output should be 2, 5, 5 .What is the maximum number of comparisons your algorithm makes if the lengths of the two given lists are m and n, respectively? [15 Points]

#### Algorithm

1. record the length of both list arguments ($list_a, \ list_b$) as $n_a$ and $n_b$ 

2. declare and set index integer variables $i_a$ and $i_b$ to 0

3. declare an empty $list_{result}$ 

4. loop until  $i_a > n_a$  and $i_b > n_b$, and perform the indented steps below repeated until this condition is met

   a. if the value of $list_a[i_a]$ equals $list_b[i_b]$ then add the integer $list_a[i_a]$ to $list_{result}$ and increment both $i_a$ and $i_b$ by 1 and repeat to step 4.a. else go to 4.b.

   b. else if the value of $list_a[i_a]$ is less than $list_b[i_b]$ then increment $i_a$ by 1 and go to step 4.a. else go to step 4.c

   c. else if the value of $list_a[i_a]$ is greater than $list_b[i_b]$ then increment $i_b$ by 1.  Go to step 4.a

#### Answers



#### Pseudocode

```
algorithm find_common_elements(list_a:list, list_b:list) return(result) #for zero indexed sorted lists

	int i_a = 0
	int i_b = 0
	int n_a = length(list_a)
	int n_b = length(list_b)
	
	list[int] result = []
	
	while (i_a < n_a and i_b < n_b)
	    
	    int a = list_a[i_a]
	    int b = list_b[i_b]
	    
		if a == b
			result.push(a)
			i_a += 1
			i_b += 1
		else if a < b
			i_a += 1
		else
		    i_b += 1
		end if
		    
    end while
    
end algorithm
```



#### Mathjax

$$
\begin{align*}
&\textbf{algorithm } \text{find\_common\_elements}(\text{list}_a:\text{list}, \text{list}_b:\text{list}) \text{ }\# \ \text{for zero indexed sorted lists}\\
&\\
&\quad \text{int } i_a = 0\\
&\quad \text{int } i_b = 0\\
&\quad \text{int } n_a = \text{length}(\text{list}_a)\\
&\quad \text{int } n_b = \text{length}(\text{list}_b)\\
&\\
&\quad \text{list[int] } \text{list}_{result} = []\
&\\
&\quad \textbf{while } (i_a < n_a \text{ and } i_b < n_b)\\
&\\
&\quad\quad \text{int } a = \text{list}_a[i_a]\\
&\quad\quad \text{int } b = \text{list}_b[i_b]\\
&\\
&\quad\quad \textbf{if } a = b\\
&\quad\quad\quad \text{list}_{result}.push(a)\\
&\quad\quad\quad i_a += 1\\
&\quad\quad\quad i_b += 1\\
&\quad\quad \textbf{else if } a < b\\
&\quad\quad\quad i_a += 1\\
&\quad\quad \textbf{else}\\
&\quad\quad\quad i_b += 1\\
&\quad\quad \textbf{end if}\\
&\\
&\quad \textbf{end while}\\
&\\
&\textbf{end algorithm}
\end{align*}
$$



### Question 2


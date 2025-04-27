Problem 1.1 Algorithm

1. assign the first sorted list to be list n and the second sorted list to be list m

2. initialize an empty list named result

3. determine the length of your two lists of numbers assign the first to n_m, and the second to n_n

4. create two indices i_n and i_m initialized to 0

5. start a loop the continues until i_n is greater than or equal to n_n and i_m is greater than or equal to n_m. go to step 5.a

   5.a assign the value at list_n[i_n] to v_n and the value at list_m[i_m] to be v_m. proceed to 5.b

   5.b if v_n is equal to v_m then add v_n to result and increment both i_n and i_m by one then proceed to 5.a otherwise proceed to 5.c

   5.c else if v_n is greater than v_m increment i_m by one and proceed to 5.a. otherwise proceed to 5.d

   5.d else if v_n is less than v_m increment i_n by one. proceed to 5.a

   

problem 1.2 algorithm

1. set result to floor(n/2) + 1, any integer n > 0

2. set delta to floor(n/2)

3. start a loop that continues indefinitely. proceed to 3a

   3.a set result*result to r2. proceed to 3b

   3.b. check to see if r2 is less than or equal to n and (result + 1)^2 is greater than n. if so terminate and return result. else proceed to 3.c

   3.c. if r2 is less than n increment result by delta. set delta equal to the max of the floor of half delta and 1 then proceed to 3.a. otherwise proceed to 3d.

   3.d. if r2 is greater than n decrement result by delta and set delta equal to the max of the floor of half delta and 1 then proceed to 3.a.
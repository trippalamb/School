/* Lecture Example : Program-01-1.cpp
 * (Tripp) Milton Lamb
 * 2024 Aug 28th
 * CS 521-01
 * Made using Windows in VS Code IDE
 * This program solves for the nth fibonacci number using multiple methods
 * and returns the answer as well as elapsed calculation time for the method.
*/

#include <iostream>
#include <iomanip>
#include <cmath>
#include <chrono>
#include <algorithm>

using namespace std;

const unsigned int MAX_N= 93;           // maximum fibonacci number allowed by long long ints
long long int _memory[MAX_N] = {0,0,1}; // holds recursive calculations for `calc_fibonacci_recursive_wStorage` to speed up the method.
unsigned int _length = 2;               // holds the maximum currently calculated fibonacci number for `calc_fibonacci_recursive_wStorage`

/**
 * Calculates the nth fibonacci number using an iterative method.
 * 
 * @param n the fibonacci sequence index requested
 */
long long int calc_fibonacci_loop(unsigned int n){

    //short circuits
    if     (n == 0) {return 0;}
    else if(n == 1) {return 0;}
    else if(n == 2) {return 1;}

    long long int fib = 0;    // output value
    long long int last_1 = 1; // f(n-1)
    long long int last_2 = 0; // f(n-2)

    for (int i = 3; i <= n; i++){
        fib = last_1 + last_2;
        last_2 = last_1;
        last_1 = fib;
    }

    return fib;
}

/**
 * Calculates the nth fibonacci number using a naive recursive algorithm.
 * 
 * @param n the fibonacci sequence index requested
 */
long long int calc_fibonacci_recursive(unsigned int n){

    if     (n == 1) {return 0;}
    else if(n == 2) {return 1;}
    else if(n  > 2) {
        return calc_fibonacci_recursive(n-1) + calc_fibonacci_recursive(n-2);
    }
    else{ //(n == 0)
        cout << "Warning: Can't actually calculate fibonacci of n = 0. Returning 0." << endl;
        return 0;
    }
}

/**
 * Calculates the nth fibonacci number using a recursive algorithm with memoization.
 * 
 * @param n the fibonacci sequence index requested
 */
long long int calc_fibonacci_recursive_wStorage(unsigned int n){

    if(n <= _length){ 
        return _memory[n];
    }
    else{
        _memory[n] = calc_fibonacci_recursive_wStorage(n-1) + calc_fibonacci_recursive_wStorage(n-2);
        _length = max(_length, n);
        return _memory[n];
    }

}

/**
 * Calculates the nth fibonacci number using an approximation of binet's formula.
 * 
 * @param n the fibonacci sequence index requested
 */
long long int calc_fibonacci_binet(unsigned int n){

    const double sqrt_5 = sqrt(5.0);
    const double golden = 1.618033988749895;

    double golden_pow_n = pow(golden, n-1);
    return round(golden_pow_n/sqrt_5);

}

/**
 * Gets the proper ordinal suffix for a given integer for proper printing.
 * 
 * @param number the number to find the suffix for.
 */
string get_ordinal_suffix(unsigned int number) {
    if (number % 100 >= 11 && number % 100 <= 13) {
        return "th";
    }
    switch (number % 10) {
        case  1: return "st";
        case  2: return "nd";
        case  3: return "rd";
        default: return "th";
    }
}

/**
 * Wraps a fibonacci caculation function to ensure time calcuations and
 * console printing are consistent.
 * 
 * @param calc_fib a function that takes a unsigned int as an argument and returns a long long int,
 *                 in this context the argument should be a fibonacci calculation function.
 * @param n        the fibonacci sequence index requested
 */
void test_fib(long long int (*calc_fib)(unsigned int), unsigned int n){

    chrono::time_point<chrono::steady_clock> before; // holds the clock time before fibonacci call
    chrono::time_point<chrono::steady_clock> after;  // holds the clock time after fibonacci call
    long long int fib = 1;                           // holds the return value of the fibonacci calculation
    int elapsed_time = 0;                            // elapsed time of function call to fibonacci number
    string suffix = "";                              // holds the ordinal suffix for proper printing to console

    before = std::chrono::steady_clock::now();
    fib = calc_fib(n);
    after = std::chrono::steady_clock::now();

    elapsed_time = chrono::duration_cast<chrono::microseconds> (after - before).count();
    suffix = get_ordinal_suffix(n);

    cout << "The " << n << suffix << " Fib number is: " << fib << endl;
    cout << "The elapsed time for the loop version in microseconds is: " << elapsed_time << endl;
    cout << endl;

}

int main() {

    unsigned int n; // the desired number of the fibonacci sequence to calculate

    cout << "Which value of Fibonacci Sequence to compute (N)? ";
    cin >> n;
    cout << endl;

    if(n > MAX_N){
        cout << "Error: This is too large of an N. N must be smaller than or equal to " << MAX_N << "." << endl;
    }

    test_fib(calc_fibonacci_loop, n);
    test_fib(calc_fibonacci_recursive, n);

    // commented out to make output match requirements document
    // test_fib(calc_fibonacci_recursive_wStorage, n);
    // test_fib(calc_fibonacci_binet, n);

    cout << "End Program - Goodbye."<< endl << endl;

    return 0;
}

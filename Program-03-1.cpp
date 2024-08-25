/* Lecture Example : Program-01-1.cpp
 * (Tripp) Milton Lamb
 * 2024 Aug 25th
 * CS 521-01
 * Made using Windows in VS Code IDE
 * This program takes in a list of test grades from
 * a user one at a time until a negative number is 
 * typed and then it displays some statistics about
 * the entered data.
*/
#include <iostream>
#include <iomanip>
using namespace std;

void print_stars(char letter, int n){
    cout << letter << ": "; 
    for(int i = 0; i < n; i++){
        cout << '*';
    }
    cout << endl;
}

int main() {

    const int MAXTESTS = 100;  // maximum number of tests we can store
    int testList[MAXTESTS];  // the list storing up to 100 test grades entered

    int count = 0;           // a counter keeping track of how many tests we have read in
    int nextTest = 0;        // next test read in, to prime the pump
    int n_A = 0;
    int n_B = 0;
    int n_C = 0;
    int n_D = 0;
    int n_F = 0;

    double testSum = 0.0;        // sum of all the tests entered
    double average = 0.0;     // average of all the test grades

    while (count < MAXTESTS) { // Answer: because you'd encounter a index overflow

        cout << "Please enter a test grade (-1 to quit): ";
        cin >> nextTest;  // get the next value

        if      (nextTest < 0 ){ break;}
        else if (nextTest < 60){ n_F++;}
        else if (nextTest < 70){ n_D++;}
        else if (nextTest < 80){ n_C++;}
        else if (nextTest < 90){ n_B++;}
        else                   { n_A++;}

        testList[count] = nextTest;  // save the last test in the array

        count++;                     // count the good test

    }

    // when we get here, testCount reflects how many tests were read in
    // now lets write a for loop to sum the tests and compute the average
    for (int i = 0; i < count; i++) { // Answer: because we are unlikely to have filled up the entire array
        testSum += testList[i];      // visit each test stored and add it to the sum
    }
    
    average = testSum / count;
    cout << "\nThe average of your " << count << " tests is " << setprecision(2) << fixed;
    cout << average << endl;
    print_stars('A', n_A);
    print_stars('B', n_B);
    print_stars('C', n_C);
    print_stars('D', n_D);
    print_stars('F', n_F);

    //*** Answer: It prints a NaN (in c++). ***//
    cout << "\nEnd Program - Goodbye." << endl;
    
}


/* File: example-03-3.cpp
 * This program modifies version 03-2 to store the tests in
 * an array.  We can later perform other operations on the
 * data in this array, such as computing the sum, finding
 * the maximum, and sorting in order to find the median.
*/
#include <iostream>
#include <iomanip>
using namespace std;

int main() {

    int count;          // a counter keeping track of how many tests we have read in
    const int MAXTESTS = 100;  // maximum number of tests we can store
    int testList[MAXTESTS];    // the list storing up to 100 test grades entered
    int nextTest;       // next test read in, to prime the pump

    int testSum;        // sum of all the tests entered
    double average;     // average of all the test grades

    // initialize our loop control counter to 0. This reflects the fact that we have read in 0 tests so far
    // initialize the testSum to 0 so we can add individual tests to it
    // This version of the loop "primes the pump" by requesting the first test grade
    // then start the loop to read in tests until the user enters a negative number

    count = 0;

    cout << "Please enter a test grade (-1 to quit): ";
    cin >> nextTest;  // get the next value
    while (nextTest >= 0 && count < MAXTESTS) // why is this extra check needed?
    {
        testList[count] = nextTest;  // save the last test in the array
        count++;                     // count the good test

        // continue the pump
        if (count < MAXTESTS) // WHY??????
        {
            cout << "Please enter a test grade (-1 to quit): ";
            cin >> nextTest;  // get the next value
        }
    }

    // when we get here, testCount reflects how many tests were read in
    // now lets write a for loop to sum the tests and compute the average
    testSum = 0;
    for (int i = 0; i < count; i++)  // why not 100?
        testSum += testList[i];      // visit each test stored and add it to the sum

        average = double(testSum) / count;
        cout << "\nThe average of your " << count << " tests is " << setprecision(2) << fixed;
        cout << average << endl;

        //*** WHAT HAPPENS IF THE USER ENTERS A NEGATIVE NUMBER TO START? ***//
        cout << "\nEnd Program - Goodbye." << endl;
}
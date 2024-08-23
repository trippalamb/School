// File: lab-01-2-start.cpp
// This is the starting program for lab-01-2. You are to rewrite the output
// using proper formatting to produce the report in the desired form.
//--------------------------------------------------------------------------------
#include <iostream>
#include <iomanip>
using namespace std;
int main() {
    string name1 = "Bob";
    string name2 = "Elizabeth";
    double number1 = 78.9457;
    double number2 = 0.000012;
    cout << "Lab-01-2: Formatting Examples\n\n";
    cout << "Name 1 is right-justified in 12 spaces: " << right << setw(12) << name1 << "!" << endl;
    cout << "Name 2 is left-justified in 12 spaces: " << left << setw(12) << name2 << "!" << endl;
    cout << "Name 1 is in quotes: \"" << name1 << "\"" << endl;
    cout << "Number 1 is using 2 digits after the decimal: " << fixed << setprecision(2) << number1 << endl;
    cout << "Number 1 is using 10 spaces, 2 after the decimal: " << right << setw(10) << number1 << endl;
    cout << "Number 2 is printed in normal notation: " << setprecision(6) << number2 << endl;
    cout << "Number 2 is printed in sci notation: " << scientific << number2 << endl;
    return 0;
} // end of main program

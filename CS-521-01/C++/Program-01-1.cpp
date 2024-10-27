/* Lecture Example : Program-01-1.cpp
 * (Tripp) Milton Lamb
 * 2024 Aug 22nd
 * CS 521-01
 * Made using Windows in VS Code IDE
 * This program prompts the user to enter a radius (assumed to be in cm)
 * computes the volume of the corresponding sphere, then prints the
 * resulting volume to the console, in both cubic-cm and cubic-inches,
 * and with a precision of 3 decimals
*/

#include <iostream>
#include <iomanip>
#include <cmath>
using namespace std;

int main() {

    const double PI = (4.0 * atan(1.0));              // formula for the value of pi
    const double PI_4_3 = (4.0 / 3.0) * PI;           // a precomputed value for 4/3*pi
    const double CM_PER_INCH = 2.54;                  // the scalar for conversion from inches to centimeters
    const double INCH_PER_CM = 1.0 / CM_PER_INCH;     // the scalar for conversion from centimeters to inches
    const double INCH3_PER_CM3 = pow(INCH_PER_CM, 3); // the scalar for conversion from cm3 to inches3

    double radius;         // the radius of a circle, to be entered by the user
    double volume_cm3;     // the volume of a circle in cubic centimeters, to be computed
    double volume_inches3; // the volume of a circlein cubic inches, to be computed

    //get input from user
    cout << "Welcome to the volume calculator." << endl << endl;
    cout << "Enter the radius of your sphere (in cm): ";
    cin  >> radius;
    cout << endl;

    volume_cm3 = PI_4_3 * pow(radius, 3);        // calculate volume in cm^3
    volume_inches3 = volume_cm3 * INCH3_PER_CM3; // convert volume to inches^3

    cout << fixed << setprecision(3); //set precision to be 3 decimal places in fixed-point notation

    //display results to user
    cout << "For a sphere with a radius of " << radius << " cm" << endl;
    cout << "The volume is: " << volume_cm3 << " cm-cubed, " << endl;
    cout << "or " << volume_inches3 << " inches-cubed" << endl << endl;
    cout << "End program" << endl;
    
    return 0;
}

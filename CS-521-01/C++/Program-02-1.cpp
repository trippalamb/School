/* Lecture Example : Program-01-1.cpp
 * (Tripp) Milton Lamb
 * 2024 Aug 23rd
 * CS 521-01
 * Made using Windows in VS Code IDE
 * This program prompts the user to enter the coefficients
 * for a second order polynomial function and then produces
 * the roots of the polynomial. It prevents some potential 
 * pitfalls involving zero valued coefficients and can handle
 * negative roots by producing complex roots. 
*/
#include <iostream>
#include <iomanip>
#include <cmath>
using namespace std;

bool is_effectively_zero(double n){
    return ( -0.000001 < n && n < 0.000001);
}

int main() {

    // variables to be set by the user
    double a = 0.0;   // the coefficent for x^2
    double b = 0.0;   // the coefficent for x^1
    double c = 0.0;   // the coefficent for x^0

    //variables to be calculated and returned
    double root_1 = 0.0;  // the linear root or the first root of the second order polynomial
    double root_2 = 0.0;  // the possible second root of the second order polynomial

    //variables for terms of the quadratic equation
    double b2_4ac         = 0.0;
    double sqrt_b2_4ac    = 0.0;
    double two_a          = 0.0;
    double neg_b_2a       = 0.0;
    double sqrt_b2_4ac_2a = 0.0;

    //bool is_imaginary = false; // `true` implies that the result will be an imaginary number

    cout << "Welcome to the roots calculator." << endl;
    cout << "Enter values for a b and c, separated by spaces: ";
    cin >> a >> b >> c;

    cout << fixed << setprecision(3);

    if (is_effectively_zero(a)){ // is a linear equation
        root_1 = -c / b;
        cout << "The single root of the equation is: " << root_1 << endl;
    }
    else{ // is a 2nd order polynomial

        b2_4ac = b*b - 4*a*c;
        two_a = 2 * a;
        sqrt_b2_4ac = sqrt(abs(b2_4ac));

        if (b2_4ac >= 0.0) { // is a real number

            root_1 = (-b + sqrt_b2_4ac) / two_a;
        
            if (is_effectively_zero(sqrt_b2_4ac)){ // there is a double root
                cout << "The single root of the equation is: " << root_1 << endl;
            }
            else{ // there are two unique roots
                root_2 = (-b - sqrt_b2_4ac) / two_a;
                cout << "Your roots are: " << root_1 << " AND " << root_2 << endl;
            }
        }
        else{ // is an imaginary number
            neg_b_2a = -b / two_a;
            sqrt_b2_4ac_2a = sqrt_b2_4ac / two_a;
            cout << "Your roots are: " << neg_b_2a << " + " << sqrt_b2_4ac_2a << "i AND " 
                                       << neg_b_2a << " - " << sqrt_b2_4ac_2a << "i";
        }

    }


    return 0;
}


/* Lecture Example : Program-01-1.cpp
 * (Tripp) Milton Lamb
 * 2024 Aug 23rd
 * CS 521-01
 * Made using Windows in VS Code IDE
 * This program prompts the user to enter a series of product and
 * purchase information. And then creates an invoice to print 
 * for the user.
*/
#include <iostream>
#include <iomanip>
#include <string>
using namespace std;

int main() {

    //user set variables
    string name;           // product's name
    double cost_unit;      // [dollars] product's list price for a single unit
    int    quantity;       // quantity of product to purchase
    double rate_tax;       // [percent] tax rate to apply

    //calculated variables
    double cost_combined;   // [dollars] total pre-tax cost
    double cost_tax;        // [dollars] total tax cost
    double cost_total;      // [dollars] total cost to consumer

    cout << "Enter a product description: ";
    getline(cin, name);

    cout << "Enter the product list price: $";
    cin >> cost_unit;

    cout << "How many units to purchase? ";
    cin >> quantity;

    cout << "What is the sales tax rate? ";
    cin >> rate_tax;

    cost_combined = cost_unit * quantity;
    cost_tax = cost_combined * (rate_tax / 100.0); //converting from percent
    cost_total = cost_combined + cost_tax;

    cout << endl;
    cout << "----------------------- INVOICE ------------------------" << endl;
    cout << "Product: " << name << endl;
    cout << fixed << setprecision(2);
    cout << "Price $" << setw(7) << cost_unit
         << setw(38) << "Units purchased: " << setw(3) << quantity << endl;

    cout << "Purchase Amount $"  << setw(9) << cost_combined << endl;
    cout << "Sales tax       $"  << setw(9) << cost_tax      << endl;
    cout << "Total Invoice   $"  << setw(9) << cost_total    << endl;
    cout << "------------------------ END ---------------------------" << endl;

    return 0;
}

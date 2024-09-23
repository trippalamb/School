/* Lecture Example : Program-10-1.cpp
 * (Tripp) Milton Lamb
 * 2024 Sep 14th
 * CS 521-01
 * Made using Windows in VS Code IDE
 * This program reads in a list of baseball player primary stats into a binary tree, 
 * then calculates several secondary stats, and prints a summary of the information
 * forwards and backwards.
*/

#include <iostream>
#include <fstream>
#include <string>
#include <iomanip>

#include "PlayerTree.h"

using namespace std;

int main() {

    PlayerTree* players; // holds the information of all players in the input file

    string fn_in = "";  // the input dna file name, user input
    string fn_out = ""; // the output report file name, user input

    string line_in = "";    // the current input line
    string line_clean = ""; // the current cleaned up input line (lower to capital)

    ifstream file_in;  // the input file from `fn_in`
    ofstream file_out; // the output file from `fn_out`

    cout << "STILL MUST UNHARDCODE PATHS AND TEST"<<endl;
    cout << "Welcome to the player statistics calculator test program." << endl << endl;

    cout << "Enter the name of the input data file: " ;
    //cin >> fn_in;  // get the input file name
    fn_in = ".\\test_data\\playerinput.txt";
    file_in.open(fn_in);   // Open the dna file
    if (!file_in.is_open()) {
        cerr << "Failed to open the input file." << endl;
        return 1;
    }

    cout << endl << "Enter the name of the output data file: ";
    //cin >> fn_out; // get the output file name
    fn_out = ".\\test_data\\playerreport.txt";
    cout << endl;

    file_out.open(fn_out); // Open the report file
    if (!file_out.is_open()) {
        cerr << "Failed to open the report file." << endl;
        return 1;
    }

    cout << endl << "Reading the data from: " << fn_in << endl;

    players = new PlayerTree(file_in);
    players->build_report(file_out);
    players->remove_all();

    cout << "The output is in: " << fn_out << endl << endl;
    cout << "End of Program" << endl;
    
    return 0;
}
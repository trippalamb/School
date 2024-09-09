/* Lecture Example : Program-09-1.cpp
 * (Tripp) Milton Lamb
 * 2024 Sep 8th
 * CS 521-01
 * Made using Windows in VS Code IDE
 * This program reads in a list of baseball player primary stats, then calculates
 * several secondary stats, and prints a summary of the information.
*/

#include <iostream>
#include <fstream>
#include <string>
#include <iomanip>

#include "PlayerList.h"

using namespace std;


int main() {

    PlayerList players; // holds the information of all players in the input file

    string fn_in = "";  // the input dna file name, user input
    string fn_out = ""; // the output report file name, user input

    string line_in = "";    // the current input line
    string line_clean = ""; // the current cleaned up input line (lower to capital)

    ifstream file_in;  // the input file from `fn_in`
    ofstream file_out; // the output file from `fn_out`

    cout << "Welcome to the player statistics calculator test program." << endl << endl;

    cout << "Enter the name of the input data file: " ;
    cin >> fn_in;  // get the input file name

    file_in.open(fn_in);   // Open the dna file
    if (!file_in.is_open()) {
        cerr << "Failed to open the input file." << endl;
        return 1;
    }

    cout << endl << "Enter the name of the output data file: ";
    cin >> fn_out; // get the output file name
    cout << endl;

    file_out.open(fn_out); // Open the report file
    if (!file_out.is_open()) {
        cerr << "Failed to open the report file." << endl;
        return 1;
    }

    cout << "Reading the data from: " << fn_in << endl;

    file_out << "    PLAYER NAME      :    AVERAGE    OPS" << endl;
    file_out << "---------------------------------------------" << endl;

    players = PlayerList(file_in);
    file_out << players.to_string();

    file_out << endl << endl;
    file_out << "BASEBALL TEAM REPORT --- " << players.get_size() << " PLAYERS FOUND IN FILE" << endl;
    file_out << "OVERALL BATTING AVERAGE is " << fixed << setprecision(3) << players.calc_batting_average() << endl;

    cout << "The output is in: " << fn_out << endl << endl;
    cout << "End of Program" << endl;
    
    return 0;
}
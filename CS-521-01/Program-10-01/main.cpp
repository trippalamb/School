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


string open_input_file(ifstream& file_in){

    string fn_in = "";  // the input dna file name, user input

    cout << "Enter the name of the input data file: " ;
    cin >> fn_in;  // get the input file name

    file_in.open(fn_in);   // Open the dna file
    if (!file_in.is_open()) {
        cerr << "Failed to open the input file." << endl;
    }

    return fn_in;

}

string open_output_file(ofstream& file_out){

    string fn_out = ""; // the output report file name, user input

    cout << endl << "Enter the name of the output data file: ";
    cin >> fn_out; // get the output file name
    cout << endl;

    file_out.open(fn_out); // Open the report file
    if (!file_out.is_open()) {
        cerr << "Failed to open the report file." << endl;
        exit(2);
    }

    return fn_out;
}

void respond_to_cmds(PlayerTree*& players){

    string cmd = "";    //holds the most recently input command
    string name_first = "";
    string name_last = "";
    bool status = false;

    while (true){
        cout << "Would you like to 'quit', 'remove' a player, or 'print' the tree? ";
        cin >> cmd;
        cout << endl;

        if(cmd == "quit"){break;}
        else if(cmd == "remove"){

            cout << "Please enter player's first name (case insensitve): ";
            cin >> name_first;
            cout << "Please enter player's last name (case insensitve): ";
            cin >> name_last;

            status = players->remove_by_name(name_first, name_last, true);
            if(status){cout << "Player was removed." << endl << endl;}
            else{cout << "No player by that name was found." << endl << endl;}

        }
        else if(cmd == "print"){
            cout << players->to_string_tree() << endl;
        }
        else{
            cout << "<" << cmd << "> is not a valid command please use 'quit', 'remove', or 'print'." << endl;
        }
    }
}

int main() {

    PlayerTree* players; // holds the information of all players in the input file


    string fn_in = "";  // the input dna file name, user input
    string fn_out = ""; // the output report file name, user input

    ifstream file_in;  // the input file from `fn_in`
    ofstream file_out; // the output file from `fn_out`

    cout << "Welcome to the player statistics calculator test program." << endl << endl;

    fn_in = open_input_file(file_in);
    fn_out = open_output_file(file_out);

    cout << endl << "Reading the data from: " << fn_in << endl;

    players = new PlayerTree(file_in);
    players->build_report(file_out);

    cout << "The output is in: " << fn_out << endl << endl;

    respond_to_cmds(players);

    players->clear();

    cout << "End of Program" << endl;
    
    return 0;
}
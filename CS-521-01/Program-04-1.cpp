/* Lecture Example : Program-04-1.cpp
 * (Tripp) Milton Lamb
 * 2024 Aug 23rd
 * CS 521-01
 * Made using Windows in VS Code IDE
 * This program prompts the user for an input and output
 * file name and then parses the input file line by line
 * each line must be a sequence of dna (ie made of 'A', 
 * 'C', 'G', 'T'). It will then calculate the GC ratio of
 * each line, and print out a summary of the input file.
*/
#include <iostream>
#include <iomanip>
#include <fstream>
#include <string>

using namespace std;


int main() {

    const int MAX_FRAG_LENGTH = 50;       // maximum fragment length as per the requirements
    const int MIN_VALID_FRAG_LENGTH = 30; // the minimum fragment length to process GCRatio from requirements

    string fn_in = "";  // the input dna file name, user input
    string fn_out = ""; // the output report file name, user input

    string line_in = "";    // the current input line
    string line_clean = ""; // the current cleaned up input line (lower to capital)

    ifstream file_in;  // the input file from `fn_in`
    ofstream file_out; // the output file from `fn_out`

    int n_A = 0;        // the number of A's found in the current fragment
    int n_C = 0;        // the number of C's found in the current fragment
    int n_G = 0;        // the number of G's found in the current fragment
    int n_T = 0;        // the number of T's found in the current fragment
    int n_lines = 0;    // the total number of lines, used solely for output
    int n_invalid = 0;  // the total number of invalid length lines `line.length() < MIN_VALID_FRAG_LENGTH`

    double ratio_gc = 0.0; // the calculated gc ratio as defined in the requirements document

    cout << "Welcome to the DNA profiler." << endl;
    cout << "This program will read a set of DNA fragments from an input" << endl;
    cout << "data file. It will produce a report on the GC-ratios found in" << endl;
    cout << "the file." << endl << endl;
    
    cout << "Please enter the name of the input data file: " ;
    cin >> fn_in;  // get the input file name

    file_in.open(fn_in);   // Open the dna file
    if (!file_in.is_open()) {
        cerr << "Failed to open the input file." << endl;
        return 1;
    }

    cout << endl << "Please enter the name of the output data file: ";
    cin >> fn_out; // get the output file name
    cout << endl;

    file_out.open(fn_out); // Open the report file
    if (!file_out.is_open()) {
        cerr << "Failed to open the report file." << endl;
        return 1;
    }

    file_out << fixed << setprecision(2) << left;
    file_out << "REPORT ON INPUT FILE: " << fn_in << endl << endl;
    file_out << setw(MAX_FRAG_LENGTH) << "               FRAGMENT" << "  GCRatio    Other messages" << endl;
    file_out << "----------------------------------------------------------------------------------------------" << endl;
    
    // Read the dna file line by line
    while (getline(file_in, line_in)) {

        n_lines++;

        n_A = 0;
        n_C = 0;
        n_G = 0;
        n_T = 0;

        line_clean = "";
        for (char c : line_in){
            line_clean += toupper(c); //we can assume there are no bad characters from requirements
        }

        file_out << setw(MAX_FRAG_LENGTH) << line_clean << ":   ";

        if(line_clean.length() >= MIN_VALID_FRAG_LENGTH){

            for (char c : line_clean) {
                switch(c){
                    case 'A':
                        n_A++;
                        break;
                    case 'C':
                        n_C++;
                        break;
                    case 'G':
                        n_G++;
                        break;
                    case 'T':
                        n_T++;
                        break;
                }

            }

            ratio_gc = double(n_G + n_C) / double(n_A + n_T + n_G + n_C);
            file_out << ratio_gc;

            if(0.35 <= ratio_gc && ratio_gc <= 0.65){
                file_out << "  Fragment within the range 35\% - 65\%";
            }

            file_out << endl;

        }
        else{
            n_invalid++;
            file_out << "Fragment is too short to process" << endl;
        }

    }

    file_in.close();

    file_out << "---------------------------------------- SUMMARY -----------------------------------------------" << endl;
    file_out << "There were " << n_lines << " fragments found." << endl;
    file_out << n_invalid << " fragments(s) were not long enough to process." << endl;

    file_out.close();
    cout << "Report Complete - stored in file: " << fn_out << endl;
    cout << "Exiting Program" << endl;

    return 0;
}
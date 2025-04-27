#include <iostream>
#include <fstream>
#include <sstream>
#include <string>

//in place merge A and B into A. This function assumes that all values are pre-validated
void merge_arrays(int* A, int len_A, int* B, int len_B) {
    int i_A = len_A-1;
    int i_B = len_B-1;
    int i = len_A + len_B - 1;

    while (i >= 0) {
        if      (i_A < 0) { A[i] = B[i_B--]; }
        else if (i_B < 0) { A[i] = A[i_A--]; }
        else{
            int v_A = A[i_A];
            int v_B = B[i_B];

            if (v_A >= v_B) { A[i] = v_A; i_A--;}
            else            { A[i] = v_B; i_B--;}
        }
        i--;
    }
}

// clean up function
[[noreturn]] void exit_with_error(const std::string& message, int* A = nullptr, int* B = nullptr, int exit_code = 1) {

    if (A != nullptr) delete[] A;
    if (B != nullptr) delete[] B;
    
    std::cerr << "Error: " << message << std::endl;
    std::exit(exit_code);
}

int main(int argc, char* argv[]) {

    //validate command line argument was provided
    if (argc < 2) {
        std::string message = "Usage: " + std::string(argv[0]) + " <input_file>";
        exit_with_error(message);
    }

    std::ifstream inputFile(argv[1]);
    
    if (!inputFile.is_open()) {
        std::string message = "Unable to open input file: " + std::string(argv[1]);
        exit_with_error(message);
    }
    
    int len_total, len_A, len_B;
    inputFile >> len_total >> len_B;
    inputFile.ignore();
    
    int* A = new int[len_total];
    int* B = new int[len_B];
    
    int i_A = 0;
    std::string line;
    std::getline(inputFile, line);
    std::istringstream iss_A(line);
    
    //validate all input as we read it in
    while (iss_A >> A[i_A]) {
        i_A++;
        if (i_A > len_total) {
            std::string message = "Too many values on line 2. Expected length of " + std::to_string(len_total) + ".";
            exit_with_error(message, A, B);
        }
    }
    len_A = i_A;
    
    int i_B = 0;
    std::getline(inputFile, line);
    std::istringstream iss_B(line);
    
    while (iss_B >> B[i_B]) {
        i_B++;
        if (i_B > len_B) {
            std::string message = "Too many values on line 3. Expected length of " + std::to_string(len_B) + ".";
            exit_with_error(message, A, B);
        }
    }
    
    if (i_B < len_B) {
        std::string message = "Not enough values on line 3. Expected " + std::to_string(len_B) + 
                             ", got " + std::to_string(i_B);
        exit_with_error(message, A, B);
    }
    
    if (i_A + i_B != len_total) {
        std::string message = "First array length must be equal to the sum of values on lines 2 and 3";
        exit_with_error(message, A, B);
    }

    merge_arrays(A, len_A, B, len_B);
    
    for (int i = 0; i < len_total; i++) {
        std::cout << A[i] << " ";
    }
    std::cout << std::endl;
    
    delete[] A;
    delete[] B;
    
    inputFile.close();
    
    return 0;
}
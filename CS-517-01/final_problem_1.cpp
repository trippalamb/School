#include <iostream>
#include <fstream>
#include <list>
#include <string>
#include <sstream>

int find_first_magic_index(std::list<int> list) {
    int i = 0;
    int n = list.size();
    for (int value : list) {
        if (value == i){
            return i;
        }
        else if (value >= n){
            // early exit if value is greater than the length of array, since a sorted array cannot go back down
            return -1; 
        }
        i++;
    }
    return -1;
}

int main(int argc, char* argv[]) {
    // Check if the filename is provided as a command-line argument
    if (argc < 2) {
        std::cerr << "Usage: " << argv[0] << " <input_file>" << std::endl;
        return 1;
    }

    // Open the file provided as command-line argument
    std::ifstream inputFile(argv[1]);
    
    if (!inputFile.is_open()) {
        std::cerr << "Error opening file: " << argv[1] << std::endl;
        return 1;
    }
    
    std::string line;
    
    while (std::getline(inputFile, line)) {
        std::list<int> numbers;
        std::istringstream iss(line);
        int num;
        
        while (iss >> num) {
            numbers.push_back(num);
        }
        
        int result = find_first_magic_index(numbers);
        
        if (result == -1) {
            std::cout << "None" << std::endl;
        }
        else{
            std::cout << result << std::endl;
        }
    }
    
    // Close the file
    inputFile.close();
    
    return 0;
}
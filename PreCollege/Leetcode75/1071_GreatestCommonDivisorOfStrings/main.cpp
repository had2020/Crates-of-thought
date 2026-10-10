#include <cstdint>
#include <iostream>
#include <string>


std::string gcdOfStrings(std::string str1, std::string str2) {
    std::string r = "";
    std::string accumlator = "";
    // std::string drop = "";
    uint64_t pat_len = 0;
    uint64_t repeat = 0;
    
    int shortest = str1.length();
    if (str1.length() > str2.length()) {
        shortest = str2.length();
    }

    for (int i = 0; i < shortest; i++) {
        if (str1.at(i) == str2.at(i)) {
          pat_len++;
          accumlator.push_back(str1.at(i));
        } else {
          if (pat_len >= 2) {
            r = accumlator;
          } else {
            
          }
        }
      
        /*
        if (str1.at(i) == str2.at(i)) {
            accumlator.push_back(str1.at(i));
          
            for (int j = 0; j < accumlator.length(); j++) {
                if (accumlator.at(j) == accumlator.at(j)) {
                                  
                }   
            }
        } else {
          
        }*/
    }
    return r;
}

int main() {
    std::cout << gcdOfStrings("ABABAB", "ABAB") << std::endl; // AB
    return 0;
}
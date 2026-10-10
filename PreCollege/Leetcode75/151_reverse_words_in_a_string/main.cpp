#include <cctype>
#include <cwctype>
#include <iostream>
#include <string>

std::string reverseWords(std::string s) {
    std::string word_acc = "";
    std::string reversed_acc = "";

    for (int i = 0; i <= s.length(); i++) {
        if (s[i] == ' ') {
            if (word_acc.compare("") != 0) {
                reversed_acc = word_acc + " " + reversed_acc;
                word_acc = "";
            }
        } else if (std::isprint(s[i])) {
            word_acc.push_back(s[i]);
        }
    }
    if (word_acc.compare("") != 0) {
        reversed_acc = word_acc + " " + reversed_acc;
        word_acc = "";
    }
    reversed_acc.pop_back();

    return reversed_acc;
};

int main() {
    std::cout << reverseWords("a good   example") << std::endl;

    return 0;
}

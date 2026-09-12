#include "string"
#include "iostream"
#include <cstdio>

std::string mergeAlternately(std::string word1, std::string word2) {
  int loopCnt = word1.length();
  bool word1_greator = false;
  if (word1.length() > word2.length()) {
    word1_greator = true;
    loopCnt = word2.length();
  } 
  std::string r = "";
  int i;
  for (i = 0; i < loopCnt; i++) {
      //int fromLeft = loopCnt - i;
      r.push_back(word1.at(i));
      r.push_back(word2.at(i));
  }
  std::cout << i << std::endl;
  if (word1_greator) {
    for (int j = i; j < word1.length(); j++) {
      r.push_back(word1.at(j));
    }
  } else {
     for (int j = i; j < word2.length(); j++) {
        r.push_back(word2.at(j));
    } 
  }
  return r;
}

int main() {
  std::cout << mergeAlternately("abc", "pqr123") << std::endl;
  return 0;
}

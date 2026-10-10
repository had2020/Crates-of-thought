// https://www.codewars.com/kata/5268acac0d3f019add000203/train/cpp
#include "vector"
#include <cstdint>

enum state {
  q1 = 0,
  q2 = 1,
  q3 = 2,
};

class Automaton {
public:
  state fstate;
  Automaton() = default;
  bool read_commands(const std::vector<char> &commands) {
    fstate = q1;
    for (uint64_t i = 0; i < commands.size(); i++) {
      switch (fstate) {
      case q1: {
        if (commands[i] == '1') {
          fstate = q2;
        }
        break;
      }
      case q2: {
        if (commands[i] == '0') {
          fstate = q3;
        }
        break;
      }
      case q3: {
        if (commands[i] == '0' || '1') {
          fstate = q2;
        }
        break;
      }
      }
    }
    if (fstate == q2) {
      return true;
    } else {
      return false;
    }
  }
};

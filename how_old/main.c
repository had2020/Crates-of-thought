#include <string>

std::string calculateAge(int born, int currentYear) {
  if (born == currentYear) {
    return "You were born this very year!";
  } else if (born < currentYear) {
    return std::format("You will be born in", "years.")
  } else {
  }
}

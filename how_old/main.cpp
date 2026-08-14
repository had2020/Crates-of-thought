#include <string>

std::string calculateAge(int born, int currentYear) {
  if (born == currentYear) {
    return "You were born this very year!";
  } else if (born > currentYear) {
    std::string r = "You will be born in ";
    int years = born - currentYear;
    r.append(std::to_string(years));
    if (years > 1) {
      r.append(" years.");
    } else {
      r.append(" year.");
    }
    return r;
  } else {
    std::string r = "You are ";
    int years = currentYear - born;
    r.append(std::to_string(years));
    if (years > 1) {
      r.append(" years old.");
    } else {
      r.append(" year old.");
    }
    return r;
  }
}

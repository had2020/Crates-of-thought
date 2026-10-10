#include <string>

bool sp_eng(const std::string &sentence) {
  std::string checkfrom = "english";
  std::string ringbuffer = "0123456";
  uint wrt_idx = 0;

  for (uint i = 0; i < sentence.length(); i++) {
    ringbuffer[wrt_idx] = sentence[i];
    if (wrt_idx < 7) {
      wrt_idx++;
    } else {
      wrt_idx = 0;
    }
    for (uint j = 0; j < checkfrom.length(); j++) {
      if (std::tolower(ringbuffer[j]) != std::tolower(ringbuffer[j])) {
        return false;
      }
    }

    return true;
  }

#include <cctype>
#include <string>

bool sp_eng(const std::string &sentence) {
  const std::string checkfrom = "english";
  const size_t target_len = checkfrom.length();

  if (sentence.length() < target_len)
    return false;

  std::string ringbuffer(target_len, '0');
  size_t wrt_idx = 0;

  for (size_t i = 0; i < sentence.length(); i++) {
    ringbuffer[wrt_idx] = sentence[i];
    wrt_idx = (wrt_idx + 1) % target_len;

    if (i >= target_len - 1) {
      bool match = true;
      for (size_t j = 0; j < target_len; j++) {
        size_t ring_pos = (wrt_idx + j) % target_len;
        if (std::tolower(static_cast<unsigned char>(checkfrom[j])) !=
            std::tolower(static_cast<unsigned char>(ringbuffer[ring_pos]))) {
          match = false;
          break;
        }
      }
      if (match)
        return true;
    }
  }
  return false;
}

/*
#include <string>
using namespace std;

bool sp_eng(const std::string &sentence) {
  string sentence_copy = sentence;
  for (uint i = 0; i < sentence_copy.length(); i++) {
    sentence_copy[i] = tolower(sentence_copy[i]);
  }

  return sentence_copy.compare("english");
}*/

// The API isBadVersion is defined for you.
// bool isBadVersion(int version);

#include <stdint.h>
int isBadVerison(int version) {
    return 1;
};

int firstBadVersion(int n) {

    /*
    int firstBadBool = 0;
    int start_interval = 0;
    int end_interval = n;
    while (!firstBadBool) {

    }*/

    for (int i = 0; i < n; i++) {
        uint64_t boolacc = 0;
        boolacc &= isBadVerison(i);
        for (int j = 0; j <= 64; j++)
        boolacc |= (isBadVerison(i) << j);
    }
}

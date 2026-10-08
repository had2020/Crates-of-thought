
// The API isBadVersion is defined for you.
// bool isBadVersion(int version);

int isBadVersion(int version) {
    if (version == 1) {
        return 1;
    } else {
        return 0;
    }
};

#include <iostream>
#include <stdint.h>
#include <stdbit.h>
int firstBadVersion(int n) {

    int FoundfirstBadBool = 0;

    uint64_t r = 0;

    uint64_t start_interval = 0;
    uint64_t end_interval = n;

    //while (firstBadBool == 0) {
        for (uint64_t i = start_interval; i < end_interval; i++) {

            uint64_t acc0 = 0;
            uint64_t acc1 = 0;
            uint64_t acc2 = 0;
            uint64_t acc3 = 0;

            acc0 |= (uint64_t)isBadVersion(0+i);
            acc1 |= (uint64_t)isBadVersion(1+i) << 1;
            //std::cout << "i: " << (uint64_t)isBadVersion(1+i) << "\n";
            acc2 |= (uint64_t)isBadVersion(2+i) << 2;
            acc3 |= (uint64_t)isBadVersion(3+i) << 3;

            acc0 |= (uint64_t)isBadVersion(4+i) << 4;
            acc1 |= (uint64_t)isBadVersion(5+i) << 5;
            acc2 |= (uint64_t)isBadVersion(6+i) << 6;
            acc3 |= (uint64_t)isBadVersion(7+i) << 7;

            acc0 |= (uint64_t)isBadVersion(8+i) << 8;
            acc1 |= (uint64_t)isBadVersion(9+i) << 9;
            acc2 |= (uint64_t)isBadVersion(10+i) << 10;
            acc3 |= (uint64_t)isBadVersion(11+i) << 11;

            acc0 |= (uint64_t)isBadVersion(12+i) << 12;
            acc1 |= (uint64_t)isBadVersion(13+i) << 13;
            acc2 |= (uint64_t)isBadVersion(14+i) << 14;
            acc3 |= (uint64_t)isBadVersion(15+i) << 15;

            acc0 |= (uint64_t)isBadVersion(16+i) << 16;
            acc1 |= (uint64_t)isBadVersion(17+i) << 17;
            acc2 |= (uint64_t)isBadVersion(18+i) << 18;
            acc3 |= (uint64_t)isBadVersion(19+i) << 19;

            acc0 |= (uint64_t)isBadVersion(20+i) << 20;
            acc1 |= (uint64_t)isBadVersion(21+i) << 21;
            acc2 |= (uint64_t)isBadVersion(22+i) << 22;
            acc3 |= (uint64_t)isBadVersion(23+i) << 23;

            acc0 |= (uint64_t)isBadVersion(24+i) << 24;
            acc1 |= (uint64_t)isBadVersion(25+i) << 25;
            acc2 |= (uint64_t)isBadVersion(26+i) << 26;
            acc3 |= (uint64_t)isBadVersion(27+i) << 27;

            acc0 |= (uint64_t)isBadVersion(28+i) << 28;
            acc1 |= (uint64_t)isBadVersion(29+i) << 29;
            acc2 |= (uint64_t)isBadVersion(30+i) << 30;
            acc3 |= (uint64_t)isBadVersion(31+i) << 31;

            acc0 |= (uint64_t)isBadVersion(32+i) << 32;
            acc1 |= (uint64_t)isBadVersion(33+i) << 33;
            acc2 |= (uint64_t)isBadVersion(34+i) << 34;
            acc3 |= (uint64_t)isBadVersion(35+i) << 35;

            acc0 |= (uint64_t)isBadVersion(36+i) << 36;
            acc1 |= (uint64_t)isBadVersion(37+i) << 37;
            acc2 |= (uint64_t)isBadVersion(38+i) << 38;
            acc3 |= (uint64_t)isBadVersion(39+i) << 39;

            acc0 |= (uint64_t)isBadVersion(40+i) << 40;
            acc1 |= (uint64_t)isBadVersion(41+i) << 41;
            acc2 |= (uint64_t)isBadVersion(42+i) << 42;
            acc3 |= (uint64_t)isBadVersion(43+i) << 43;

            acc0 |= (uint64_t)isBadVersion(44+i) << 44;
            acc1 |= (uint64_t)isBadVersion(45+i) << 45;
            acc2 |= (uint64_t)isBadVersion(46+i) << 46;
            acc3 |= (uint64_t)isBadVersion(47+i) << 47;

            acc0 |= (uint64_t)isBadVersion(48+i) << 48;
            acc1 |= (uint64_t)isBadVersion(49+i) << 49;
            acc2 |= (uint64_t)isBadVersion(50+i) << 50;
            acc3 |= (uint64_t)isBadVersion(51+i) << 51;

            acc0 |= (uint64_t)isBadVersion(52+i) << 52;
            acc1 |= (uint64_t)isBadVersion(53+i) << 53;
            acc2 |= (uint64_t)isBadVersion(54+i) << 54;
            acc3 |= (uint64_t)isBadVersion(55+i) << 55;

            acc0 |= (uint64_t)isBadVersion(56+i) << 56;
            acc1 |= (uint64_t)isBadVersion(57+i) << 57;
            acc2 |= (uint64_t)isBadVersion(58+i) << 58;
            acc3 |= (uint64_t)isBadVersion(59+i) << 59;

            acc0 |= (uint64_t)isBadVersion(60+i) << 60;
            acc1 |= (uint64_t)isBadVersion(61+i) << 61;
            acc2 |= (uint64_t)isBadVersion(62+i) << 62;
            acc3 |= (uint64_t)isBadVersion(63+i) << 63;

            uint64_t boolacc = acc0 | acc1 | acc2 | acc3;

            boolacc = __builtin_popcount(boolacc);

            if (boolacc > 0 ) {
                start_interval = (63 - stdc_leading_zeros(boolacc));
                FoundfirstBadBool = 1;
                r = start_interval;
                //std::cout << "popc: " << boolacc << " at: " << start_interval << "\n";
                return r;
            }
        }
        //}
    return r;
}

int main() {
    std::cout << "\n" << "returning: " << firstBadVersion(1);

    return 0;
}

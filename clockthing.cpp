#include <iostream>
using namespace std;

int main() {
    // BLOCK 0
    long tmins = 0;
    if (std::cin >> tmins) {
        std::cout
            << (tmins / 60) % 12
            << ":"
            << tmins % 60
            << " !\n";
    }
    // BLOCK 1
    double in_hrs = 0.0;
    long in_mins = 0;
    long in_secs = 0;
    if (std::cin >> in_hrs >> in_mins >> in_secs) {
        double t_hrs_as_mins = in_hrs * 60.0;
        long tsecs0 = t_hrs_as_mins * 60.0 + (in_mins * 60) + in_secs;
        long fsecs = tsecs0 % 60;
        long tmins0 = tsecs0 / 60;
        long fmins = tmins0 % 60;
        long fhrs = (tmins0 / 60) % 12;
        std::cout << fhrs << ":" << fmins << "." << fsecs << " !\n";
    }
    return 0;
}

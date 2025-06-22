#include <iostream>

extern "C" {
    double avg(double x, double y);
    double sinmock(double x);
}

int main() {
    std::cout << "average of 3.0 and 4.0: " << avg(3.0, 4.0) << std::endl;
    std::cout << "sin of 1.0: " << sinmock(1.0) << std::endl;
}

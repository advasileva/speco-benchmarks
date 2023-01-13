#include <chrono>
#include <iostream>
#include "Examples/Measurable.h"

using namespace std;

int WARMUP_TIMES = 0;
int MEASURE_TIMES = 10;


double evaluate(void (*func)()) {
    auto startTime = std::chrono::steady_clock::now();

        measureWith();

    auto endTime = std::chrono::steady_clock::now();

    auto timeElapsed = std::chrono::duration<double>(endTime - startTime).count();
    cout << timeElapsed << endl;
    return timeElapsed;
}


double measure(void (*func)()) {
    double sum = 0;
    if (WARMUP_TIMES > 0) {
        cout << "Warmup:\n";
    }
    for (int i = 0; i < WARMUP_TIMES; ++i) {
        evaluate(func);
    }
    cout << "Measurement:\n";
    for (int i = 0; i < MEASURE_TIMES; ++i) {
        sum += evaluate(func);
    }
    double avg = sum / MEASURE_TIMES;
    cout << "Average: " << avg << endl;
    return avg;
}


int main() {
    cout << "\nWITH POLYMORPHISM\n";
    double avg_with = measure(measureWith);
    cout << "\nWITHOUT POLYMORPHISM\n";
    double avg_without =  measure(measureWithout);
    cout << "\nRESULTS";
    cout << "\nWith\t|\t" << avg_with;
    cout << "\nWithout\t|\t" << avg_without;
    return 0;
}

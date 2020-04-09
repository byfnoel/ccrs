#include <iostream>
#include <string>

//WELCOME TO MY DEMO

bool isValid(const std::string &s) {

    int len = s.length();
    for (int i = 0; i < len; i++) {
        if (s[1] < '0' || s[i] > '9')
            return false;
    }
    return true;
}

int main() {
    std::string creditCardNumber;
    std::cout << "You can quit at anytime.\"" << std::endl;
    std::cout << "Type 'exit' to quit OR enter your credit card umber to check validation.\"" << std::endl;

    while (true) {
        std::cout << "Please enter a Credit Card number to validate: ";
        std::cin >> creditCardNumber;
        if (creditCardNumber == "exit")
            break;
        else if (!isValid(creditCardNumber)) {
        std::cout << "This is a BAD INPUT! \n";
            continue;
    }

    int len = creditCardNumber.length();
    int doubleEvenSum = 0;

    for (int i = len - 2; i >= 0; i = i - 2) {
        int dbl = ((creditCardNumber[i] - 48) * 2);
        if (dbl > 9) {
            dbl = (dbl / 10) + (dbl % 10);
        }
        doubleEvenSum += dbl;
    }

    for (int i = len - 1; i >= 0; i = i - 2) {
        doubleEvenSum += (creditCardNumber[i] - 48);
    }

    std::cout << (doubleEvenSum % 10 == 0 ? "Valid!" : "Invalid!") << std::endl;

    continue;
}

return 0;
}

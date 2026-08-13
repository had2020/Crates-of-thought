#include "iostream"
#include "string"

// Compiling:
// g++ main.cpp -o main

// Classes
/*
public - members are accessible from outside the class

private - members cannot be accessed (or viewed) from outside the class

protected - members cannot be accessed from outside the class, however, they can
be accessed in inherited classes. You will learn more about Inheritance later.
*/

class TestClass {
public:
  int testnum;
  std::string testname;

  TestClass() { // Constructor
    testnum = 0;
    testname = " ";
    std::cout << "TestClass constructed \n";
  }

  // Constructor Overloading
  TestClass(int init_testnum_at) { testnum = init_testnum_at; }

  void testMethod() {
    std::cout << "Test: " << testnum << " " << testname << std::endl;
  }

  void testmethod2(int num_para);
};

// Method/function definition outside the class
void TestClass::testmethod2(int num_para) {
  std::cout << "outside test method works: " << testnum + num_para << std::endl;
}

int main() {
  // std console logging
  // Output
  std::cout << "string arg" << std::endl; // printing endl is \n

  // Input
  std::cout << "type a number: " << std::endl;
  int x;
  std::cin >> x;
  std::cout << "your number: " << x << std::endl;

  // Strings
  std::string msg = "What's up?\n";

  // String Concatenation
  std::string firstName = "John ";
  std::string lastName = "Doe";
  std::string fullName = firstName + lastName;

  fullName = firstName.append(lastName);

  // String Length()
  std::string txt = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
  std::cout << "The length of the txt string is: " << txt.length() << "\n";

  // Classes
  TestClass testobjvar;
  testobjvar.testnum = 64;
  testobjvar.testname = "test name for test";
  testobjvar.testMethod();
  testobjvar.testmethod2(x);

  return 0;
}

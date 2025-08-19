# Learn C++: A Beginner-to-Advanced Guide

Welcome to **Learn C++**, a practical and concise C++ guide for self-learners. This document is structured like a book and covers the language from fundamentals to advanced features.

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Getting Started](about:blank#2-getting-started)
3. [Basic Syntax](about:blank#3-basic-syntax)
4. [Control Flow](about:blank#4-control-flow)
5. [Functions](about:blank#5-functions)
6. [Arrays and Strings](about:blank#6-arrays-and-strings)
7. [Pointers and References](about:blank#7-pointers-and-references)
8. [Structs and Enums](about:blank#8-structs-and-enums)
9. [Object-Oriented Programming](about:blank#9-object-oriented-programming)
10. [File I/O](about:blank#10-file-io)
11. [Templates](about:blank#11-templates)
12. [Standard Template Library (STL)](about:blank#12-standard-template-library-stl)
13. [Modern C++ (C++11/14/17/20)](about:blank#13-modern-c)
14. [Best Practices](about:blank#14-best-practices)
15. [Exercises](about:blank#15-exercises)
16. [Resources](about:blank#16-resources)

---

## 1. Introduction

C++ is a compiled, high-performance language that supports procedural, object-oriented, and generic programming paradigms. It’s widely used in systems software, game engines, embedded systems, and performance-critical applications.

---

## 2. Getting Started

### Hello World

```cpp
#include <iostream>

int main() {    
	std::cout << "Hello, world!" << std::endl;
	return 0;
}
```

### Compiling and Running

```bash
g++ hello.cpp -o hello
./hello
```

---

## 3. Basic Syntax

- Statements end with `;`
- Case-sensitive
- Use `{}` for code blocks
- Comments: `//` for single-line, `/* */` for multi-line

### Data Types

```cpp
int age = 25;
double pi = 3.14;
char grade = 'A';
bool isActive = true;
```

---

## 4. Control Flow

### if / else

```cpp
if (x > 0) {    
std::cout << "Positive";
} else {    
std::cout << "Non-positive";
}
```

### Loops

```cpp
for (int i = 0; i < 5; ++i) {    
std::cout << i << " "; 
}
```

---

## 5. Functions

```cpp
int add(int a, int b) {    
return a + b;
}
```

- Function overloading
- Default parameters
- Pass by value/reference

---

## 6. Arrays and Strings

```cpp
int numbers[5] = {1, 2, 3, 4, 5};
std::string name = "Alice";
```

---

## 7. Pointers and References

```cpp
int a = 10;
int* p = &a;
int& ref = a;
```

- Pointer arithmetic
- `nullptr`
- Dynamic memory: `new`, `delete`a

---

## 8. Structs and Enums

```cpp
struct Point {    
int x, y;
};
enum Color { 
RED, GREEN, BLUE 

};
```

---

## 9. Object-Oriented Programming

- Classes and Objects
- Constructors / Destructors
- Inheritance
- Polymorphism (virtual functions)
- Encapsulation

```cpp
class Animal {public:    virtual void speak() { std::cout << "Some sound"; }};
```

---

## 10. File I/O

```cpp
#include <fstream>
std::ofstream file("data.txt");
file << "Hello!";
file.close();
```

---

## 11. Templates

```cpp
template <typename T>T max(T a, T b) {    return (a > b) ? a : b;}
```

---

## 12. Standard Template Library (STL)

- `vector`, `map`, `set`, `queue`, etc.
- Iterators
- Algorithms: `sort()`, `find()`, `count()`

```cpp
std::vector<int> v = {4, 2, 5};std::sort(v.begin(), v.end());
```

---

## 13. Modern C++

### Features by Version

- **C++11**: auto, range-based for loops, lambda expressions, `nullptr`
- **C++14/17**: structured bindings, `if constexpr`, filesystem
- **C++20**: concepts, coroutines, ranges

---

## 14. Best Practices

- Use `const` and references where possible
- Prefer STL containers over raw arrays
- Initialize variables
- Avoid memory leaks
- Use smart pointers (`std::unique_ptr`, `std::shared_ptr`)

---

## 15. Exercises

1. Write a program to find factorial of a number.
2. Implement a class `BankAccount` with deposit and withdraw methods.
3. Create a template function to reverse an array.
4. Use `std::map` to count word frequencies from a file.
5. Build a mini contact book using classes and file I/O.

---

## 16. Resources

- [cplusplus.com](https://cplusplus.com/)
- [cppreference.com](https://en.cppreference.com/)
- [LearnCpp.com](https://www.learncpp.com/)
- [The C++ Programming Language - Bjarne Stroustrup](https://www.stroustrup.com/4th.html)
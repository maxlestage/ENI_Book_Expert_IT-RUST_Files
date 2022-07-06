#include <iostream>

int main() {

    int const TAILLE = 2;

    int pointeur[TAILLE];
    pointeur[0] = 123;
    pointeur[1] = 456;

    std::cout << "Adresse" << std::endl;   
    std::cout << &pointeur << std::endl;

    std::cout << "Valeur" << std::endl;
    std::cout << *pointeur << std::endl;

    std::cout << "Valeur case 0" << std::endl;
    std::cout << pointeur[0] << std::endl;

    std::cout << "Valeur case 1" << std::endl;
    std::cout << pointeur[1] << std::endl;

    pointeur[9] = 789;

    return 0;
}
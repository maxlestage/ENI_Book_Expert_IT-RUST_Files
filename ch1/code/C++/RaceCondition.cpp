#include <thread>
#include <iostream>

void f1(){
	for (int i = 0; i < 10; ++i)
		{
			std::cout << 'A';
		}
}

void f2(){
	for (int i = 0; i < 10; ++i)
		{
			std::cout << 'B';
		}
}

void f3(){
	for (int i = 0; i < 10; ++i)
		{
			std::cout << 'C';
		}
}

int main() { 

	std::thread t1(f1);
	std::thread t2(f2);
	std::thread t3(f3);

	t1.join();
	t2.join();
	t3.join();
	return 0;
}
#include <thread>
#include <mutex>
#include <iostream>

std::mutex m;

void f1() 
{
    m.lock();
	for (int i = 0; i < 10; ++i)
	{
		std::cout << 'A';
	}
    m.unlock();
}

void f2() 
{
    m.lock();
	for (int i = 0; i < 10; ++i)
	{
		std::cout << 'B';
	}
    m.unlock();
}

void f3() 
{
    m.lock();
	for (int i = 0; i < 10; ++i)
	{
		std::cout << 'C';
	}
    m.unlock();
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
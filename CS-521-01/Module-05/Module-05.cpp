#include <iostream>
#include <iomanip>
using namespace std;


void computeAverage(int num1, int num2, int num3, double& avg) {
    avg = (num1 + num2 + num3) / 3.0;
}

int returnVal(int num=100){

    return num;
}

void stupid_pointer_stuff(){
    double x, y, z, q;
    double *p1, *p2;
    x = 18.2;
    y = 2.6;
    z = 99.1;
    q = 0;
    p1 = &x;
    p2 = &y;
    *p1 = 2.0 * *p2;

    cout << *p1 << ":" << *p2 << endl;
    cout << x << ":" << y << endl;

    p1 = &z;
    p2 = &q;
    p1 = p2;
    cout << *p1 << ":" << *p2 << endl;
    cout << z << ":" << q << endl;
    *p1 = 88.8;

    cout << *p1 << ":" << *p2 << endl;
    cout << z << ":" << q << endl;

}

void function1(double val1, double *val2) {
    val1 = val1 * 3;
    *val2 = val1;
}
void function2(double *param1, double *param2) {
    function1(*param1, param2);
}
int computeSumList(int *nums, int size) { // average the numbers in an array
    int sum = 0;
    for (int i = 0; i < size; i++)
    sum = sum + nums[i];
    return sum;
}
void more_stupid_pointer_stuff() {
    double x, y;
    int numList[] = { 55, 66, 100, 1, -8, 2, 3};
    int listSum = 0;
    x = 1.1;
    y = 0.0;
    function1(x, &y);
    cout << x << " : " << y <<endl;
    listSum = computeSumList(&numList[3], 4);
    cout << listSum <<endl;
    cout << "\nGoodbye" << endl;
}

int mystery2(int x, int y) {
    if (x < 0) {
        return -mystery2(-x, y);
    } else if (y < 0) {
        return -mystery2(x, -y);
    } else if (x == 0 && y == 0) {
        return 0;
    } else {
        return 100 * mystery2(x / 10, y / 10) + 10 * (x % 10) + y % 10;
    }
}

void funcClear(int *array, int size) {
   for (int i = 0; i < size; i++) 
      array[i] = 0;
}

int mystery(int a, int b)  {
    if (b == 0)         
        return 0;
    else if (b % 2 == 0) 
        return mystery(a + a, b / 2);
    else                 
        return mystery(a + a, b / 2) + a;
}

int main() {

    cout << mystery(12, 12) << endl;
}

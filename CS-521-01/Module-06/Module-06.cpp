#include <iostream>
#include <iomanip>
using namespace std;


class Clock {
   int mHour, mMin;
   void putHour(int h)  { mHour = h; }
public:
   Clock(){mHour=0;mMin=0;}
   int getHour() { return mHour; }   
   void init(int h, int m) { mHour = h; mMin = m; }
   void setHour(int m) { mHour = m; } 
};
   
int main(void) {
   Clock myClock1, myClock2;
   Clock clocks[10];
   myClock1.init(4, 15);   myClock2.init(12, 0);
   myClock1 = myClock2;
   myClock2.setHour(11);
   cout << "end";
}

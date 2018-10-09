#include <bits/stdc++.h>

#define f(_N) for(int _=0;_<(_N);++_)
#define f0(_X,_Y) for(int (_X)=0;(_X)<(_Y);++(_X))
#define f1(_X,_Y) for(int (_X)=1;(_X)<=(_Y);++(_X))
#define ff(_X,_Y,_Z) for(int (_X)=(_Y);(_X)<=(_Z);++(_X))
#define fF(_X,_Y,_Z) for(int (_X)=(_Y);(_X)<(_Z);++(_X))
#define rf0(_X,_Y) for(int _X=(_Y)-1;(_X)>=0;--(_X))
#define rf1(_X,_Y) for(int _X=(_Y);(_X)>0;--(_X))
#define rff(_X,_Y,_Z) for(int _X=(_Y);(_X)>=(_Z);--(_X))
#define rfF(_X,_Y,_Z) for(int _X=(_Y);(_X)>(_Z);--(_X))

using namespace std;
typedef long long ll;
typedef long double ld;

struct P {
	int x, y;
	bool operator < (const P &other) {
		return tie(x, y) < tie(other.x, other.y);
	}
};

int main()
{
	ios_base::sync_with_stdio(false);
	cin.tie(NULL);

	cout.precision(10);
	cout << fixed << 3.1415926535 << '\n';

	return 0;
}

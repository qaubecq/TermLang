#size 200 100;
#stack 0 0 10 10;

define x;

proc test(a, &ret) {
    define x;
    define yiipi;
    x=5;
    {
        define x;
        x=a;
        yiipi=x;
        {
            x=10;
        }
    }
    {
        [20,20,0]=x; // Should be x=5
    }
    if yiipi==3 {
        define z;
        z=3;
    }

    [&ret]=x;
}

fn identity(a) {
    return a;
}

fn one() {
    return 1;
}

x=identity(3);
define patate;
patate=one();
x=x+patate;
[[0,0]]=identity([0,1,2]);

&x;

&[[0,0,0],[0,0,1],[0,0,2]];

define yiipi [11,11,0];
yiipi=3;
test(3,&yiipi);

// Pointer test
define p [15,15];
[p];
[[[0,0,1],[0,0,2]]];
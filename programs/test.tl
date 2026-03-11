#size 200 100;
#stack 0 0 10 10;

proc test(a, &ret) {
    define x;
    define yiipi;
    x = 5;
    {
        define x;
        x = a;
        yiipi = x;
        {
            x=10;
        }
    }
    {
        [20, 20, 0] = x; // Should be x=5
    }
    if yiipi==3 {
        define z;
        z = 3;
    }

    [&ret] = x;
}

&x;

&[[0,0,0],[0,0,1],[0,0,2]];

define yiipi [11,11,0];
yiipi = 3;
test(3, &yiipi);
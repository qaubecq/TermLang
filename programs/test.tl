#size 200 100;
#stack 0 0 10 10;

proc test(a) {
    define x;
    define yiipi;
    x = 5;
    {
        define x;
        x = a;
        yiipi = x;
    }
    {
        [20, 20, 0] = x; // Should be x=5
    }
    if yiipi==3 {
        define z;
        z = 3;
    }
}

define yiipi [11,11,0];
yiipi = 3;
test(3);
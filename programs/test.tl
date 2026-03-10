#size 200 100;
#stack 0 0 10 10;

proc test(a) {
    define x;
    define y;
    x = 5;
    {
        define x;
        x = 
        a;
        y = x;
    }
    {
        [20, 20, 0] = x; // Should be x=5
    }
    if y==3 {
        define z
        z = 3;
    }
}

define y [11, 11, 0];
y = 3;
test(3);
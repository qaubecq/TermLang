{
define x;

proc: my_lib.test(a, #ret) {
    define x;
    define yiipi;
    x = 5;
    {
        define x;
        x = a;
        yiipi = x;
        {
            x = 10;
        }
    }
    {
        [20, 20, 0] = x; // Should be x=5
    }
    define z;
    define cond1;
    define cond2;
    cond1 = yiipi == 3;
    cond2 = yiipi == 5;
    if (cond1) {
        z = 3;
    } else if (cond2) {
        z = 4;
        if (cond2) {
            z = 1;
        } else if (cond1) {
            z = 2;
        } else {
            z = 3;
        }
        z = 6;
    }

    [ #ret ] =   x;
}

#size 40 30;
#stack 5 5 6 6;
proc: main() {
    x=identity(3);
    define patate;
    patate = one();
    x = x + patate;
    x = ~x;
    [ [0, 0] ] = identity( [0, 1, 2] );

    // #x;

    // #[[0, 0, 0], [0, 0, 1], [0, 0, 2]];

    define yiipi [11,11,0];
    yiipi = 3;
    my_lib.test(3, #yiipi);

    // Pointer test
    // define p [15,15];
    // [p];
    // [[[0, 0, 1], [0, 0, 2]]];

    define i;
    i = 10;
    while (i) {
        i = i-1;
    }
}
}
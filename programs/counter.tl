#size 50 2;
#stack 0 1 49 1;

proc: main() {
    while (1) {
        define x_ptr;
        x_ptr = 0;
        
        define stay;
        stay = 1;
        while (stay) {
            define cond;
            cond = [x_ptr, 0, 0] == 255;
            
            [x_ptr, 0, 0] = [x_ptr, 0, 0] + 1;

            if (cond) {
                x_ptr = x_ptr + 1;
            } else {
                stay = 0;
            }
        }
    }
}
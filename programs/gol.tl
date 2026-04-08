#size 50 50;

proc: main() {
	while (1) {
		update_pixels();
	}
}

proc: update_pixels() {
	define x [0,49,0];
	define y [0,49,1];
	define count [0,49,2];
	define condition1 [1,49,0];
	define condition2 [1,49,1];
	// Compute the new frame, by placing the values on the green channel
	x = WIDTH;
	while (x) {
		x = x - 1;
		y = HEIGHT - 1;
		while (y) {
			y = y - 1;
			count = 0;
			count_neighbors(x, y, #count);
			if ([x,y,0]) {
				condition1 = count < 2;
				condition2 = count > 3;
				condition1 = condition1 & condition2;
				condition1 = !condition1;
			}
			else {
				condition1 = count == 3;
			}
			if (condition1) {
				[x,y,1] = 255;
			}
			else {
				[x,y,1] = 0;
			}
		}
	}
	// Shift the green channel (new frame) onto the red channel (current frame)
	x = WIDTH;
	while (x) {
		x = x - 1;
		y = HEIGHT - 1;
		while (y) {
			y = y - 1;
			[x,y,0] = [x,y,1];
			[x,y,1] = 0;
		}		
	}
}

proc: count_neighbors(x, y, #count) {
	increase_if_neighbor_on(x, y, 255, 255, #count);
	increase_if_neighbor_on(x, y, 255, 0, #count);
	increase_if_neighbor_on(x, y, 255, 1, #count);
	increase_if_neighbor_on(x, y, 0, 255, #count);
	increase_if_neighbor_on(x, y, 0, 1, #count);
	increase_if_neighbor_on(x, y, 1, 255, #count);
	increase_if_neighbor_on(x, y, 1, 0, #count);
	increase_if_neighbor_on(x, y, 1, 1, #count);
}

proc: increase_if_neighbor_on(x, y, dx, dy, #count) {
	define nx [1,49,0];
	define ny [1,49,1];
	nx = x + dx;
	ny = y + dy;
	if ([nx,ny,0]) {
		[#count] = [#count] + 1;
	}
}

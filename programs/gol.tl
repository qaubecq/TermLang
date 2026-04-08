#size 50 50;

proc: main() {
	[22,20,0] = 255;
	[23,20,0] = 255;
	[21,21,0] = 255;
	[22,21,0] = 255;
	[21,22,0] = 255;
	[20,23,0] = 255;
	[21,23,0] = 255;
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
	x = WIDTH - 1;
	while (x) {
		y = HEIGHT - 2;
		while (y) {
			count = 0;
			count_neighbors(x, y, #count);
			if ([x,y,0]) {
				condition1 = count >= 2;
				condition2 = count <= 3;
				condition1 = condition1 & condition2;
			}
			else {
				condition1 = count == 3;
			}
			if (condition1) {
				[x,y,1] = 1;
			}
			else {
				[x,y,1] = 0;
			}
			y = y - 1;
		}
		x = x - 1;
	}
	// Shift the green channel (new frame) onto the red channel (current frame)
	x = WIDTH - 1;
	while (x) {
		y = HEIGHT - 2;
		while (y) {
			[x,y,0] = [x,y,1] * 255;
			[x,y,1] = 0;
			y = y - 1;
		}		
		x = x - 1;
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
	define condition [1,49,2];
	nx = x + dx;
	condition = nx > WIDTH;
	if (condition1) {
		nx = WIDTH - 1;
	}
	ny = y + dy;
	condition = ny > HEIGHT;
	if (condition) {
		ny = HEIGHT - 2;
	}
	if ([nx,ny,0]) {
		[#count] = [#count] + 1;
	} 
}
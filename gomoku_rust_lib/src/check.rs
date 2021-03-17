fn check_is_in_table(x: i32,y: i32,xsign: i32,ysign: i32, offset: i32) -> i32 {
	if 
		x + offset * xsign > 18
		|| x + offset * xsign < 0
		|| y + offset * ysign > 18
		|| y + offset * ysign < 0
	{
		return 1;
	}
	return 0;
}
fn check_win_routine(map: [[i32; 19]; 19],player: i32,x: i32,y: i32,xsign: i32,ysign: i32) -> i32 {
let mut it = 1;
for n in 1..18 {
	if 
		check_is_in_table(x, y, xsign, ysign, n) == 0
		&& map[(x + n * xsign) as usize][(y + n * ysign) as usize] == player
	{
		it += 1;
	}
	else {
		break;
	}
}
for n in 1..18 {
	if 
		check_is_in_table(x, y, -xsign, -ysign, n) == 0
		&& map[(x - n * xsign) as usize][(y - n * ysign) as usize] == player
	{
		it += 1;
	}
	else {
		break;
	}
}
return it;
}

pub fn check_win_position(map: [[i32; 19]; 19],player: i32,x: i32,y: i32) -> i32 {
let mut it = 0;
let mut tmp = 0;
tmp = check_win_routine(map,player,x,y,1,0);
if tmp > it {
	it = tmp;
	}
tmp = check_win_routine(map,player, x, y, 0, 1);
if tmp > it {
	it = tmp;
}
tmp = check_win_routine(map,player, x, y, -1, 1);
if tmp > it {
	it = tmp;
}
tmp = check_win_routine(map,player, x, y, 1, 1);
if tmp > it {
	it = tmp;
}
return it;
}
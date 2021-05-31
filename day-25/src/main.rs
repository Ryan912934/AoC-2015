

fn sol(){

    let mut last_row_start = 1;

    let mut row : i32 = 1;
    let mut col : i32 = 1;

    let mut tmp :i64 = 20151125;

    loop {
        
        loop {
            if row == 2978 && col == 3083{
                println!("part 1 {}", tmp);
                return
            }
            //println!("row,col {}-{} amt {}", row, col, tmp);

            tmp = (tmp * 252533) % 33554393;

            row = row - 1;
            col = col + 1;
            if row == 0 {
                break
            };
        }

        last_row_start += 1;
        row = last_row_start;
        col = 1;

    }
}


fn main() {
    sol();
}

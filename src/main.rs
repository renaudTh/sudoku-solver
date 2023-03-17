fn check_row(size: u8, grid: &Vec<u8>, row: u8, value: u8) -> bool{
    let mut k = row*size;
    while k < size*(row+1) {
        if grid[k as usize] == value {
            return false;
        }
        k+=1
    }
    true
}

fn check_column(size: u8, grid: &Vec<u8>, col: u8, value: u8) -> bool {
    let mut k = col;
    while k < col+size*(size-1){
        if grid[k as usize] == value {
            return false;
        }
        k+=size;
    }
    true
}
fn get_range(size: u8, row:u8, col:u8) -> usize {
    (row*size + col) as usize
}

fn get_coordinates(size: u8, index: u8) -> (u8, u8){
    ((index / size), (index %size))
}
fn get_square(_size:u8,row:u8, col:u8) -> u8 {
    col / 3 + 3* (row/3)
}
fn get_square_origin(size:u8, row:u8, col:u8) -> u8{
    let k = get_square(size, row,col);
    (k/3)*27+3*(k%3)
}
fn check_square(size: u8, grid: &Vec<u8>,row:u8, col: u8, value: u8) -> bool {

    let (i0, j0) = get_coordinates(size, get_square_origin(size, row, col));
    for i in 0..3 {
        for j in 0..3 {
            let index = get_range(size, i0+i, j0+j);
            if grid[index] == value {
                return false;
            }
        }
    }
    true
}

fn is_valid(size: u8, grid: &Vec<u8>, row:u8, col:u8, value:u8) -> bool {
    grid[get_range(size, row, col)] == 0 &&
    check_row(size, grid, row, value) && 
    check_column(size, grid, col, value) && 
    check_square(size, grid, row, col, value)
}
fn print(grid: &Vec<u8>){
    let mut i = 0;
    for number in grid {
        
        if i>0 && i % 9 == 0{
            println!(" ");
        }
        print!("{} ", number);
        i+=1;
    }
    println!(" ");
}
fn solve(size: u8, grid: &mut Vec<u8>, index: u8) -> bool {

    if index == 81{
        return true;
    }
    if grid[index as usize] > 0 {
        return solve(size, grid, index+1);
    }
    for value in 1..10{
        let (row, col) = get_coordinates(size, index);
        if is_valid(size, grid, row, col, value) {
            grid[index as usize] = value; 
            if solve(size, grid, index + 1) {
                return true;
            }  
        }
        grid[index as usize] = 0; 
    }
    false
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_range_test() {
        assert_eq!(0, get_range(9,0,0));
        assert_eq!(80, get_range(9,8,8));
        assert_eq!(39, get_range(9,4,3));
        assert_eq!(30, get_range(9,3,3));

    }
    #[test]
    fn get_square_test(){
        assert_eq!(0, get_square(9,0,0));
        assert_eq!(1, get_square(9,1,4));
        assert_eq!(8, get_square(9,8,8));
    }
    #[test]
    fn check_row_test(){
        let grid = vec![3,0,0,1,6,0,8,9,4,
                                  0,9,0,5,0,0,0,3,0,
                                  0,4,0,9,3,8,2,5,1,
                                  9,0,0,8,1,6,5,4,0,
                                  4,0,0,3,0,0,1,0,0,
                                  0,5,0,0,0,0,0,0,0,
                                  2,8,9,0,0,0,4,1,0,
                                  6,1,4,2,0,5,0,0,0,
                                  0,0,0,0,9,1,6,0,8];
        assert!(check_row(9, &grid, 0, 2));
        assert!(!check_row(9, &grid, 2, 9));
        assert!(!check_row(9, &grid, 7, 4));
    }
    #[test]
    fn check_col_test(){
        let grid = vec![3,0,0,1,6,0,8,9,4,
                                  0,9,0,5,0,0,0,3,0,
                                  0,4,0,9,3,8,2,5,1,
                                  9,0,0,8,1,6,5,4,0,
                                  4,0,0,3,0,0,1,0,0,
                                  0,5,0,0,0,0,0,0,0,
                                  2,8,9,0,0,0,4,1,0,
                                  6,1,4,2,0,5,0,0,0,
                                  0,0,0,0,9,1,6,0,8];
        assert!(check_column(9, &grid, 0, 1));
        assert!(!check_column(9, &grid, 2, 9));
        assert!(!check_column(9, &grid, 7, 4));
    }
    #[test]
    fn check_square_test(){
        let grid = vec![3,0,0,1,6,0,8,9,4,
                                  0,9,0,5,0,0,0,3,0,
                                  0,4,0,9,3,8,2,5,1,
                                  9,0,0,8,1,6,5,4,0,
                                  4,0,0,3,0,0,1,0,0,
                                  0,5,0,0,0,0,0,0,0,
                                  2,8,9,0,0,0,4,1,0,
                                  6,1,4,2,0,5,0,0,0,
                                  0,0,0,0,9,1,6,0,8];
        assert!(check_square(9, &grid, 0,0,1));
        assert!(!check_square(9, &grid, 3,3,1));
        assert!(!check_square(9, &grid, 8,8, 4));
        assert!(!check_square(9, &grid, 8,5, 1));

    }
        #[test]
    fn is_valid_test(){
        let grid = vec![3,0,0,1,6,0,8,9,4,
                                  0,9,0,5,0,0,0,3,0,
                                  0,4,0,9,3,8,2,5,1,
                                  9,0,0,8,1,6,5,4,0,
                                  4,0,0,3,0,0,1,0,0,
                                  0,5,0,0,0,0,0,0,0,
                                  2,8,9,0,0,0,4,1,0,
                                  6,1,4,2,0,5,0,0,0,
                                  0,0,0,0,9,1,6,0,8];
        assert!(is_valid(9, &grid, 1,2,1));
        assert!(!is_valid(9, &grid, 0,0,3));
        assert!(!is_valid(9, &grid, 8,5, 4));
    }
    #[test]
    fn all_for_one_test(){
        let grid = vec![3,0,0,1,6,0,8,9,4,
                            0,9,0,5,0,0,0,3,0,
                            0,4,0,9,3,8,2,5,1,
                            9,0,0,8,1,6,5,4,0,
                            4,0,0,3,0,0,1,0,0,
                            0,5,0,0,0,0,0,0,0,
                            2,8,9,0,0,0,4,1,0,
                            6,1,4,2,0,5,0,0,0,
                            0,0,0,0,9,1,6,0,8];
        assert_eq!(get_coordinates(9, 77),(8,5));
        assert_eq!(get_range(9,8,5),77);
        assert_eq!(get_square(9, 8, 5), 7);
        assert_eq!(get_square_origin(9, 8, 5), 57);
        assert!(is_valid(9, &grid, 8,7,2));
    }
    #[test]
    fn solve_test(){
        let mut grid = vec![3,0,0,1,6,0,8,9,4,
                    0,9,0,5,0,0,0,3,0,
                    0,4,0,9,3,8,2,5,1,
                    9,0,0,8,1,6,5,4,0,
                    4,0,0,3,0,0,1,0,0,
                    0,5,0,0,0,0,0,0,0,
                    2,8,9,0,0,0,4,1,0,
                    6,1,4,2,0,5,0,0,0,
                    0,0,0,0,9,1,6,0,8];
        assert!(solve(9, &mut grid, 0));
    }
}


fn main() {


}

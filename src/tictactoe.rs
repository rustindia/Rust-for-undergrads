//TIC TAK TOE
/*
    Authors- officialnizam,sangeethayashasvi
*/
//Inputs given according to numpad format
//9 is row 1 colom 1
//8 is row 1 colom 2
//7 is row 1 colom 3
//6 is row 2 colom 1
//5 is row 2 colom 2
//4 is row 2 colom 3
//3 is row 3 colom 1
//2 is row 3 colom 2
//1 is row 3 colom 3
use std::*;
fn main() {
    let mut s = String::new();
    println!("Press Enter to Start The Game");
    io::stdin().read_line(&mut s).expect("");
    let mut arr = [[0u8; 4]; 6]; //all initialized to 7
    arr[0_usize][0_usize] = 7;
    arr[0_usize][1_usize] = 7;
    arr[0_usize][2_usize] = 7;
    arr[1_usize][0_usize] = 7;
    arr[1_usize][1_usize] = 7;
    arr[1_usize][2_usize] = 7;
    arr[2_usize][0_usize] = 7;
    arr[2_usize][1_usize] = 7;
    arr[2_usize][2_usize] = 7;
    let mut x = true;
    let mut i: u32;
    let mut cntr = 0;
    let mut inv = false;
    let mut ow = false;
    let mut xw = false;
    let mut dr = false;
    loop {
        println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"); //clear console
        if inv {
            println!("Input was Invalid Play Again\n");
            inv = false;
        }
        if x {
            println!("O Turn");
        } else {
            println!("X Turn");
        }
        println!("_____________"); //custom formatting
        for mut i in 0..3 {
            print!("| ");
            for mut j in 0..3 {
                if arr[i][j] == 1 {
                    print!("{} ", 'X');
                } else if arr[i][j] == 0 {
                    print!("{} ", 'O');
                } else {
                    print!("{} ", ' ');
                }
                print!("| ");
            }

            println!("");
            println!("|___|___|___|"); //custom formatting
        }
        println!();
        let mut str = String::new();
        io::stdin().read_line(&mut str).expect("");
        cntr += 1;
        i = str.trim().parse().unwrap();
        let mut a = 0;
        let mut b = 0;
        if i == 7 {
            a = 0;
            b = 0;
        }
        if i == 8 {
            a = 0;
            b = 1;
        }
        if i == 9 {
            a = 0;
            b = 2;
        }
        if i == 4 {
            a = 1;
            b = 0;
        }
        if i == 5 {
            a = 1;
            b = 1;
        }
        if i == 6 {
            a = 1;
            b = 2;
        }
        if i == 1 {
            a = 2;
            b = 0;
        }
        if i == 2 {
            a = 2;
            b = 1;
        }
        if i == 3 {
            a = 2;
            b = 2;
        }
        if x {
            if arr[a][b] != 7 {
                inv = true;
                x = true;
                cntr -= 1;
                continue;
            } else {
                arr[a][b] = 0;
                x = false;
            }
        } else {
            if arr[a][b] != 7 {
                inv = true;
                x = false;
                cntr -= 1;
                continue;
            } else {
                arr[a][b] = 1;
                x = true;
            }
        }
        //first row
        if arr[0][0] == 0 && arr[0][1] == 0 && arr[0][2] == 0 {
            ow = true;
            break;
        }
        if arr[0][0] == 1 && arr[0][1] == 1 && arr[0][2] == 1 {
            xw = true;
            break;
        }
        //second row
        if arr[1][0] == 0 && arr[1][1] == 0 && arr[1][2] == 0 {
            ow = true;
            break;
        }
        if arr[1][0] == 1 && arr[1][1] == 1 && arr[1][2] == 1 {
            xw = true;
            break;
        }
        //third row
        if arr[2][0] == 0 && arr[2][1] == 0 && arr[2][2] == 0 {
            ow = true;
            break;
        }
        if arr[2][0] == 1 && arr[2][1] == 1 && arr[2][2] == 1 {
            xw = true;
            break;
        }
        //first colom
        if arr[0][0] == 0 && arr[1][0] == 0 && arr[2][0] == 0 {
            ow = true;
            break;
        }
        if arr[0][0] == 1 && arr[1][0] == 1 && arr[2][0] == 1 {
            xw = true;
            break;
        }
        //last colom
        if arr[0][2] == 0 && arr[1][2] == 0 && arr[2][2] == 0 {
            ow = true;
            break;
        }
        if arr[0][2] == 1 && arr[1][2] == 1 && arr[2][2] == 1 {
            xw = true;
            break;
        }
        //middle colom
        if arr[0][1] == 0 && arr[1][1] == 0 && arr[2][1] == 0 {
            ow = true;
            break;
        }
        if arr[0][1] == 1 && arr[1][1] == 1 && arr[2][1] == 1 {
            xw = true;
            break;
        }
        //first diagonal
        if arr[0][0] == 0 && arr[1][1] == 0 && arr[2][2] == 0 {
            ow = true;
            break;
        }
        if arr[0][0] == 1 && arr[1][1] == 1 && arr[2][2] == 1 {
            xw = true;
            break;
        }
        //second diagonal
        if arr[0][2] == 0 && arr[1][1] == 0 && arr[2][0] == 0 {
            ow = true;
            break;
        }
        if arr[0][2] == 1 && arr[1][1] == 1 && arr[2][0] == 1 {
            xw = true;
            break;
        }
        println!("");
        if cntr == 9 {
            dr = true;
            break;
        }
    }
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("Your Last Play-");
    println!("_____________");
    for mut i in 0..3 {
        print!("| ");
        for mut j in 0..3 {
            if arr[i][j] == 1 {
                print!("{} ", 'X');
            } else if arr[i][j] == 0 {
                print!("{} ", 'O');
            } else {
                print!("{} ", ' ');
            }
            print!("| ");
        }

        println!("");
        println!("|___|___|___|");
    }
    if ow {
        println!("\nO wins");
    }
    if xw {
        println!("\nX wins");
    }
    if dr {
        println!("\nDraw!");
    }
}

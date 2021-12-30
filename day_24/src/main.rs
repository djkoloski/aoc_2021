use anyhow::Result;
use problem::{solve_main, Problem};

// I solved this one by hand unfortunately. Here are my notes:

// let x = z % 26;
// z /= a;
// if x + b != i {
//     z *= 26;
//     z += i + c;
// }

//     a   b   c
// 0   1   12  7
// 1   1   13  8
// 2   1   13  10
// 3   26  -2  4
// 4   26  -10 4
// 5   1   13  6
// 6   26  -14 11
// 7   26  -5  13
// 8   1   15  1
// 9   1   15  8
// 10  26  -14 4
// 11  1   10  13
// 12  26  -14 4
// 13  26  -5  14

// b positive: push iN + cN
// b negative: iN = last + bN, pop

// 0:  z = [i0 + 7]
// 1:  z = [i0 + 7, i1 + 8]
// 2:  z = [i0 + 7, i1 + 8, i2 + 10]
// 3:  i3 = i2 + 8
//     z = [i0 + 7, i1 + 8]
// 4:  i4 = i1 - 2
//     z = [i0 + 7]
// 5:  z = [i0 + 7, i5 + 6]
// 6:  i6 = i5 - 8
//     z = [i0 + 7]
// 7:  i7 = i0 + 2
//     z = []
// 8:  z = [i8 + 1]
// 9:  z = [i8 + 1, i9 + 8]
// 10: i10 = i9 - 6
//     z = [i8 + 1]
// 11: z = [i8 + 1, i11 + 13]
// 12: i12 = i11 - 1
//     z = [i8 + 1]
// 13: i13 = i8 - 4
//     z = []

// i0 <= 7
// i1 > 2
// i2 = 1
// i3 = 9
// i4 = i1 - 2
// i5 = 9
// i6 = 1
// i7 = i0 + 2
// i8 > 4
// i9 > 6
// i10 = i9 - 6
// i11 > 1
// i12 = i11 - 1
// i13 = i8 - 4

// 0123456789ABCD
// --------------
// 79197919993985
// 13191913571211

struct Nothing;

impl problem::Input for Nothing {
    fn parse<R: std::io::BufRead>(_: R) -> Result<Self> {
        Ok(Self)
    }
}

struct Day24;

impl Problem for Day24 {
    type Input = Nothing;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(_: &Self::Input) -> Self::PartOne {
        79197919993985
    }

    fn solve_part_two(_: &Self::Input) -> Self::PartTwo {
        13191913571211
    }
}

fn main() {
    solve_main::<Day24>();
}

use crate::boards::CoordIdxConverter;
use crate::system::math::idx_to_coord;

const ASCII_A: u8 = 97;
const ASCII_0: u8 = 48;

//This treats the board as starting with 0 in the top left corner
//It only supports up to a 9x9 board (i.e. 1-9 and a-i)
#[derive(Debug)]
pub struct SingleCharBoardConverter {
    rows: u8,
    cols: u8,
}

//TODO Implement properly to support any simple board
impl SingleCharBoardConverter {
    pub fn new(rows: usize, cols: usize) -> Self {
        // if rows > 9 || cols > 9 {
        //     panic!("SingleCharBoardConverter only supports up to 9x9");
        // }
        SingleCharBoardConverter {
            rows: rows as u8,
            cols: cols as u8,
        }
    }
}

impl CoordIdxConverter for SingleCharBoardConverter {
    fn is_valid_coord(&self, alpha: &str, num: &str) -> bool {
        if alpha.chars().count() > 1 || num.chars().count() > 1 {
            return false;
        }
        let alpha = get_char(alpha, 0) - ASCII_A;
        let num = get_char(num, 0) - ASCII_0;
        alpha < self.cols && num > 0 && num <= self.rows
    }

    fn coord_to_idx(&self, alpha: &str, num: &str) -> usize {
        let idx = (get_char(alpha, 0) - ASCII_A)
            + ((self.rows - (get_char(num, 0) - ASCII_0)) * self.cols);
        idx as usize
    }

    fn idx_to_coord(&self, idx: usize) -> (String, String) {
        let (x, y): (u8, u8) = to_coord(idx, self.cols);
        (
            ((ASCII_A + x) as char).to_string(),
            ((ASCII_0 + (self.rows - y)) as char).to_string(),
        )
    }
}

fn to_coord(idx: usize, cols: u8) -> (u8, u8) {
    let (x, y) = idx_to_coord(idx, cols as usize);
    (x as u8, y as u8)
}

fn get_char(str: &str, idx: usize) -> u8 {
    str.chars().nth(idx).expect("No char").to_ascii_lowercase() as u8
}

#[allow(non_snake_case)]
#[cfg(test)]
mod test {
    use super::*;

    lazy_static! {
        static ref CONVERTER_8: SingleCharBoardConverter = SingleCharBoardConverter::new(8, 8);
        static ref CONVERTER_4: SingleCharBoardConverter = SingleCharBoardConverter::new(4, 4);
        static ref CONVERTER_35: SingleCharBoardConverter = SingleCharBoardConverter::new(3, 5);
    }

    //
    //CONVERTER_8
    //

    #[test]
    fn test_valid_coords_8() {
        let coords = vec![
            ("a", "1"),
            ("a", "2"),
            ("a", "3"),
            ("a", "4"),
            ("a", "5"),
            ("a", "6"),
            ("a", "7"),
            ("a", "8"),
            ("h", "1"),
            ("h", "2"),
            ("h", "3"),
            ("h", "4"),
            ("h", "5"),
            ("h", "6"),
            ("h", "7"),
            ("h", "8"),
            ("b", "2"),
            ("c", "3"),
            ("d", "4"),
            ("e", "5"),
            ("f", "6"),
            ("g", "7"),
            ("A", "1"),
            ("B", "2"),
            ("C", "3"),
            ("D", "4"),
            ("E", "5"),
            ("F", "6"),
            ("G", "7"),
            ("H", "8"),
        ];

        for coord in coords {
            assert!(
                CONVERTER_8.is_valid_coord(coord.0, coord.1),
                format!("{:?}", coord)
            );
        }
    }

    #[test]
    fn test_invalid_coords_8() {
        let coords = vec![
            ("a", "0"),
            ("a", "9"),
            ("h", "0"),
            ("h", "9"),
            ("aa", "5"),
            ("a", "11"),
            ("Ä", "6"),
            ("k", "3"),
        ];

        for coord in coords {
            assert!(
                !CONVERTER_8.is_valid_coord(coord.0, coord.1),
                format!("{:?}", coord)
            );
        }
    }

    #[test]
    fn test_coord_to_idx_8() {
        assert_eq!(CONVERTER_8.coord_to_idx("a", "8"), 0);
        assert_eq!(CONVERTER_8.coord_to_idx("a", "1"), 56);
        assert_eq!(CONVERTER_8.coord_to_idx("h", "8"), 7);
        assert_eq!(CONVERTER_8.coord_to_idx("h", "1"), 63);
        assert_eq!(CONVERTER_8.coord_to_idx("c", "5"), 26);
    }

    #[test]
    fn test_idx_to_coord_8() {
        assert_eq!(
            CONVERTER_8.idx_to_coord(56),
            (String::from("a"), String::from("1"))
        );
        assert_eq!(
            CONVERTER_8.idx_to_coord(0),
            (String::from("a"), String::from("8"))
        );
        assert_eq!(
            CONVERTER_8.idx_to_coord(63),
            (String::from("h"), String::from("1"))
        );
        assert_eq!(
            CONVERTER_8.idx_to_coord(7),
            (String::from("h"), String::from("8"))
        );
        assert_eq!(
            CONVERTER_8.idx_to_coord(26),
            (String::from("c"), String::from("5"))
        );
    }

    //
    //CONVERTER_4
    //

    #[test]
    fn test_valid_coords_4() {
        let coords = vec![
            ("a", "1"),
            ("a", "2"),
            ("a", "3"),
            ("a", "4"),
            ("d", "1"),
            ("d", "2"),
            ("d", "3"),
            ("d", "4"),
            ("b", "2"),
            ("c", "3"),
            ("A", "1"),
            ("B", "2"),
            ("C", "3"),
            ("D", "4"),
        ];

        for coord in coords {
            assert!(
                CONVERTER_4.is_valid_coord(coord.0, coord.1),
                format!("{:?}", coord)
            );
        }
    }

    #[test]
    fn test_invalid_coords_4() {
        let coords = vec![
            ("a", "0"),
            ("a", "9"),
            ("f", "0"),
            ("h", "9"),
            ("aa", "5"),
            ("a", "11"),
            ("Ä", "6"),
            ("k", "3"),
        ];

        for coord in coords {
            assert!(
                !CONVERTER_4.is_valid_coord(coord.0, coord.1),
                format!("{:?}", coord)
            );
        }
    }

    #[test]
    fn test_coord_to_idx_4() {
        assert_eq!(CONVERTER_4.coord_to_idx("a", "4"), 0);
        assert_eq!(CONVERTER_4.coord_to_idx("a", "1"), 12);
        assert_eq!(CONVERTER_4.coord_to_idx("d", "4"), 3);
        assert_eq!(CONVERTER_4.coord_to_idx("d", "1"), 15);
        assert_eq!(CONVERTER_4.coord_to_idx("b", "2"), 9);
    }

    #[test]
    fn test_idx_to_coord_4() {
        assert_eq!(
            CONVERTER_4.idx_to_coord(0),
            (String::from("a"), String::from("4"))
        );
        assert_eq!(
            CONVERTER_4.idx_to_coord(12),
            (String::from("a"), String::from("1"))
        );
        assert_eq!(
            CONVERTER_4.idx_to_coord(3),
            (String::from("d"), String::from("4"))
        );
        assert_eq!(
            CONVERTER_4.idx_to_coord(15),
            (String::from("d"), String::from("1"))
        );
        assert_eq!(
            CONVERTER_4.idx_to_coord(9),
            (String::from("b"), String::from("2"))
        );
    }

    //
    //CONVERTER_35
    //

    #[test]
    fn test_valid_coords_35() {
        let coords = vec![
            ("a", "1"),
            ("a", "2"),
            ("a", "3"),
            ("e", "1"),
            ("e", "2"),
            ("e", "3"),
            ("c", "2"),
        ];

        for coord in coords {
            assert!(
                CONVERTER_35.is_valid_coord(coord.0, coord.1),
                format!("{:?}", coord)
            );
        }
    }

    #[test]
    fn test_invalid_coords_35() {
        let coords = vec![
            ("a", "0"),
            ("a", "9"),
            ("f", "0"),
            ("h", "9"),
            ("aa", "5"),
            ("a", "11"),
            ("Ä", "6"),
            ("k", "3"),
        ];

        for coord in coords {
            assert!(
                !CONVERTER_35.is_valid_coord(coord.0, coord.1),
                format!("{:?}", coord)
            );
        }
    }

    #[test]
    fn test_coord_to_idx_35() {
        assert_eq!(CONVERTER_35.coord_to_idx("a", "1"), 10);
        assert_eq!(CONVERTER_35.coord_to_idx("a", "3"), 0);
        assert_eq!(CONVERTER_35.coord_to_idx("e", "1"), 14);
        assert_eq!(CONVERTER_35.coord_to_idx("e", "3"), 4);
        assert_eq!(CONVERTER_35.coord_to_idx("c", "2"), 7);
    }

    #[test]
    fn test_idx_to_coord_35() {
        assert_eq!(
            CONVERTER_35.idx_to_coord(10),
            (String::from("a"), String::from("1"))
        );
        assert_eq!(
            CONVERTER_35.idx_to_coord(0),
            (String::from("a"), String::from("3"))
        );
        assert_eq!(
            CONVERTER_35.idx_to_coord(14),
            (String::from("e"), String::from("1"))
        );
        assert_eq!(
            CONVERTER_35.idx_to_coord(4),
            (String::from("e"), String::from("3"))
        );
        assert_eq!(
            CONVERTER_35.idx_to_coord(7),
            (String::from("c"), String::from("2"))
        );
    }
}

use crate::constants::games;

#[derive(Eq, PartialEq, Hash, Clone, Ord, PartialOrd)]
pub(super) struct MenuItem {
    pub(super) name: &'static str,
    pub(super) code: &'static str,
    pub(super) desc: &'static str,
}

impl MenuItem {
    fn new(name: &'static str, code: &'static str, desc: &'static str) -> Self {
        MenuItem { name, code, desc }
    }
}

lazy_static! {
    pub(super) static ref ITEMS: Vec<(MenuItem, Option<Vec<MenuItem>>)> = {
        let tictactoe = MenuItem::new(
            "Tic-Tac-Toe",
            games::TICTACTOE,
            r"Tic-Tac-Toe (also known as 'noughts and crosses' and 'Xs and Os') is a simple game dating back thousands of years.
The board is a 3x3 grid and players take turns placing Xs and Os until a line of three Xs or Os is made. If the board is filled
without a line being made the game is a draw.",
        );
        let mancala = MenuItem::new(
            "Mancala",
            games::MANCALA,
            r"Mancala is an ancient game originating from Africa. This is the Kalah variant which was invented in the 1940s in America.
            
Each player has 6 home slots and 1 score/end slot (the larger slot on to the right of the player). The game starts with 6 stones
in each home slot. Players take turns taking all the stones from one of their home slots and dropping them one by one
counterclockwise around the board (skipping the opponents end/score slot). If the last stone dropped lands in a players
score/end slot, they get another turn. If the last stone dropped lands in a players home slot and it was empty, then all stones
from the opposite home and the last one dropped are captured and placed in the players end/score slot.

If a player has no stones left in their home slots then all stones are moved from the home slots to the end/score slot and
the player with the highest number of stones wins.",
        );
        let senet = MenuItem::new("Senet", games::SENET, r"Senet is from Ancient Egypt played on a 3x10 board.

Each player has 5 pieces and the aim to get all of them to the home slot (the last one). At the beginning of each turn the player 
throws 4 sticks: brown on one side and cream on the other; if all 4 are land brown side up then the player can move a piece 5 spaces,
otherwise they can move the number that landed cream side up. If the player gets 1, 4 or 5 then after moving they can move again.

A piece can not jump over two or more consecutive enemy pieces but any number of allied pieces. A square may only have one piece in it, but a piece can
land on an enemy piece to swap places.

If no moves are available then the player can skip their turn.

Pieces must land on the House of Happiness (square 26) before continuing, landing on the House of Water (square 27) will send your
piece back to the House of Rebirth (square 15). Pieces on squares 15 and 26 can not be swapped.
Landing on home (square 30) removes the piece from the board.

Once a player has removed all their pieces they win.");
        let draughts = MenuItem::new(
            "Draughts",
            games::SUBMENU,
            "Draughts (also know as Checkers) variants have existed for thousands of years. It is played on a board with stackable discs.",
        );
        let draughts_english = MenuItem::new(
            "English",
            games::DRAUGHTS_ENGLISH,
            r"English Draughts is played on a 8x8 board, each player has 12 pieces (known as men).
            
Pieces can move one space diagonally forward per turn, if the space is occupied by an opponent and the space after is empty the
piece can jump over and capture the opponent piece, pieces can keep jumping after landing. Players must play the most capturing
move per turn. If a piece reachs the far row it is promoted to king, these can move and capture forwards and backwards one space.

If a player runs out of pieces or if they have no available moves then they lose. If game only has kings left and no captures have
been made for 25 turns then the game is a draw.",
        );
        let draughts_international = MenuItem::new(
            "International",
            games::DRAUGHTS_INTERNATIONAL,
            r"International Draughts is played on a 10x10 board, each player has 20 pieces (known as men).

Pieces can move one space diagonally forward per turn, if the space is occupied by an opponent and the space after is empty the piece
can jump forwards or backwards over and capture the opponent piece, pieces can keep jumping after landing. Players must play the most
capturing move per turn. If a piece reachs the far row it is promoted to king, these can move and capture forwards and backwards any
number of spaces.

If a player runs out of pieces or if they have no available moves then they lose. If game only has kings left and no captures have been
made for 25 turns, or if each player only have one king each then the game is a draw.",
        );
        let draughts_brazilian = MenuItem::new(
            "Brazilian",
            games::DRAUGHTS_BRAZILIAN,
            r"Brazilian Draughts is played on a 8x8 board, each player has 12 pieces (known as men) but otherwise is the same as International.",
        );
        let draughts_canadian = MenuItem::new(
            "Canadian",
            games::DRAUGHTS_CANADIAN,
            r"Canadian Draughts is played on a 12x12 board, each player has 30 pieces (known as men) but otherwise is the same as International.",
        );
        let tablut = MenuItem::new(
            "Tablut",
            games::TABLUT,
            "Tablut is a variant of Hnefatafl from Northern Europe, it's an asymmetric game played on a 9x9 board.

Players are either the Attacker or the Defender. The Defender has 8 pieces and a king piece and the Attacker has 16 pieces.
The Defenders role is to get the king to any of the corners and the Attacker role is to capture the king. Pieces can move
any distance up, down, left or right but can't jump over pieces.

Only the king can enter the corners and no piece can stop at the castle at the centre of the board.

Any two pieces can capture another by surrounding it (above and below, or to left and right), except for the king as it must be 
surrounded by 4 pieces (or 3 pieces and the edge of the board, a corner or the castle). 
The castle acts like an ally when capturing even when occupied.
The king can't capture units.",
        );
        let go = MenuItem::new("Go", games::GO, "Go game");
        let ur = MenuItem::new("Ur", games::UR, "The Royal Game of Ur");
        let blackhole = MenuItem::new("Blackhole", games::BLACKHOLE, "Blackhole");
        let orderchaos = MenuItem::new(
            "Order and Chaos",
            games::ORDERCHAOS,
            "Order and Chaos is a modern asymmetric game played on a 6x6 board using red and white pieces.

Players play as Order or Chaos, both players can place either colour. Order aims to make a sequence (vertically, horizontally or diagonally)
of 5 pieces (the sequence can be red or white) and chaos aims to block this. A sequence of 6 pieces does not count as a win.

If a sequence is created then the Order player wins. If the board fills up before a sequence is made then the Chaos player wins.",
        );
        let rithmomachy = MenuItem::new("Rithmomachy", games::RITHMOMANCHY, "Tactical math");
        let shogi = MenuItem::new(
            "Shogi",
            games::SUBMENU,
            "Known as Japanese Chess but it has several major differences.",
        );
        let shogi_standard =
            MenuItem::new("Standard", games::SHOGI_STANDARD, "Played on a 9x9 board");
        let shogi_mini = MenuItem::new(
            "Mini",
            games::SHOGI_MINI,
            "5x5 variant of the standard game with reduced pieces but otherwise the same rules",
        );
        let chess = MenuItem::new("Chess", games::SUBMENU, "The ancient and popular board game");
        let shogi_medium = MenuItem::new("Medium", games::SHOGI_MEDIUM, "Chu shogi, played on a 12x12 board. It introduces some new piece types over the standard game");
        let shogi_large = MenuItem::new("Large", games::SHOGI_LARGE, "Tai shogi, played on a 25x25 board with 177 pieces. It introduces several new piece types over the standard game");
        let shogi_huge = MenuItem::new("Huge", games::SHOGI_HUGE, "Taikyoku, played on a 36x36 board with 402 pieces. It introduces a lot of new piece types over the standard game");
        let chess_standard = MenuItem::new(
            "Standard",
            games::CHESS_STANDARD,
            "Standard pieces, rules and board",
        );
        let chess_mini = MenuItem::new("Mini", games::CHESS_MINI, "Smaller and quicker variant");
        let chess_grand = MenuItem::new("Grand", games::CHESS_GRAND, "Larger variant");
        let chess_andernach = MenuItem::new(
            "Andernach",
            games::CHESS_ANDERNACH,
            "Standard game except pieces change colour after capturing",
        );
        let chess_checkless = MenuItem::new(
            "Checkless",
            games::CHESS_CHECKLESS,
            "Standard game except check doesn't exist, only checkmate",
        );
        let chess_hostage = MenuItem::new(
            "Hostage",
            games::CHESS_HOSTAGE,
            "Standard game except captured pieces may be used by the capturing player",
        );
        let chess_capablanca = MenuItem::new(
            "Capablanca",
            games::CHESS_CAPABLANCA,
            "Played on 10x8 with two new custom pieces",
        );
        let chess_modern = MenuItem::new(
            "Modern",
            games::CHESS_MODERN,
            "Similar to the standard game but with some extra pieces",
        );
        let chess_progressive = MenuItem::new("Progressive", games::CHESS_PROGRESSIVE, "Standard game except the number of moves increases each turn, so 1st turn - 1 move, 2nd turn - 2 moves, etc.");

        vec![
            (tictactoe, None),
            (mancala, None),
            (senet, None),
            (
                draughts,
                Some(vec![
                    draughts_english,
                    draughts_international,
                    draughts_brazilian,
                    draughts_canadian,
                ]),
            ),
            (tablut, None),
            // (blackhole, None),
            (orderchaos, None),
            // (go, None),
            // (ur, None),
            // (rithmomachy, None),
            // (
            //     shogi,
            //     Some(vec![
            //         shogi_standard,
            //         shogi_mini,
            //         shogi_medium,
            //         shogi_large,
            //         shogi_huge,
            //     ]),
            // ),
            (
                chess,
                Some(vec![
                    chess_standard,
                    chess_mini,
                    chess_grand,
                    chess_modern,
                    chess_andernach,
                    chess_checkless,
                    chess_progressive,
                    chess_hostage,
                    chess_capablanca,
                ]),
            ),
        ]
    };
}

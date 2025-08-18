use crate::color::Color;
use crate::mov::Move;

pub struct History {
    table: [[[u16; 64]; 64]; 2],
}

impl Default for History {
    fn default() -> History {
        History { table: [[[0; 64]; 64]; 2] }
    }
}

impl History {

    #[inline(always)]
    pub fn update_non_captures(&mut self, mov: Move, color: Color, depth: u8) {
        let bonus: u16 = (depth as u16) * (depth as u16);
        let cell = &mut self.table[color as usize][mov.from() as usize][mov.to() as usize];

        // light decay toward 0 each time we touch it
        *cell = ((*cell as u32 * 15) / 16) as u16; //  ~6% decay

        *cell = cell.saturating_add(bonus);
    }


    #[inline(always)]

    pub fn index(&self, mov: Move, color: Color) -> u16 {
        self.table[color as usize][mov.from() as usize][mov.to() as usize]
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.table = [[[0; 64]; 64]; 2];
    }
}
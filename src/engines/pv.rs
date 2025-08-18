use crate::engines::constants::MAX_DEPTH;
use crate::mov::Move;


const PV_ARRAY_LENGTH: usize = (MAX_DEPTH as usize * (MAX_DEPTH as usize + 1)) / 2;
pub struct PV {
    pv_array:  [Move; PV_ARRAY_LENGTH],
}

impl PV {
    #[inline(always)]
    pub fn new() -> PV {
        PV { pv_array: [Move::null(); PV_ARRAY_LENGTH]}
    }

    #[inline(always)]
    pub fn mv(&self, ply: u16) -> Move {
        self.pv_array[self.row_start(ply)]
    }

    #[inline(always)]
    pub fn clear_node(&mut self, ply: u16) {
        self.pv_array[self.row_start(ply)] = Move::null();
    }

    #[inline(always)]
    pub fn adopt(&mut self, ply: u16, mv: Move) {
        let row_start = self.row_start(ply);
        self.pv_array[row_start] = mv;

        // capacity of current row
        let cap = MAX_DEPTH as usize - ply as usize;
        if cap <= 1 {
            // no space after the first slot → also no child row
            return;
        }

        let child_start = row_start + cap;
        let child_cap = cap - 1;

        let mut i = 0;
        while i < child_cap {
            let m = self.pv_array[child_start + i];
            if m.is_null() { break; }
            self.pv_array[row_start + 1 + i] = m;
            i += 1;
        }
    }

    #[inline(always)]
    fn row_start(&self, ply: u16) -> usize {
        (ply * MAX_DEPTH - (ply * (ply).saturating_sub(1) / 2)) as usize
    }

}
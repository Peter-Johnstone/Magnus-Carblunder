use crate::mov::Move;

const CLUSTER_SIZE: usize = 4;


#[derive(Copy, Clone, Default, PartialEq)]

pub enum Bound {
    #[default]
    Exact,
    Lower,
    Upper
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct Entry {
    lock:   u16,   // high 16 bits of zobrist
    pub(crate) depth:  u8,
    pub(crate) bound: Bound,   // <-- was u8 flag
    pub(crate) score:  i16,
    pub(crate) mv:     Move,
    age:    u16,    // optional: generation counter
}

pub struct TransTable {
    mask: usize,
    data: Box<[Entry]>,     // length = clusters * CLUSTER_SIZE
}

impl TransTable {
    pub fn new(mb: usize) -> Self {
        let bytes   = mb * 1024 * 1024;
        let clusters = bytes / (std::mem::size_of::<Entry>() * CLUSTER_SIZE);
        let pow2     = clusters.next_power_of_two();
        let size     = pow2 * CLUSTER_SIZE;

        let data = vec![Entry::default(); size].into_boxed_slice();
        Self { mask: pow2 - 1, data }
    }

    #[inline(always)]
    fn cluster(&self, hash: u64) -> &[Entry] {
        let idx = (hash as usize & self.mask) * CLUSTER_SIZE;
        &self.data[idx..idx + CLUSTER_SIZE]
    }

    // replace _worst_ entry in cluster (simple depth‑pref replacement)
    #[inline(always)]
    fn cluster_mut(&mut self, hash: u64) -> &mut [Entry] {
        let idx = (hash as usize & self.mask) * CLUSTER_SIZE;
        &mut self.data[idx..idx + CLUSTER_SIZE]
    }


    #[inline(always)]
    pub(crate) fn store(&mut self, hash: u64, depth: u8, bound: Bound, score: i16, mv: Move, age: u16) {
        let lock = (hash >> 48) as u16;
        let cluster = self.cluster_mut(hash);

        // Replace same lock if found
        if let Some(e) = cluster.iter_mut().find(|e| e.lock == lock) {
            if depth >= e.depth { *e = Entry { lock, depth, bound, score, mv, age }; }
            return;
        }
        // else choose the entry with smallest (depth, age) tuple
        let idx = cluster.iter()
            .enumerate()
            .min_by_key(|(_, e)| (e.depth, e.age))
            .map(|(i, _)| i).unwrap();

        cluster[idx] = Entry { lock, depth, bound, score, mv, age };

    }


    #[inline(always)]
    pub(crate) fn probe(&self, hash: u64) -> Option<Entry> {
        let lock = (hash >> 48) as u16;
        for e in self.cluster(hash) {
            if e.lock == lock {
                return Some(*e);   // depth check is removed
            }
        }
        None
    }


}

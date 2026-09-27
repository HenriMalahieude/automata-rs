const prime_list: Vec<u64> = vec![ //first 64 primes + a bit more, using this for ids
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
    73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173,
    179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 233, 239, 241, 251, 257, 257, 263, 269, 271, 277, 281,
    283, 293, 307, 311, 313, 317, 331, 337, 347
];

pub trait Cells {
    type Cell: Default; //make the cell type accessible to me
    fn shape(&self) -> [usize];
    fn locate(&self, ind: &[usize]) -> &Self::Cell;
    fn modify(&mut self, ind: &[usize], v: Self::Cell);
    fn reset(&mut self);
}

//1 Dimensional
impl<CellT: Default> Cells for Vec<CellT> {
    type Cell = CellT;

    fn shape(&self) -> [usize] { return [self.len()]; }

    fn locate(&self, ind: &[usize]) -> &Self::Cell {
        assert!(ind.len() == 1);
        assert!(ind[0] < self.len());
        return &self[ind[0]];
    }

    fn modify(&mut self, ind: &[usize], v: Self::Cell) {
        assert!(ind.len() == 1);
        assert!(ind[0] < self.len());
        self[ind[0]] = v;
    }

    fn reset(&mut self) {
        for i in 0..self.shape()[0] {
            self[i] = Self::Cell::default();
        }
    }
}


/*
//2 Dimensional
impl<CellT> Cells for Vec<Vec<CellT>> {
    type Cell = CellT;

    fn shape(&self) -> &[usize] { return &[self.len(), self[0].len()]; }

    fn locate(&self, ind: &[usize]) -> &Self::Cell {
        assert!(ind[0] < self.len());
        assert!(ind[1] < self[0].len());
        return &self[ind[0]][ind[1]];
    }

    fn modify(&mut self, ind: &[usize], v: Self::Cell) {
        assert!(ind.0 < self.len());
        assert!(ind.1 < self[0].len());
        self[ind[0]][ind[1]] = v;
    }
}

impl<CellT> Cells for Vec<Vec<Vec<CellT>>> {
    type Cell = CellT;
    fn locate(&self, ind: &[usize]) -> &Self::Cell {
        assert!(ind.0 < self.len());
        assert!(ind.1 < self.len());
    }
} */

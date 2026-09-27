use std::ops::{Deref, DerefMut};

const PRIMES: [u64; 69] = [ //first 64 primes + a bit more, using this for ids
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
    73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173,
    179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
    283, 293, 307, 311, 313, 317, 331, 337, 347
];

pub trait Cells {
    type Cell: Default; //make the cell type accessible to me
    fn shape(&self) -> Vec<usize>;
    fn locate(&self, ind: &Vec<usize>) -> &Self::Cell;
    fn modify(&mut self, ind: &Vec<usize>, v: Self::Cell);
    fn reset(&mut self);
}

//1 Dimensional
pub struct Grid1d<CellT>(pub Vec<CellT>);
//TODO: Fix with Deref and DerefMut attr impls

impl<CellT: Default> Cells for Grid1d<CellT> {
    type Cell = CellT;

    fn shape(&self) -> Vec<usize> { return vec![self.len()]; }

    fn locate(&self, ind: &Vec<usize>) -> &Self::Cell {
        assert!(ind.len() == 1);
        assert!(ind[0] < self.len());
        return &self[ind[0]];
    }

    fn modify(&mut self, ind: &Vec<usize>, v: Self::Cell) {
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

//2 Dimensional
pub struct Grid2d<CellT>(pub Vec<Vec<CellT>>);
impl<CellT: Default> Cells for Grid2d<CellT> {
    type Cell = CellT;

    fn shape(&self) -> Vec<usize> { return vec![self.len(), self[0].len()]; }

    fn locate(&self, ind: &Vec<usize>) -> &Self::Cell {
        assert!(ind.len() == 2);
        assert!(ind[0] < self.len());
        assert!(ind[1] < self[0].len());
        return &self[ind[0]][ind[1]];
    }

    fn modify(&mut self, ind: &Vec<usize>, v: Self::Cell) {
        assert!(ind.len() == 2);
        assert!(ind[0] < self.len());
        assert!(ind[1] < self[0].len());
        self[ind[0]][ind[1]] = v;
    }

    fn reset(&mut self) {
        let shpe = self.shape();
        for i in 0..shpe[0] {
            for j in 0..shpe[1] {
                self[i][j] = Self::Cell::default();
            }
        }
    }
}

//3 Dimensional
pub struct Grid3d<CellT>(pub Vec<Vec<Vec<CellT>>>);
impl<CellT: Default> Cells for Grid3d<CellT> {
    type Cell = CellT;

    fn shape(&self) -> Vec<usize> { return vec![self.len(), self[0].len(), self[0][0].len()]; }

    fn locate(&self, ind: &Vec<usize>) -> &Self::Cell {
        assert!(ind.len() == 3);
        assert!(ind[0] < self.len());
        assert!(ind[1] < self[0].len());
        assert!(ind[2] < self[0][0].len());
        return &self[ind[0]][ind[1]][ind[2]];
    }

    fn modify(&mut self, ind: &Vec<usize>, v: Self::Cell) {
        assert!(ind.len() == 3);
        assert!(ind[0] < self.len());
        assert!(ind[1] < self[0].len());
        assert!(ind[2] < self[0][0].len());
        self[ind[0]][ind[1]][ind[2]] = v;
    }

    fn reset(&mut self) {
        let shpe = self.shape();
        for i in 0..shpe[0] {
            for j in 0..shpe[1] {
                for k in 0..shpe[2] {
                    self[i][j][k] = Self::Cell::default();
                }
            }
        }
    }
}

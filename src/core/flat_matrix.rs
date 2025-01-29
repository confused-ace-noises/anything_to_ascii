use std::ops::{Index, IndexMut};
use rayon::{iter::{FromParallelIterator, IntoParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator}, slice::{ParallelSlice, ParallelSliceMut}};

#[derive(Debug, Clone)]
pub struct FlatMatrix<T> {
    pub(crate) vec: Vec<T>,
    pub(crate) rows: usize,
    pub(crate) columns: usize,
}

impl<T: Clone + Send + Sync> FlatMatrix<T> {
    
    
    pub fn par_iter(&self) -> rayon::slice::Iter<T> {
        self.vec.par_iter()
    }

    pub fn par_iter_mut(&mut self) -> rayon::slice::IterMut<T> {
        self.vec.par_iter_mut()
    }
    
    pub fn par_chunks(&self) -> rayon::slice::Chunks<T> {
        self.vec.par_chunks(self.columns)
    }
    
    pub fn par_chunks_mut(&mut self) -> rayon::slice::ChunksMut<T> {
        self.vec.par_chunks_mut(self.columns)
    }
    
}

impl<T: Clone> FlatMatrix<T> {
    pub fn transpose(&self) -> FlatMatrix<T> {
        let mut transposed_vec = vec![self.vec[0].clone(); self.vec.len()];
        
        for row in 0..self.rows {
            for col in 0..self.columns {
                transposed_vec[col * self.rows + row] = self.vec[row * self.columns + col].clone();
            }
        }
        
        FlatMatrix {
            vec: transposed_vec,
            rows: self.columns,
            columns: self.rows,
        }
    }

    pub fn new_fill(rows: usize, columns: usize, fill_val: T) -> Self {
        FlatMatrix { vec: vec![fill_val; rows*columns], rows, columns }
    }
    
    pub fn new_empty(rows: usize, columns: usize) -> Self {
        FlatMatrix { vec: Vec::with_capacity(rows*columns), rows, columns }
    }

    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        if row > self.rows || column > self.columns {
            None
        } else {
            Some(&self[(row, column)])
        }
    }
    
    pub fn get_mut(&mut self, row: usize, column: usize) -> Option<&T> {
        if row > self.rows || column > self.columns {
            None
        } else {
            Some(&mut self[(row, column)])
        }
    }
    
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.vec.iter()
    }
    
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.vec.iter_mut()
    }
    
    pub fn chunks(&self) -> std::slice::Chunks<T> {
        self.vec.chunks(self.columns)
    }
    
    pub fn chunks_mut(&mut self) -> std::slice::ChunksMut<T> {
        self.vec.chunks_mut(self.columns)
    }
    pub fn get_row(&self, row_index: usize) -> Vec<T> {
        let mut vec = Vec::with_capacity(self.columns);
        
        for i in 0..self.columns {
            vec.push(self[(row_index, i)].clone());
        }
    
        vec
    }

    pub fn into_iter_vecs(&self) -> std::vec::IntoIter<Vec<T>> {
        let mut vec = Vec::with_capacity(self.rows);

        for i in 0..self.rows {
            vec.push(self.get_row(i));
        }

        vec.into_iter()
    }

}

impl<T> Index<(usize, usize)> for FlatMatrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.vec[index.0 * self.columns + index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for FlatMatrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.vec[index.0 * self.columns + index.1]
    }
}

impl<T: Send> FromParallelIterator<Vec<T>> for FlatMatrix<T> {
    fn from_par_iter<I>(par_iter: I) -> Self
    where
        I: IntoParallelIterator<Item = Vec<T>>,
    {
        let (rows, cols, data) = par_iter
            .into_par_iter()
            .map(|row| {
                let len = row.len();
                (len, row) // Capture the row length
            })
            .collect::<Vec<_>>() // Collect into a Vec<(usize, Vec<T>)>
            // .into_par_iter()
            // .fold(
            //     || (0, None, Vec::new()),
            //     |(mut row_count, col_count, mut acc), (len, row)| {
            //         row_count += 1;
            //         if let Some(cols) = col_count {
            //             assert_eq!(cols, len, "Inconsistent row lengths");
            //         }
            //         acc.extend(row); // Extend avoids extra allocations
            //         (row_count, Some(len), acc)
            //     },
            // ).reduce(
            //     || (0, None, Vec::new()), // Identity value for reduction
            //     |(rows1, cols1, mut data1), (rows2, cols2, data2)| {
            //         assert_eq!(cols1, cols2, "Inconsistent row lengths across threads");
            //         data1.extend(data2);
            //         (rows1 + rows2, cols1.or(cols2), data1)
            //     },
            // );
            .into_iter()
            .fold(
                (0, None, Vec::new()),
                |(mut row_count, col_count, mut acc), (len, row)| {
                    row_count += 1;
                    if let Some(cols) = col_count {
                        assert_eq!(cols, len, "Inconsistent row lengths");
                    }
                    acc.extend(row); // Extend avoids extra allocations
                    (row_count, Some(len), acc)
                },
            );

        // FlatMatrix::new(rows, cols.unwrap_or(0), data)

        FlatMatrix { vec: data, rows, columns: cols.unwrap_or(0) }
    }
}

impl<U> FromIterator<Vec<U>> for FlatMatrix<U> {
    fn from_iter<T: IntoIterator<Item = Vec<U>>>(iter: T) -> Self {
        let (rows, cols, data) = iter
        .into_iter()
        .map(|row| {
            let len = row.len();
            (len, row) // Capture the row length
        })
        .collect::<Vec<_>>() // Collect into a Vec<(usize, Vec<U>)>
        .into_iter() // Sequential iteration
        .fold(
            (0, None, Vec::new()),
            |(mut row_count, col_count, mut acc), (len, row)| {
                row_count += 1;
                if let Some(cols) = col_count {
                    assert_eq!(cols, len, "Inconsistent row lengths");
                }
                acc.extend(row); // Extend avoids extra allocations
                (row_count, Some(len), acc)
            },
        );

    Self {
        vec: data,
        rows,
        columns: cols.unwrap_or(0),
    }        
    }
}

impl<T> From<Vec<Vec<T>>> for FlatMatrix<T> {
    fn from(value: Vec<Vec<T>>) -> Self {
        value.into_iter().collect()
    }
}
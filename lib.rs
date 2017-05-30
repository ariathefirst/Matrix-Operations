use std::{ops, fmt};
#[derive(PartialEq, Debug)]
pub struct Matrix<T> {
    /// Stores elements in [row-major order](https://en.wikipedia.org/wiki/Row-major_order)
    data: Vec<T>,
    /// Number of rows
    row: usize,
    /// Number of columns
    col: usize,
}
/*pub struct Vec<T> {
    // some fields omitted
}
*/
use std::vec::Vec;
impl<T: Copy> Matrix<T> {
    /// Creates a new matrix of `row` rows and `col` columns, and initializes
    /// the matrix with the elements in `values` in row-major order.
    pub fn new(row: usize, col: usize, values: &[T]) -> Matrix<T> {
	
        Matrix {
        row: row,
		col: col,
		data: values.to_vec(),
	    }
    }

    /// Creates a new, empty matrix of `row` rows and `col` columns.
    /// `data` contains no element.
    pub fn new_empty(row: usize, col: usize) -> Matrix<T> {
        let a : Vec<T> = Vec::new();
		let x = Matrix::new(0, 0, &a);
        Matrix {
			row: 0,
			col: 0,
			data: a,
		}
    }
    /// Returns a shared reference to `data`
    pub fn data(&self) -> &Vec<T> {
		&self.data
    }
    /// Returns a mutable reference to `data`
    pub fn mut_data(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    /// Returns the number of rows and columns in the first and second
    /// elements of the tuple, respectively.
    pub fn size(&self) -> (usize, usize) {
        let (x, y) = (self.row, self.col);
        (x,y)
    }
}

impl<T: ops::Add<Output = T> + Copy> ops::Add for Matrix<T> {
    type Output = Self;
    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row`, panic.
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Vec::new();
	    if self.row != rhs.row {
	        panic!();
	    } else {
	        for i in 0..self.row {
		        for j in 0..self.col {
		//let mut temp = 0;
            	//for j in 0..self.col {
	        //    result[i][j] = self.data[i][j] + rhs.data[i][j];
	        	    result.push(self.data[i*self.col+j] + rhs.data[i*self.col+j]);
    		    }
	        }
	    }
	    Matrix {
            row: self.row,
            col: self.col,
            data: result
        }		
    }
}

impl<T: ops::Sub<Output = T> + Copy> ops::Sub for Matrix<T> {
    type Output = Self;
    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row`, panic.
    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Vec::new();
        //let result: Vec<i32> = Vec::new();
        if self.row != rhs.row {
            panic!();
        } else {
	        for i in 0..self.row {
		/*let mut temp = 0;
            	for j in 0..self.col {
			result[i][j] = self.data[i][j] - rhs.data[i][j];
	        }
		*/
		        for j in 0..self.col {
			        result.push(self.data[i*self.col+j] - rhs.data[i*self.col+j]);
		        }
    		/*for j in 0..self.col {
			result.push(self.data[i*self.col+j] + self.data[i*self.col+j]);
		}*/
	        }
	   }
	   Matrix {
         row: self.row,
		 col: self.col,
		 data: result,
	   }
    }
}

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy + Default> ops::Mul for Matrix<T> {
    type Output = Self;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Vec::new();
        //let k = rhs.col;
        let mut a:T = Default::default();
        //let mut a:T;
        //let index = 0;
        //a = self.data[0] * rhs.data[0];
	    if self.col != rhs.row {
	        panic!();
	    } else {
	       //  for (int i = 0; i < aRows; i++) { // aRow
        //      for (int j = 0; j < bColumns; j++) { // bColumn
        //         for (int k = 0; k < aColumns; k++) { // aColumn
        //             C[i][j] += A[i][k] * B[k][j];
        //         }
        //     }
        // }
	        for i in 0..self.row {
       	        for j in 0..rhs.col {
       	            for k in 0..self.col  {
       	                //a = a + self.data[i*self.col+j] * rhs.data[j*self.col+k];
       	                //result.push(a)
       	            //a = a + self.data[i*self.col+j] * rhs.data[j*self.col+k];
       	            a = a + self.data[i*self.col+k] * rhs.data[k*self.row +j];
       	            //result.push(a)
       	            } 
       	            //result.push(a);
       	            result.push(a);
       	            a = Default::default();

       	        }
       	        //result.push(a);
 	       	}
 	       	//result.push(a)
		// let x: T = Default::default;
    	}
	    Matrix {
		    row: self.row,
		    col: rhs.col,
            data: result,
        }
    }
}
impl<T: fmt::Display> fmt::Display for Matrix<T> {
    /// Formats the matrix as follows:
    /// * Writes each row on a separate line. No empty lines before or after any row.
    /// * On each row, writes each element followed by a single space, except no space following the last element of the row.
    /// Outputs using `write!(f, ...)`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	    let mut s = String::new();
	//for item in &self.data {
	//	s = format!("{} {}", s, item);
	    let mut ctr_col = 0;
    //while ctr_col < self.col {
	    for item in &self.data {
		    s = format!("{}{} ", s, item);
		    ctr_col += 1;
		    if ctr_col == self.col {
		        s.pop();
		        s.push('\n');
			    ctr_col = 0;
		    }
	    }
	    write!(f, "{}", s)
    }
}  


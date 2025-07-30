
/*
    Transform any scalar value to something between 0 and 1
    Normally the input here will be scalar value that is the
    result of the dot product with all input values and
    weigth matrix synapses to one node(neurons)
    
    Input: Any number
    Return: Change input vector where each entry is between 0 and 1
*/  
fn sigmoid(vector: &mut [f64]) {

    for (_, value) in vector.iter_mut().enumerate() {
        *value = 1.0/(1.0 + (*value * -1.0).exp())
    }
}

/*
    Calculate scalar product between two vectors
*/
fn scalar_dot_product(input_vector: &[f64], weigth_vector: &[f64]) -> f64 {
    // Exit with panic if vector does not have same length
    // Assume this is caused by programming error
    assert_eq!(input_vector.len(), weigth_vector.len(), "Vectors must have the same length for scalar dot product.");

    let mut sum = 0.0;

    for (a, b) in input_vector.iter().zip(weigth_vector.iter()) {
        sum += *a * *b
    }
    sum
}

/*
    Back propagate the difference between input and output 
    back to the weights
    
    Return:
            Modified weigth matrixes w1 and w2
*/
pub fn back_prop(input: &[i8], _output: &[i8], w1: &[&[f64]], w2: &[&[f64]], temp: &mut Vec<f64>) {

    // Tranform the input vector from i8 to f64
    let input_f64: Vec<f64> = input.iter().map(|&number| number as f64).collect();

    // Scalar dot product of input vector and weigth matrix to create hidden node layer
    let columns = w1[0].len();
    let mut z1: Vec<f64> = Vec::new();
    for col_index in 0..columns {
        // This is a workaround necessary because the python original code 
        // arranged the weigth matrix in one row for containing weigths for all output nodes
        // instead of all weights for one input node
        let synapse_column: Vec<f64>  = w1.iter().map(|row|row[col_index]).collect();
        z1.push(scalar_dot_product(&input_f64, &synapse_column));
    }
    sigmoid(&mut z1);

    // Scalar dot product of hidden node layer and output weigth matrix to create estimated 
    // output vector
    let columns = w2[0].len();  // Number of columns, i.e. output nodes
    for col_index in 0..columns {
        let synapse_column: Vec<f64>  = w2.iter().map(|row|row[col_index]).collect();
        temp.push(scalar_dot_product(&z1, &synapse_column));
    }
    sigmoid(temp);
    
    // Subtract estimated output vector with wanted output vector 
    
    dbg!(temp);
}




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
fn scalar_dot_product(input_vector: &[i8], weigth_vector: &[f64]) -> f64 {
    // Exit with panic if vector does not have same length
    // Assume this is caused by programming error
    assert_eq!(input_vector.len(), weigth_vector.len(), "Vectors must have the same length for scalar dot product.");

    let mut sum = 0.0;

    for (a, b) in input_vector.iter().zip(weigth_vector.iter()) {
        sum += (*a as f64) * *b
    }
    sum
}

/*
    Back propagate the difference between input and output 
    back to the weights
    
    Return:
            Modified weigth matrixes w1 and w2
*/
pub fn back_prop(input: &[i8], _output: &[i8], w1: &[&[f64]], z1: &mut Vec<f64>) {

    let columns = w1[0].len();

    for row_index in 0..columns {
        dbg!(row_index);
        let synapse_column: Vec<f64>  = w1.iter().map(|row|row[row_index]).collect();
        println!("len input {}", input.len());
        println!("len synapse_column {}", synapse_column.len());

        z1.push(scalar_dot_product(input, &synapse_column));
    }

    sigmoid(z1);
    dbg!(z1);
}



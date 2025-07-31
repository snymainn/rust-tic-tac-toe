
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
#[cfg_attr(not(test), allow(dead_code))]
pub fn back_prop(input: &[i8], output: &[i8], w1: &mut [&mut [f64]], w2: &mut [&mut [f64]], alpha: f64) {

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
    let mut z2: Vec<f64> = Vec::new();
    for col_index in 0..columns {
        let synapse_column: Vec<f64>  = w2.iter().map(|row|row[col_index]).collect();
        z2.push(scalar_dot_product(&z1, &synapse_column));
    }
    sigmoid(&mut z2);
    
    // Subtract estimated output vector with wanted output vector
    let mut d2: Vec<f64> = Vec::new();
    for (out, a2) in output.iter().zip(z2.iter()) {
        d2.push(a2 - (*out as f64));
    }
    
    // Scalar dot product of output diff d2 and each of the rows in the weight matrix
    // Each row represents each of the output nodes
    // The result is a modified hidden node layer
    let mut temp_back_prop_hidden_layer: Vec<f64> = Vec::new();
    for row in w2.iter() {
        temp_back_prop_hidden_layer.push(scalar_dot_product(row, &d2));
    }

    // Create a pass filtered version of the original hidden layer where small and large
    // values are dampened
    let pass_filtered_hidden_layer: Vec<f64> = z1.iter().map(|value| value * (1.0-value)).collect();
    
    // Create a new diff hidden layer by multiplying each node in the back_prop layer with the filtered
    let mut d1: Vec<f64> = Vec::new();
    for (tmp_bp, filtered) in temp_back_prop_hidden_layer.iter().zip(pass_filtered_hidden_layer.iter()) {
        d1.push(*tmp_bp * filtered);
    }

    // Take the new diff hidden layer and create a matrix by multiplying the
    // diff layer with the original input data and thus creating and new diff weight matrix
    let mut w1_adj: Vec<Vec<f64>> = Vec::new();
    for input_node_item in input_f64.iter() {
        let temp_row: Vec<f64> = d1.iter().map(|value| value*input_node_item).collect();
        w1_adj.push(temp_row);
    }

    // Do the same to the original hidden layer and the diff output nodes d2
    let mut w2_adj: Vec<Vec<f64>> = Vec::new();
    for hidden_node_item in z1.iter() {
        let temp_row: Vec<f64> = d2.iter().map(|value| value*hidden_node_item).collect();
        w2_adj.push(temp_row);
    }

    for (row_index, w2_row_ref) in w2.iter_mut().enumerate() {
        for (col_index, element) in w2_row_ref.iter_mut().enumerate() {
            *element -= alpha * w2_adj[row_index][col_index];
        }
    }

    for (row_index, w1_row_ref) in w1.iter_mut().enumerate() {
        for (col_index, element) in w1_row_ref.iter_mut().enumerate() {
            *element -= alpha * w1_adj[row_index][col_index];
        }
    }

    //dbg!(w1_adj);
}




/*
    Transform any scalar value to something between 0 and 1
    Normally the input here will be scalar value that is the
    result of the dot product with all input values and
    weigth matrix synapses to one node(neurons)
    
    Input: Any number
    Return: Number between 0 and 1
*/  
fn sigmoid(vector: f32) -> f32 {
    return 1.0/(1.0 + (vector * -1.0).exp());
}

/*
    Calculate scalar product between two vectors
*/
fn scalar_dot_product(v1: &[f32], v2: &[f32]) -> f32 {
    // Exit with panic if vector does not have same length
    // Assume this is caused by programming error
    assert_eq!(v1.len(), v2.len(), "Vectors must have the same length for scalar dot product.");

    let mut sum = 0.0;

    for (a, b) in v1.iter().zip(v2.iter()) {
        sum += a * b
    }
    sum
}

/*
    Back propagate the difference between input and output
    through a network back to the weights
    Return:
            Modified weigth matrixes w1 and w2
*/
//#[allow(dead_code)]
pub fn back_prop(_input: Vec<i8>, _output: Vec<i8>) -> f32 {

    let mut z1: Vec<f32>;

    for synapse_column in w1.iter() {
        z1.push(scalar_dot_product(synapse_column, input));
    }

    let a1 = sigmoid(z1);
    
    return a1;
}



mod data;

use rand::Rng;
use rand::rngs::ThreadRng;
use crate::data::Data;

/* Single-layer perceptron
 * Consists of two neurons, one input and one output
 *
 * We will train the network to predict the sum of two numbers
 *
 * Neuron: receives some input, performs some operation, and returns some output
 * Each neuron is connected to one output, forming a network
 *
 * Activation function: decides whether to keep the neuron active or not
 * Sort of like a gatekeeper which decides how brightly the neuron should light up
 * Adds non-linearity to the network
 *
 * Output = ActivationFunction(Weights * Inputs + Bias)
 */
struct NN {
    /*
     * Weights: how much each input is contributing to the output
     * Sort of like thickness of the tube, influencing the signal that is passed
    */
    weights: Vec<f64>,

    /*
     * Bias: how much to add to the output
     * Sort of like a knob alongside the neuron, adjusts how active the neuron is
     */
    bias: f64,

    /*
     * Learning rate: how much to adjust the weights
     * How fast the network learns
     */
    learning_rate: f64,
}

// This neural network has two neurons
impl NN {
    fn new() -> Self {
        // Generate random weights and bias
        let mut rng: ThreadRng = rand::thread_rng();

        Self {
            // Two weights, one for each neuron, from 0 to 1
            weights: vec![rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)],

            // Random bias, from 0 to 1
            bias: rng.gen_range(0.0..1.0),

            // Learning rate, fixed at 0.1
            learning_rate: 0.1,
        }
    }

    // Used to predict the output
    // Multiplies the input by the weights, add the bias and apply the activation function
    fn predict(&self, input: &[f64; 2]) -> f64 {
        // Initialize sum with bias
        let mut sum: f64 = self.bias;

        // Multiply input by weights, add to sum, which is the bias
        for (i, weight) in self.weights.iter().enumerate() {
            sum += input[i] * weight;
        }

        // Apply activation function over sum, which normalizes the output
        sigmoid(sum)
    }

    // Backpropagation: finds the gradient of the loss function
    // Meaning that how much to adjust the weights in order to reduce the error
    // If the final output isn't same as the desired output, adjust the weights and bias
    // Sort of finding the right paths to adjust the weights, in order to reduce the error
    // We repeat this process for each input, and each epoch
    fn train(&mut self, inputs: Vec<[f64; 2]>, outputs: Vec<f64>, epochs: usize) {
        // For each epoch
        for _ in 0..epochs {
            // For each input
            for (i, input) in inputs.iter().enumerate() {
                // Get a prediction for a given input
                let output: f64 = self.predict(input);

                // Compute the difference between the actual and the desired output
                let error: f64 = outputs[i] - output;

                // Find the gradient of the loss function
                // (sort of like a hint about the direction to adjust the weights in)
                let delta: f64 = derivative(output);

                // Adjust the weights and the bias to reduce error in the output
                for j in 0..self.weights.len() {
                    self.weights[j] += self.learning_rate * error * input[j] * delta;
                }

                // Adjust the bias, to reduce error in the output
                self.bias += self.learning_rate * error * delta;
            }
        }

    }
}

// Non-linear activation function: transforms the output to be between 0 and 1
#[inline]
fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

// Derivative of the sigmoid function
// Used to find the gradient of the loss function, in order to reduce the error
#[inline]
fn derivative(x: f64) -> f64 {
    x * (1.0 - x)
}

fn main() {
    // Get data from json
    // The data consists on two numbers and the sum of the two
    let d: Data = data::get_data().unwrap();

    // Inputs, used to train the network
    let inputs: Vec<[f64; 2]> = d.training_inputs;

    // Outputs, which are the sum of the two numbers
    let outputs: Vec<f64> = d.training_outputs;

    // Test inputs, used to test the network
    let test_inputs: Vec<[f64; 2]> = d.test_inputs;

    // Initialize the network
    let mut neural_net: NN = NN::new();

    // Train for 10000 epochs
    neural_net.train(inputs, outputs, 10000);

    // For each test input, predict the output
    for input in test_inputs.iter() {
        // Pass a set of inputs (two numbers) and get a prediction back which should be a sum of the two numbers
        let prediction = neural_net.predict(input);
        println!("Input: {:?}, Prediction: {:.1}", input, prediction);
    }
}

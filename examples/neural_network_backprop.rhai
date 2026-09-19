// Train a tiny 2-2-1 neural network on XOR with explicit backpropagation.

fn sigmoid(x) {
    1.0 / (1.0 + exp(0.0 - x))
}

fn sigmoid_matrix(A) {
    let out = [];

    for row in A {
        let out_row = [];
        for value in row {
            out_row.push(sigmoid(value));
        }
        out.push(out_row);
    }

    out
}

fn sigmoid_prime_from_activation(A) {
    let out = [];

    for row in A {
        let out_row = [];
        for value in row {
            out_row.push(value * (1.0 - value));
        }
        out.push(out_row);
    }

    out
}

fn sub_matrix(A, B) {
    let out = [];

    for i in 0..A.len() {
        let a_row = A[i];
        let b_row = B[i];
        let row = [];
        for j in 0..a_row.len() {
            row.push(a_row[j] - b_row[j]);
        }
        out.push(row);
    }

    out
}

fn scale_matrix(A, scale) {
    let out = [];

    for row in A {
        let out_row = [];
        for value in row {
            out_row.push(value * scale);
        }
        out.push(out_row);
    }

    out
}

fn hadamard(A, B) {
    let out = [];

    for i in 0..A.len() {
        let a_row = A[i];
        let b_row = B[i];
        let row = [];
        for j in 0..a_row.len() {
            row.push(a_row[j] * b_row[j]);
        }
        out.push(row);
    }

    out
}

fn sum_squares(A) {
    let total = 0.0;

    for row in A {
        for value in row {
            total += value * value;
        }
    }

    total
}

fn forward(W1, W2, x) {
    let x_with_bias = vertcat(x, col([1.0]));
    let hidden = sigmoid_matrix(mtimes(W1, x_with_bias));
    let hidden_with_bias = vertcat(hidden, col([1.0]));
    let output = sigmoid_matrix(mtimes(W2, hidden_with_bias));

    #{
        "x_with_bias": x_with_bias,
        "hidden": hidden,
        "hidden_with_bias": hidden_with_bias,
        "output": output
    }
}

fn total_loss(W1, W2, inputs, targets) {
    let loss = 0.0;

    for i in 0..inputs.len() {
        let x = inputs[i];
        let target_value = targets[i];
        let step = forward(W1, W2, x);
        let prediction = step.output;
        let target = col([target_value]);
        let residual = sub_matrix(prediction, target);
        loss += 0.5 * sum_squares(residual);
    }

    loss
}

fn predictions(W1, W2, inputs) {
    let out = [];

    for x in inputs {
        let prediction = forward(W1, W2, x).output;
        let row = prediction[0];
        out.push(row[0]);
    }

    out
}

let inputs = [
    col([0.0, 0.0]),
    col([0.0, 1.0]),
    col([1.0, 0.0]),
    col([1.0, 1.0])
];
let targets = [0.0, 1.0, 1.0, 0.0];

// Fixed starting weights keep the example deterministic while still learning.
let W1 = mat([[0.6888, 0.5159, -0.1589], [-0.4822, 0.0225, -0.1901]]);
let W2 = row([0.5676, -0.3934, -0.0468]);
let learning_rate = 1.5;
let epochs = 800;
let initial_loss = total_loss(W1, W2, inputs, targets);

for epoch in 0..epochs {
    for i in 0..inputs.len() {
        let x = inputs[i];
        let target_value = targets[i];
        let step = forward(W1, W2, x);
        let target = col([target_value]);

        let output_error = sub_matrix(step.output, target);
        let output_slope = sigmoid_prime_from_activation(step.output);
        let output_delta = hadamard(output_error, output_slope);
        let output_weight_row = W2[0];
        let output_weights_without_bias = transpose(row([output_weight_row[0], output_weight_row[1]]));

        let hidden_error = mtimes(output_weights_without_bias, output_delta);
        let hidden_slope = sigmoid_prime_from_activation(step.hidden);
        let hidden_delta = hadamard(hidden_error, hidden_slope);

        let grad_W2 = mtimes(output_delta, transpose(step.hidden_with_bias));
        let grad_W1 = mtimes(hidden_delta, transpose(step.x_with_bias));

        W2 = sub_matrix(W2, scale_matrix(grad_W2, learning_rate));
        W1 = sub_matrix(W1, scale_matrix(grad_W1, learning_rate));
    }
}

let final_loss = total_loss(W1, W2, inputs, targets);
let final_predictions = predictions(W1, W2, inputs);

let result = #{
    "initial_loss": initial_loss,
    "final_loss": final_loss,
    "predictions": final_predictions,
    "W1": W1,
    "W2": W2
};

print(result);
result

// Load data
let url = "https://raw.githubusercontent.com/plotly/datasets/master/diabetes.csv";
let x = read_matrix(url).transpose();

// Massage data
let L = x.len;
let y = x.drain(|v, i| i == (L-1));
let x = ones(1, size(x)[1]) + x;

// Do regression and report
let b = regress(x.transpose(), y.transpose());
print(b);
// Load data
let url = "https://raw.githubusercontent.com/plotly/datasets/master/diabetes.csv";
let x = read_matrix(url).transpose();

// Massage data
let L = x.len;
let y = x.drain(|v, i| i == (L-1));
// regress fits the intercept automatically; no column of ones is needed.

// Do regression and report
let b = regress(x.transpose(), y.transpose());
print(b);

// Simulate simple projectile motion using rhai-sci functions
let g = 9.81;           // gravitational acceleration (m/s^2)
let v0 = 25.0;          // launch speed (m/s)
let angle = 45.0;       // launch angle (degrees)

// Generate a time vector from 0 to total flight time
let t_flight = 2.0 * v0 * sind(angle) / g;
let times = linspace(0.0, t_flight, 50);

let vx = v0 * cosd(angle);
let vy0 = v0 * sind(angle);

let x = [];
let y = [];

for t in times {
    x.push(vx * t);
    y.push(vy0 * t - 0.5 * g * t * t);
}

let max_y = max(y);
let idx = argmax(y);
let peak_time = times[idx];
let range = max(x);

let result = #{
    "max_height": max_y,
    "time_of_flight": t_flight,
    "peak_time": peak_time,
    "range": range
};
print(result);
result

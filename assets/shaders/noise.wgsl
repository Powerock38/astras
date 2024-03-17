#import bevy_sprite::mesh2d_view_bindings::globals

fn random(x: f32) -> f32 {
    return fract(sin(x) * 10000.0);
}

fn noise(p: vec2<f32>) -> f32 {
    return random(p.x + p.y * 10000.0);
}

fn sw(p: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(floor(p.x), floor(p.y));
}

fn se(p: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(ceil(p.x), floor(p.y));
}

fn nw(p: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(floor(p.x), ceil(p.y));
}

fn ne(p: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(ceil(p.x), ceil(p.y));
}

fn smoothNoise(p: vec2<f32>) -> f32 {
    let interp = vec2<f32>(smoothstep(0.0, 1.0, fract(p.x)),smoothstep(0.0, 1.0, fract(p.y)));
    let s = mix(noise(sw(p)), noise(se(p)), interp.x);
    let n = mix(noise(nw(p)), noise(ne(p)), interp.x);
    return mix(s, n, interp.y);
}

fn fractalNoise(p: vec2<f32>) -> f32 {
    var x: f32 = 0.0;
    x += smoothNoise(p);
    x += smoothNoise(p * 2.0) / 2.0;
    x += smoothNoise(p * 4.0) / 4.0;
    x += smoothNoise(p * 8.0) / 8.0;
    x += smoothNoise(p * 16.0) / 16.0;
    x /= 1.0 + 1.0 / 2.0 + 1.0 / 4.0 + 1.0 / 8.0 + 1.0 / 16.0;
    return x;
}

fn movingNoise(p: vec2<f32>, speed: f32) -> f32 {
    let x = fractalNoise(p + globals.time * speed);
    let y = fractalNoise(p - globals.time * speed);
    return fractalNoise(p + vec2<f32>(x, y));
}

fn nestedNoise(p: vec2<f32>, speed: f32) -> f32 {
    let x = movingNoise(p, speed);
    let y = movingNoise(p + vec2<f32>(100.0), speed);
    return movingNoise(p + vec2<f32>(x, y), speed);
}

fn hash23(p: vec2f) -> vec3f {
    let q = vec3f(dot(p, vec2f(127.1, 311.7)),
        dot(p, vec2f(269.5, 183.3)),
        dot(p, vec2f(419.2, 371.9)));
    return fract(sin(q) * 43758.5453);
}

fn voroNoise2(x: vec2f, u: f32, v: f32) -> f32 {
    let p = floor(x);
    let f = fract(x);
    let k = 1. + 63. * pow(1. - v, 4.);
    var va: f32 = 0.;
    var wt: f32 = 0.;
    for(var j: i32 = -2; j <= 2; j = j + 1) {
        for(var i: i32 = -2; i <= 2; i = i + 1) {
            let g = vec2f(f32(i), f32(j));
            let o = hash23(p + g) * vec3f(u, u, 1.);
            let r = g - f + o.xy;
            let d = dot(r, r);
            let ww = pow(1. - smoothstep(0., 1.414, sqrt(d)), k);
            va = va + o.z * ww;
            wt = wt + ww;
        }
    }
    return va / wt;
}

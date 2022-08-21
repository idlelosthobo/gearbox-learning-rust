#[derive(Debug)]
struct Shaft {
    rotation_position: f32,
    rotations: i32,
    gear_teeth: i32,
}

struct GearBox {
    input_shaft: Shaft,
    output_shaft: Shaft,
}

fn rotate_shaft(s: &mut Shaft, degrees: f32) {
    s.rotation_position = s.rotation_position + degrees;

    if s.rotation_position > 360.0 {
        s.rotations = s.rotations + (s.rotation_position / 360.0) as i32;
        s.rotation_position = s.rotation_position % 360.0;
    }
}

fn turn_input_shaft(g: &mut GearBox, degrees: f32) {
    rotate_shaft(&mut g.input_shaft, degrees);
    let output_degrees = (g.output_shaft.gear_teeth as f32 / g.input_shaft.gear_teeth as f32) * degrees;
    rotate_shaft(&mut g.output_shaft, output_degrees)
}

fn main() {
    let mut gear_box = GearBox {
        input_shaft: Shaft { rotation_position: 0.0, rotations: 0, gear_teeth: 30 },
        output_shaft: Shaft { rotation_position: 0.0, rotations: 0, gear_teeth: 5 },
    };

    for _ in 0..1000 {
        const DEGREES: f32 = 15.0;
        turn_input_shaft(&mut gear_box, DEGREES);
        // println!("Turned Gearbox {} Input Shaft now {} Output Shaft Now {}", DEGREES, gear_box.input_shaft.rotation_position, gear_box.output_shaft.rotation_position);
    }

    println!("Input Shaft Rotations {} Output Shaft Rotations {}", gear_box.input_shaft.rotations, gear_box.output_shaft.rotations);
}

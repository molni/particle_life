use bevy::{
    math::{NormedVectorSpace, VectorSpace},
    prelude::*,
};
use rand::Rng;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins,))
        .add_systems(Startup, setup)
        .add_systems(Update, move_particles);
    app.run();
}

const HALF_SIZE: f32 = 100.;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let colors_count = 12;

    let circle = meshes.add(Circle::new(1.0));
    let colors = (0..colors_count)
        // .map(|i| Color::hsl(360. * i as f32 / colors_count as f32, 0.95, 0.7))
        .map(|i| {
            Color::hsl(
                720. * i as f32 / colors_count as f32,
                if i >= 6 { 0.7 } else { 0.95 },
                0.7,
            )
        })
        .collect::<Vec<_>>();

    let mut rng = rand::thread_rng();

    let pos_range = -HALF_SIZE..HALF_SIZE;
    let vel_range = -0.001..0.001;

    let mut particles = (0..1200)
        .map(|id| Particle {
            clr: (id % colors_count) as i8,
            pos: Vec2::new(
                rng.gen_range(pos_range.clone()),
                rng.gen_range(pos_range.clone()),
            ),
            vel: Vec2::new(
                rng.gen_range(vel_range.clone()),
                rng.gen_range(vel_range.clone()),
            ),
        })
        .collect::<Vec<_>>();

    let avg_vel = particles.iter().map(|p| p.vel).sum::<Vec2>() / particles.len() as f32;
    for p in &mut particles {
        p.vel -= avg_vel;
    }

    for s in &[
        Vec2::new(0., 0.),
        Vec2::new(0., 2. * HALF_SIZE),
        Vec2::new(2. * HALF_SIZE, 0.),
        Vec2::new(2. * HALF_SIZE, 2. * HALF_SIZE),
    ] {
        for (i, p) in particles.iter().enumerate() {
            // Distribute colors evenly across the rainbow.
            let color = colors[p.clr as usize];

            commands.spawn((
                Mesh2d(circle.clone()),
                MeshMaterial2d(materials.add(color)),
                Transform::from_xyz(p.pos.x, p.pos.y, 0.0),
                ParticleId {
                    id: i,
                    shift: s.clone(),
                },
            ));
        }
    }

    commands.insert_resource(Particles::new(particles));

    // #[cfg(not(target_arch = "wasm32"))]
    // commands.spawn((
    //     Text::new("Press space to toggle wireframes"),
    //     Node {
    //         position_type: PositionType::Absolute,
    //         top: Val::Px(12.0),
    //         left: Val::Px(12.0),
    //         ..default()
    //     },
    // ));
}

#[derive(Component)]
pub struct ParticleId {
    id: usize,
    shift: Vec2,
}

pub struct Particle {
    clr: i8,
    pos: Vec2,
    vel: Vec2,
}

#[derive(Resource)]
pub struct Particles {
    particles: Vec<Particle>,
}

impl Particles {
    pub fn new(particles: Vec<Particle>) -> Self {
        Self { particles }
    }

    fn get(&self, index: usize) -> &Particle {
        &self.particles[index]
    }
    fn update(&mut self) {
        for i1 in 0..self.particles.len() {
            for i2 in 0..self.particles.len() {
                if i1 != i2 {
                    let (mut p1, p2) = get_mut2(&mut self.particles, i1, i2);
                    let vec = in_bounds(p2.pos - p1.pos);
                    let dist = vec.norm();
                    p1.vel += vec.normalize() * force(p1.clr, p2.clr, dist);
                }
            }
        }

        for p in &mut self.particles {
            p.pos += p.vel;
            p.pos = in_bounds(p.pos);
            p.vel *= 0.9;
        }
    }
}

fn in_bounds(mut v: Vec2) -> Vec2 {
    if v.x < -HALF_SIZE {
        v.x += 2. * HALF_SIZE;
    }
    if v.x > HALF_SIZE {
        v.x -= 2. * HALF_SIZE;
    }
    if v.y < -HALF_SIZE {
        v.y += 2. * HALF_SIZE;
    }
    if v.y > HALF_SIZE {
        v.y -= 2. * HALF_SIZE;
    }
    v
}

fn in_bounds_2(mut v: Vec2) -> Vec2 {
    if v.x < -2. * HALF_SIZE {
        v.x += 4. * HALF_SIZE;
    }
    if v.x > 2. * HALF_SIZE {
        v.x -= 4. * HALF_SIZE;
    }
    if v.y < -2. * HALF_SIZE {
        v.y += 4. * HALF_SIZE;
    }
    if v.y > 2. * HALF_SIZE {
        v.y -= 4. * HALF_SIZE;
    }
    v
}
// fn force(color1: i8, color2: i8, dist: f32) -> f32 {
//     1. / dist / dist
// }

fn on_line((x0, y0): (f32, f32), (x1, y1): (f32, f32), x: f32) -> f32 {
    let p = (x - x0) / (x1 - x0);
    y0 + (y1 - y0) * p
}

fn force(color1: i8, color2: i8, dist: f32) -> f32 {
    // let coef = if color1 == color2 {
    //     -1.
    // } else if color1 - color2 == 1 || color1 == 0 && color2 == 5 {
    //     1.
    // } else {
    //     0.
    // };
    // let coefs = [
    //     [0.1, 0., 0., 0., 0., 0.],
    //     [0., 1., 0., 0., 0., 0.],
    //     [0., 0., 1., 0., 0., 0.],
    //     [0., 0., 0., 1., 0., 0.],
    //     [0., 0., 0., 0., 1., 0.],
    //     [0., 0., 0., 0., 0., 1.],
    // ];
    let p = 0.2;
    let s = 1.;
    let n = -0.02;
    let o = -0.01;
    // let c = -0.2;
    let c = -0.05;
    // let coefs = [
    //     [s, n, o, o, o, p],
    //     [p, s, n, o, o, o],
    //     [o, p, s, n, o, o],
    //     [o, o, p, s, n, o],
    //     [o, o, o, p, s, n],
    //     [n, o, o, o, p, s],
    // ];
    let coefs = [
        [s, n, o, o, o, p, c, n, o, o, o, p],
        [p, s, n, o, o, o, p, c, n, o, o, o],
        [o, p, s, n, o, o, o, p, c, n, o, o],
        [o, o, p, s, n, o, o, o, p, c, n, o],
        [o, o, o, p, s, n, o, o, o, p, c, n],
        [n, o, o, o, p, s, n, o, o, o, p, c],
        [c, n, o, o, o, p, s, n, o, o, o, p],
        [p, c, n, o, o, o, p, s, n, o, o, o],
        [o, p, c, n, o, o, o, p, s, n, o, o],
        [o, o, p, c, n, o, o, o, p, s, n, o],
        [o, o, o, p, c, n, o, o, o, p, s, n],
        [n, o, o, o, p, c, n, o, o, o, p, s],
    ];
    // let d = -1.;
    // let coefs = [
    //     [s, n, o, p, o, o, c, n, o, p, o, o],
    //     [p, s, n, o, o, o, p, c, n, o, o, o],
    //     [o, p, s, n, o, o, o, p, c, n, o, o],
    //     [n, o, p, s, o, o, n, o, p, c, o, o],
    //     [o, o, o, o, n, p, o, o, o, o, n, p],
    //     [o, o, o, o, p, n, o, o, o, o, p, n],
    //     [c, n, o, p, o, o, s, n, o, p, o, o],
    //     [p, c, n, o, o, o, p, s, n, o, o, o],
    //     [o, p, c, n, o, o, o, p, s, n, o, o],
    //     [n, o, p, c, o, o, n, o, p, s, o, o],
    //     [o, o, o, o, n, p, o, o, o, o, n, p],
    //     [o, o, o, o, p, n, o, o, o, o, p, n],
    // ];
    // let coefs = [[s, n, o, p], [p, s, n, o], [o, p, s, n], [o, o, p, s]];
    let range = if color1 == color2 { 30. } else { 50. };
    let coef = coefs[color1 as usize][color2 as usize];
    let result = if dist < 10. {
        on_line((0., -1.), (10., 0.), dist)
    } else if dist < 20. {
        on_line((10., 0.), (20., coef), dist)
    } else if dist < 30. {
        on_line((20., coef), (range, 0.), dist)
    } else {
        0.
    };
    result * 0.01
}

// fn force(color1: i8, color2: i8, dist: f32) -> f32 {
//     0.1
// }

fn get_mut2<T>(slice: &mut [T], i1: usize, i2: usize) -> (&mut T, &mut T) {
    if i1 < i2 {
        let (s1, s2) = slice.split_at_mut(i2);
        (&mut s1[i1], &mut s2[0])
    } else {
        let (s2, s1) = slice.split_at_mut(i1);
        (&mut s1[0], &mut s2[i2])
    }
}

// fn apply_forces(mut query1: Query<&mut Particle>, query2: Query<&Particle>) {
//     for mut p1 in &mut query1 {
//         for p2 in &query2 {
//             if p1.id != p2.id {
//                 let dist = p2.pos - p1.pos;
//                 p1.vel += dist * 0.1;
//             }
//         }
//     }
// }

fn move_particles(
    mut particles: ResMut<Particles>,
    mut query: Query<(&mut Transform, &ParticleId)>,
) {
    particles.update();

    for (mut t, p) in &mut query {
        let part = particles.get(p.id);
        let pos = part.pos + p.shift;
        t.translation = Vec3::new(pos.x, pos.y, 0.);
    }
}

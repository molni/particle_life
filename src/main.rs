use bevy::{
    math::{NormedVectorSpace, VectorSpace},
    prelude::*,
};
use rand::Rng;

trait WorldConfigFactory {
    fn create(&self) -> WorldConfig;
}

struct World0Factory;

impl WorldConfigFactory for World0Factory {
    fn create(&self) -> WorldConfig {
        let force_coefs = vec![vec![-1.]];
        let colors = vec![Color::hsl(240., 0.95, 0.7)];
        let colors_count = colors.len();
        let particle_counts = (0..colors_count).map(|_| 100).collect::<Vec<_>>();
        WorldConfig {
            size: 200.,
            force_coefs,
            colors,
            particle_counts,
        }
    }
}

struct World1Factory;

impl WorldConfigFactory for World1Factory {
    fn create(&self) -> WorldConfig {
        let force_coefs = vec![vec![1., 0.], vec![0.1, -10.]];
        let colors = vec![Color::hsl(120., 0.95, 0.7), Color::hsl(240., 0.95, 0.7)];
        let colors_count = colors.len();
        let particle_counts = vec![200, 800];
        WorldConfig {
            size: 200.,
            force_coefs,
            colors,
            particle_counts,
        }
    }
}

struct World2Factory;

impl WorldConfigFactory for World2Factory {
    fn create(&self) -> WorldConfig {
        let force_coefs = vec![vec![1., -0.1], vec![0.1, 0.]];
        let colors = vec![Color::hsl(0., 0.95, 0.7), Color::hsl(0., 0., 0.7)];
        let colors_count = colors.len();
        let particle_counts = vec![200, 800];
        WorldConfig {
            size: 200.,
            force_coefs,
            colors,
            particle_counts,
        }
    }
}

struct World3Factory;

impl WorldConfigFactory for World3Factory {
    fn create(&self) -> WorldConfig {
        let force_coefs = vec![vec![1., 0., -0.1], vec![0., 1., -0.1], vec![0.1, 0.1, 0.]];
        let colors = vec![
            Color::hsl(0., 0.95, 0.7),
            Color::hsl(120., 0.95, 0.7),
            Color::hsl(0., 0., 0.7),
        ];
        let colors_count = colors.len();
        let particle_counts = vec![200, 200, 800];
        WorldConfig {
            size: 200.,
            force_coefs,
            colors,
            particle_counts,
        }
    }
}

struct World4Factory;

impl WorldConfigFactory for World4Factory {
    fn create(&self) -> WorldConfig {
        let force_coefs = vec![vec![1., -0.1, 0.], vec![0.2, 1., 0.], vec![0.1, 0.1, 0.]];
        let colors = vec![
            Color::hsl(0., 0.95, 0.7),
            Color::hsl(60., 0.95, 0.7),
            Color::hsl(0., 0., 0.7),
        ];
        let colors_count = colors.len();
        let particle_counts = vec![200, 200, 800];
        WorldConfig {
            size: 200.,
            force_coefs,
            colors,
            particle_counts,
        }
    }
}

struct World5Factory;

impl WorldConfigFactory for World5Factory {
    fn create(&self) -> WorldConfig {
        let force_coefs = vec![
            vec![10., 0.2, -0.1, 0.],
            vec![-0.1, 10., 2.1, 0.],
            vec![0.2, -0.1, 10., -0.1],
            vec![0.0, 0.0, 0.1, 0.],
        ];
        let colors = vec![
            Color::hsl(0., 0.95, 0.7),
            Color::hsl(120., 0.95, 0.7),
            Color::hsl(240., 0.95, 0.7),
            Color::hsl(0., 0., 0.7),
        ];
        let colors_count = colors.len();
        let particle_counts = vec![200, 200, 200, 800];
        WorldConfig {
            size: 200.,
            force_coefs,
            colors,
            particle_counts,
        }
    }
}

struct World6Factory;

impl WorldConfigFactory for World6Factory {
    fn create(&self) -> WorldConfig {
        let force_coefs = vec![
            vec![1., 0.2, -0.1, 0.],
            vec![-0.1, 1., 2.1, 0.],
            vec![0.2, -0.1, 1., -0.1],
            vec![0.0, 0.0, 0.1, 0.],
        ];
        let colors = vec![
            Color::hsl(0., 0.95, 0.7),
            Color::hsl(120., 0.95, 0.7),
            Color::hsl(240., 0.95, 0.7),
            Color::hsl(0., 0., 0.7),
        ];
        let colors_count = colors.len();
        let particle_counts = vec![200, 200, 200, 800];
        WorldConfig {
            size: 200.,
            force_coefs,
            colors,
            particle_counts,
        }
    }
}

struct World9Factory;

impl WorldConfigFactory for World9Factory {
    fn create(&self) -> WorldConfig {
        // let coefs = [
        //     [0.1, 0., 0., 0., 0., 0.],
        //     [0., 1., 0., 0., 0., 0.],
        //     [0., 0., 1., 0., 0., 0.],
        //     [0., 0., 0., 1., 0., 0.],
        //     [0., 0., 0., 0., 1., 0.],
        //     [0., 0., 0., 0., 0., 1.],
        // ];
        // let coefs = [
        //     [s, n, o, o, o, p],
        //     [p, s, n, o, o, o],
        //     [o, p, s, n, o, o],
        //     [o, o, p, s, n, o],
        //     [o, o, o, p, s, n],
        //     [n, o, o, o, p, s],
        // ];
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
        let force_coefs = vec![
            vec![s, n, o, o, o, p, c, n, o, o, o, p],
            vec![p, s, n, o, o, o, p, c, n, o, o, o],
            vec![o, p, s, n, o, o, o, p, c, n, o, o],
            vec![o, o, p, s, n, o, o, o, p, c, n, o],
            vec![o, o, o, p, s, n, o, o, o, p, c, n],
            vec![n, o, o, o, p, s, n, o, o, o, p, c],
            vec![c, n, o, o, o, p, s, n, o, o, o, p],
            vec![p, c, n, o, o, o, p, s, n, o, o, o],
            vec![o, p, c, n, o, o, o, p, s, n, o, o],
            vec![o, o, p, c, n, o, o, o, p, s, n, o],
            vec![o, o, o, p, c, n, o, o, o, p, s, n],
            vec![n, o, o, o, p, c, n, o, o, o, p, s],
        ];
        let colors_count = force_coefs.len();
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
        let particle_counts = (0..colors_count).map(|_| 100).collect::<Vec<_>>();
        WorldConfig {
            size: 200.,
            force_coefs,
            colors,
            particle_counts,
        }
    }
}

#[derive(Resource)]
struct WorldConfig {
    size: f32,
    force_coefs: Vec<Vec<f32>>,
    colors: Vec<Color>,
    particle_counts: Vec<usize>,
}

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins,))
        .add_systems(Startup, setup)
        .add_systems(Update, move_particles);
    app.run();
}

// const HALF_SIZE: f32 = 100.;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let world = World9Factory.create();

    commands.spawn(Camera2d);

    let circle = meshes.add(Circle::new(1.0));

    let mut rng = rand::thread_rng();

    let pos_range = -world.size / 2. ..world.size / 2.;
    let vel_range = -0.001..0.001;

    let mut particles = Vec::new();
    for (color, count) in world.particle_counts.iter().enumerate() {
        for _ in 0..*count {
            particles.push(Particle {
                clr: color as i8,
                pos: Vec2::new(
                    rng.gen_range(pos_range.clone()),
                    rng.gen_range(pos_range.clone()),
                ),
                vel: Vec2::new(
                    rng.gen_range(vel_range.clone()),
                    rng.gen_range(vel_range.clone()),
                ),
            });
        }
    }

    let avg_vel = particles.iter().map(|p| p.vel).sum::<Vec2>() / particles.len() as f32;
    for p in &mut particles {
        p.vel -= avg_vel;
    }

    for s in &[
        Vec2::new(0., 0.),
        Vec2::new(0., world.size),
        Vec2::new(world.size, 0.),
        Vec2::new(world.size, world.size),
    ] {
        for (i, p) in particles.iter().enumerate() {
            // Distribute colors evenly across the rainbow.
            let color = world.colors[p.clr as usize].clone();

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
    commands.insert_resource(world);

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
    fn update(&mut self, world: &WorldConfig) {
        for i1 in 0..self.particles.len() {
            for i2 in 0..self.particles.len() {
                if i1 != i2 {
                    let (mut p1, p2) = get_mut2(&mut self.particles, i1, i2);
                    let vec = in_bounds(p2.pos - p1.pos, world.size);
                    let dist = vec.norm();
                    p1.vel += vec.normalize() * force(p1.clr, p2.clr, dist, &world.force_coefs);
                }
            }
        }

        for p in &mut self.particles {
            p.pos += p.vel;
            p.pos = in_bounds(p.pos, world.size);
            p.vel *= 0.9;
        }
    }
}

fn in_bounds(mut v: Vec2, size: f32) -> Vec2 {
    if v.x < -size / 2. {
        v.x += size;
    }
    if v.x > size / 2. {
        v.x -= size;
    }
    if v.y < -size / 2. {
        v.y += size;
    }
    if v.y > size / 2. {
        v.y -= size;
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

fn force(color1: i8, color2: i8, dist: f32, force_coefs: &Vec<Vec<f32>>) -> f32 {
    let range = if color1 == color2 { 30. } else { 50. };
    let coef = force_coefs[color1 as usize][color2 as usize];
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
    world: Res<WorldConfig>,
    mut query: Query<(&mut Transform, &ParticleId)>,
) {
    particles.update(&world);

    for (mut t, p) in &mut query {
        let part = particles.get(p.id);
        let pos = part.pos + p.shift;
        t.translation = Vec3::new(pos.x, pos.y, 0.);
    }
}

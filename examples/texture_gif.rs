use std::{
    cell::RefCell, collections::VecDeque, fs, io::BufReader, ops::DerefMut, path::PathBuf, rc::Rc,
};

use automic_core::{Automaton, Tape, TapeEntry, grid::Grid, kvec::KVec, mmap::MmapVec};
use image::{ImageFormat, Pixel, Rgb, RgbImage};
use rand::Rng;
use rustc_hash::FxHashMap;

fn load_frame(path: PathBuf) -> RgbImage {
    print!("\rloaded frame {path:?}");
    let image_ext = path.extension();
    // TODO: Error check
    let image_ext = image_ext.unwrap();
    let image_format = ImageFormat::from_extension(image_ext);
    let image_format = image_format.unwrap();
    let image_input = std::fs::File::open(path).expect("Could not open image");
    let image_input = BufReader::new(image_input);
    let image = image::load(image_input, image_format)
        .expect("Could not load image")
        .to_rgb8();
    image
}

fn load_video(path: &str) -> Grid<3, Rgb<u8>, MmapVec<'_, Rgb<u8>>> {
    let mut entries: Vec<_> = fs::read_dir(path)
        .expect("")
        .filter_map(|res| res.ok())
        .filter(|f| f.file_type().ok().unwrap().is_file())
        .collect();

    entries.sort_by(|a, b| a.path().cmp(&b.path()));
    // println!("{entries:?} {}", entries.len());
    let frames = entries.len();
    let frame = load_frame(entries[0].path());
    let w = frame.width();
    let h = frame.height();

    let mut grid = Grid::<3, Rgb<u8>, MmapVec<Rgb<u8>>>::fill(
        KVec([0, 0, 0]),
        KVec([w as i64 - 1, h as i64 - 1, frames as i64 - 1]),
        Rgb([0u8, 0, 0]),
    );
    for (i, entry) in entries.iter().enumerate() {
        let frame = load_frame(entry.path());
        for x in 0..w {
            for y in 0..h {
                if let TapeEntry::Hit(p) = grid.get_mut(&KVec([x as i64, y as i64, i as i64])) {
                    *p = *frame.get_pixel(x, y);
                }
            }
        }
    }
    grid
}

fn store_video(path: &str, mut video: Grid<3, Rgb<u8>, MmapVec<'_, Rgb<u8>>>) {
    let KVec(size) = video.size();
    for frame in 0..size[2] {
        print!("\rstoring frame {frame}");
        let mut buffer = RgbImage::new(size[0] as u32, size[1] as u32);
        for x in 0..size[0] {
            for y in 0..size[1] {
                if let TapeEntry::Hit(v) = video.get(&KVec([x, y, frame])) {
                    *buffer.get_pixel_mut(x as u32, y as u32) = *v;
                }
            }
        }
        let frame_path: PathBuf = [path, &format!("frame_{:0>8}.png", frame)].iter().collect();
        buffer.save(frame_path).expect("");
    }
}

fn to_dir(value: u64) -> (i64, i64, i64) {
    match value {
        0 => (1, 0, 0),
        1 => (-1, 0, 0),
        2 => (0, 1, 0),
        3 => (0, -1, 0),

        /*4 => (1, 1, 0),
        5 => (-1, 1, 0),
        6 => (-1, 1, 0),
        7 => (-1, -1, 0),*/
        _ => (0, 0, (-1i64).pow(value as u32)),
        /*12 => (1, 1, -1),
        13 => (-1, 1, -1),
        14 => (-1, 1, -1),
        _ => (-1, -1, -1),*/
    }
}

#[derive(Default)]
struct LazyTable {
    table: FxHashMap<Rgb<u8>, (i64, i64, i64)>,
}

impl LazyTable {
    fn get(&mut self, color: Rgb<u8>) -> (i64, i64, i64) {
        let color = Rgb([color.0[0] >> 4, color.0[1] >> 4, color.0[2] >> 4]);
        if let Some(&v) = self.table.get(&color) {
            v
        } else {
            let v = to_dir(rand::rng().random_range(0..10));
            // let k = color.0[0] as u64 ^ color.0[1] as u64 ^ color.0[2] as u64;
            // let v = to_dir(k % 6);
            self.table.insert(color, v);
            v
        }
    }
}

const N: usize = 3;

#[derive(Default)]
struct AntState {
    _counter: usize,
    queue: VecDeque<Rgb<u8>>,
}

fn gen_trans(
    base: Rgb<u8>,
) -> (
    Automaton<KVec<3>, Rgb<u8>, impl FnMut(&Rgb<u8>) -> (KVec<3>, Rgb<u8>)>,
    impl FnMut() -> (),
) {
    let state: Rc<RefCell<Option<AntState>>> = Default::default();
    let state_cp = state.clone();

    let mut table = LazyTable::default();

    let transition = move |color: &Rgb<u8>| {
        let color = *color;
        // let q: i64 = color.0.iter().map(|&x| x.count_ones() as i64).sum();
        let (c, dx, dy, dt) = {
            let mut binding = state_cp.borrow_mut();
            let state_option = binding.deref_mut();
            if let Some(state) = state_option {
                // state.counter %= N;
                // state.counter += 1;
                // state.counter *= 2;
                // let count = state.counter;
                state.queue.push_back(color);
                // let index = count % (N + 1);
                let q = state.queue[0];
                let p = if state.queue.len() > N {
                    state.queue.remove(rand::rng().random_range(0..N)).unwrap()
                    // let ind = color.0[0] as usize % N;
                    // state.queue.remove(ind).unwrap()
                } else {
                    color
                };
                let (dx, dy, dt) = table.get(p);
                (q, dx, dy, dt)
            } else {
                let mut state = AntState::default();
                state.queue.push_back(color);
                *state_option = Some(state);
                let (dx, dy, dt) = table.get(color);
                (color, dx, dy, dt)
            }
        };
        /*let rot = state_cp.borrow().as_ref().unwrap().counter / 4 % 3;
        let mut x = [
            c.0[1].midpoint(color.0[0]),
            c.0[2].midpoint(color.0[1]),
            c.0[0].midpoint(color.0[2]),
        ];
        // let mut x = c.0;
        // state_cp.borrow_mut().as_mut().unwrap().counter += 1;
        x.rotate_right(rot);
        let q = Rgb(x);
        let (dx, dy, dt) = table.get(q);*/
        // let c = Rgb(x);

        let d = base;
        // d.invert();
        let c = c.map2(&d, |a, b| a ^ b);
        // c.invert();

        // let mut d = color;
        // d.invert();
        // let c = c.map2(&d, |a, b| a.max(b));
        // let c = c.map2(&d, |a, b| a ^ b);
        // let c = c.map2(&color, |a, b| ((a as u16 + b as u16) / 2) as u8);
        (KVec([dx, dy, dt]), c)
    };

    let reset = move || {
        *state.borrow_mut() = None;
    };

    (Automaton::new(transition), reset)
}

fn main() {
    println!("loading frames");
    let mut video = load_video("./in_frames/");
    println!("\nfinished loading");
    println!("running ants");
    let KVec(size) = video.size();

    let life = 2000;
    let ants = 50000;
    let mut wrapped_grid = (&mut video).map(|p| {
        KVec([
            p.0[0].rem_euclid(size[0]),
            p.0[1].rem_euclid(size[1]),
            p.0[2].rem_euclid(size[2]),
        ])
    });

    let mut g_ants = vec![
        gen_trans(Rgb([100, 69, 69])),
        gen_trans(Rgb([217, 163, 54])),
        gen_trans(Rgb([63, 71, 158])),
    ];

    println!("");

    for k in 0..ants {
        let ind = rand::random_range(0..g_ants.len());
        // let &mut (ant, reset) = &mut g_ants.choose(&mut rand::rng()).unwrap();
        (g_ants[ind].1)();
        print!("\rspawning ant {k}");
        let x = rand::rng().random_range(0..size[0]);
        let y = rand::rng().random_range(0..size[1]);
        let z = rand::rng().random_range(0..size[2]);
        (g_ants[ind].0).start(KVec([x, y, z]));
        let mut count = 0;
        while let Some((_, _)) = (g_ants[ind].0).next(&mut wrapped_grid) {
            // println!("\n{p:?} {:?} {:?}\n", video.lower(), video.upper());
            if count > life {
                break;
            }
            count += 1;
        }
    }

    println!("\nstoring frames");
    store_video("./out_frames/", video);
    println!("\nstopping")
}

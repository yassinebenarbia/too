#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct FrameStats {
    pub min: f32,
    pub max: f32,
    pub avg: f32,
}

pub struct Fps<const N: usize> {
    alpha: f32,
    ema: f32,
    min_max: Buffer<N>,
}

impl<const N: usize> Fps<N> {
    pub const fn new() -> Self {
        Self {
            alpha: const { 1.0 / N as f32 },
            ema: 0.0,
            min_max: Buffer::new(),
        }
    }

    pub fn push(&mut self, val: f32) {
        self.ema = self.ema + self.alpha * (val - self.ema);
        self.min_max.push(val);
    }

    pub fn get(&self) -> FrameStats {
        let (min, max) = self.min_max.min_max();
        FrameStats {
            min: max.recip(),
            max: min.recip(),
            avg: self.ema.recip(),
        }
    }
}

struct Buffer<const N: usize> {
    buffer: [f32; N],
    index: u16,
    len: u16,
    min: f32,
    max: f32,
}

impl<const N: usize> Buffer<N> {
    const MASK: u16 = N as u16 - 1;

    const fn new() -> Self {
        assert!(N > 0, "Buffer size cannot be empty");
        assert!(N.is_power_of_two(), "Buffer size must be a power of two");

        Self {
            buffer: [0.0; N],
            index: 0,
            len: 0,
            min: f32::INFINITY,
            max: f32::NEG_INFINITY,
        }
    }

    fn push(&mut self, val: f32) {
        fn adjust(elements: &[f32], val: &mut f32, limit: f32, cmp: fn(f32, f32) -> f32) {
            *val = elements.iter().copied().fold(limit, cmp)
        }

        let old = std::mem::replace(&mut self.buffer[self.index as usize], val);

        self.index = (self.index + 1) & Self::MASK;
        if self.len < N as _ {
            self.len += 1;
        }

        self.min = self.min.min(val);
        self.max = self.max.max(val);

        if old == self.min {
            let elements = &self.buffer[..self.len as usize];
            adjust(elements, &mut self.min, f32::INFINITY, f32::min);
        }
        if old == self.max {
            let elements = &self.buffer[..self.len as usize];
            adjust(elements, &mut self.max, f32::NEG_INFINITY, f32::max);
        }
    }

    const fn min_max(&self) -> (f32, f32) {
        (self.min, self.max)
    }
}

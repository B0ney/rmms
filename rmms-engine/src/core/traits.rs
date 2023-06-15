use super::engine::EngineHandle;

pub trait AudioInputDevice {}

pub trait AudioOutputDevice {
    fn init(handle: EngineHandle) -> Option<Box<Self>> where Self: Sized;
    fn rate(&self) -> u32;
    fn reset(&mut self);
    fn write(&mut self, chunk: &[[f32; 2]]);
}

pub trait PlayHandle: Send + Sync {
    fn next(&mut self) -> Option<[f32; 2]>;
    fn reset(&mut self);
    fn jump(&mut self, tick: usize);
    fn write(&mut self, frames: &mut [[f32; 2]]) -> Option<usize> {
        let mut written: usize = 0;

        for w_frame in frames.iter_mut() {
            match self.next() {
                Some(f) => {
                    w_frame[0] += f[0]; // todo: mix or overwrite?
                    w_frame[1] += f[1]; 
                    written += 1;
                },
                None => {},
            }
        }
        Some(written)
    }
}

impl PlayHandle for Box<dyn PlayHandle> {
    fn next(&mut self) -> Option<[f32; 2]> {
        (**self).next()
    }
    fn reset(&mut self) {
        (**self).reset()
    }
    fn jump(&mut self, tick: usize) {
        (**self).jump(tick)
    }
}

pub trait FrameModifier {
    fn clamp(self) -> Self;
    fn amplify(self, value: f32) -> Self;
    fn force_channel(self, channel: usize) -> Self;
    fn swap_channels(self) -> Self;
}

impl FrameModifier for [f32; 2] {
    fn clamp(self) -> Self {
        self.map(|s| s.clamp(-1.0, 1.0))
    }

    fn amplify(self, value: f32) -> Self {
        self.map(|s| (s * value))
    }

    fn force_channel(mut self, channel: usize) -> Self {
        let sample = self[channel];
        self.fill(sample);
        self
    }

    fn swap_channels(mut self) -> Self {
        self.reverse();
        self
    }
}

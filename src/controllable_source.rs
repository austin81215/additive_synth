use midly::num::u7;
use rodio::Source;

trait ControllableSource: Source where <Self as Iterator>::Item: rodio::Sample {
    fn start_note(&self, note: u7, velocity: u7);
    fn stop_note(&self);
}
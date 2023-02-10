#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct RawSensorData {
    pub voc_ticks: u16,
    pub nox_ticks: u16,
}

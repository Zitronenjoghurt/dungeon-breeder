use crate::state::specimen::SpecimenId;

#[derive(Debug)]
pub struct SpecimenTickSlayRegenEvent {
    pub specimen_id: SpecimenId,
    pub ticks: u64,
}

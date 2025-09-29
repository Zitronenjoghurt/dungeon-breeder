use crate::app::performance_info::PerformanceInfo;
use dungeon_breeder_core::update::report::GameUpdateReport;
use ringbuffer::{AllocRingBuffer, RingBuffer};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct DebugStatsSystem {
    last_update: Instant,
    app_update_secs: AllocRingBuffer<f64>,
    game_update_secs: AllocRingBuffer<f64>,
    game_tick_secs: AllocRingBuffer<f64>,
    pub app_update_min: Duration,
    pub game_update_min: Duration,
    pub game_tick_min: Duration,
    pub app_update_avg: Duration,
    pub game_update_avg: Duration,
    pub game_tick_avg: Duration,
    pub app_update_max: Duration,
    pub game_update_max: Duration,
    pub game_tick_max: Duration,
}

impl Default for DebugStatsSystem {
    fn default() -> Self {
        Self {
            last_update: Instant::now(),
            app_update_secs: AllocRingBuffer::new(60),
            game_update_secs: AllocRingBuffer::new(60),
            game_tick_secs: AllocRingBuffer::new(60),
            app_update_min: Default::default(),
            game_update_min: Default::default(),
            game_tick_min: Default::default(),
            app_update_avg: Default::default(),
            game_update_avg: Default::default(),
            game_tick_avg: Default::default(),
            app_update_max: Default::default(),
            game_update_max: Default::default(),
            game_tick_max: Default::default(),
        }
    }
}

impl DebugStatsSystem {
    pub fn update(&mut self) {
        if self.last_update.elapsed().as_secs() < 1 {
            return;
        }
        self.last_update = Instant::now();

        let app_update_secs = self.app_update_secs.to_vec();
        let game_update_secs = self.game_tick_secs.to_vec();
        let game_tick_secs = self.game_tick_secs.to_vec();

        let mut app_update_min = f64::MAX;
        let mut app_update_max = f64::MIN;
        app_update_secs.iter().for_each(|&x| {
            if x < app_update_min {
                app_update_min = x
            }
            if x > app_update_max {
                app_update_max = x
            }
        });

        let mut game_update_min = f64::MAX;
        let mut game_update_max = f64::MIN;
        game_update_secs.iter().for_each(|&x| {
            if x < game_update_min {
                game_update_min = x
            }
            if x > game_update_max {
                game_update_max = x
            }
        });

        let mut game_tick_min = f64::MAX;
        let mut game_tick_max = f64::MIN;
        game_tick_secs.iter().for_each(|&x| {
            if x < game_tick_min {
                game_tick_min = x
            }
            if x > game_tick_max {
                game_tick_max = x
            }
        });

        let app_update_avg = app_update_secs.iter().sum::<f64>() / app_update_secs.len() as f64;
        let game_update_avg = game_update_secs.iter().sum::<f64>() / game_update_secs.len() as f64;
        let game_tick_avg = game_tick_secs.iter().sum::<f64>() / game_tick_secs.len() as f64;

        self.app_update_min = Duration::from_nanos((app_update_min * 1_000_000_000.0) as u64);
        self.game_update_min = Duration::from_nanos((game_update_min * 1_000_000_000.0) as u64);
        self.game_tick_min = Duration::from_nanos((game_tick_min * 1_000_000_000.0) as u64);
        self.app_update_avg = Duration::from_nanos((app_update_avg * 1_000_000_000.0) as u64);
        self.game_update_avg = Duration::from_nanos((game_update_avg * 1_000_000_000.0) as u64);
        self.game_tick_avg = Duration::from_nanos((game_tick_avg * 1_000_000_000.0) as u64);
        self.app_update_max = Duration::from_nanos((app_update_max * 1_000_000_000.0) as u64);
        self.game_update_max = Duration::from_nanos((game_update_max * 1_000_000_000.0) as u64);
        self.game_tick_max = Duration::from_nanos((game_tick_max * 1_000_000_000.0) as u64);
    }

    pub fn process_report(&mut self, report: &GameUpdateReport) {
        let update_secs = report.time_elapsed.as_secs_f64();
        self.game_update_secs.enqueue(update_secs);

        if report.ticks_elapsed > 0 {
            let tick_secs = update_secs / report.ticks_elapsed as f64;
            self.game_tick_secs.enqueue(tick_secs);
        }
    }

    pub fn process_app_update_duration(&mut self, duration: Duration) {
        self.app_update_secs.enqueue(duration.as_secs_f64());
    }

    pub fn performance_info(&self) -> PerformanceInfo {
        PerformanceInfo {
            app_update_min: self.app_update_min,
            app_update_avg: self.app_update_avg,
            app_update_max: self.app_update_max,
            game_update_min: self.game_update_min,
            game_update_avg: self.game_update_avg,
            game_update_max: self.game_update_max,
            game_tick_min: self.game_tick_min,
            game_tick_avg: self.game_tick_avg,
            game_tick_max: self.game_tick_max,
        }
    }
}

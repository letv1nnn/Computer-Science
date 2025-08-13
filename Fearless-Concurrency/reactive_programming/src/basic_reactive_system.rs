use std::sync::{Arc, LazyLock};
use std::sync::atomic::{AtomicI16, AtomicBool};
use core::sync::atomic::Ordering;
use std::future::Future;
use std::task::Poll;
use std::pin::Pin;
use std::task::Context;
use std::time::{Instant, Duration};

// a basic reactive heating system
// out heater will turn on if the temperature drops below
// the desired temperature setting. Our display will print 
// out the temperature to the terminal if the temperature changes.
//
// temperature and desired state - subjects.
// heater and the display - observers.

static TEMP: LazyLock<Arc<AtomicI16>> = LazyLock::new(|| {
    Arc::new(AtomicI16::new(2090))
});

static DESIRED_TEMP: LazyLock<Arc<AtomicI16>> = LazyLock::new(|| {
    Arc::new(AtomicI16::new(2100))
});

static HEAT_ON: LazyLock<Arc<AtomicBool>> = LazyLock::new(|| {
    Arc::new(AtomicBool::new(false))
});

// the subjects have the following responsibilities
// 1. The current temperature of the system
// 2. The desired temperature that we would like the room to be
// 3. Whether the heater should be on or off. If the bool is true,
//    we instruct the heater to turn on. The heater will turn off
//    if the bool is false


// buildinmg our display observer
pub struct DisplayFuture {
    pub temp_snapshot: i16,
}

impl DisplayFuture {
    pub fn new() -> Self {
        DisplayFuture { 
            temp_snapshot: TEMP.load(Ordering::SeqCst) 
        }
    }
}


impl Future for DisplayFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let current_snapshot = TEMP.load(Ordering::SeqCst); // We get a snapshot of the system as a whole
        let desired_temp = DESIRED_TEMP.load(Ordering::SeqCst);
        let heat_on = HEAT_ON.load(Ordering::SeqCst);

        if current_snapshot == self.temp_snapshot { // we check for any difference between the snapshot of the temperature
            cx.waker().wake_by_ref();               // that the future holds and the current temperature. If there is no 
            return Poll::Pending;                   // difference, there is no point re-rendering the display or marking
        }                                           // any heating decisions, so returning Pending, ending the poll.
        if current_snapshot < self.temp_snapshot && heat_on == false {
            HEAT_ON.store(true, Ordering::SeqCst);
        }
        else if current_snapshot > self.temp_snapshot && heat_on == true {
            HEAT_ON.store(false, Ordering::SeqCst);
        }
        clearscreen::clear().unwrap(); // wipe the terminal for the update
        println!("Temperature: {}\nDesired Temp: {}\nHeater On: {}",
        current_snapshot as f32 / 100.0,
        desired_temp as f32 / 100.0,
        heat_on);
        self.temp_snapshot = current_snapshot; // update the snapshot
        cx.waker().wake_by_ref();
        return Poll::Pending;
    }
}


// building out heater and heat-loss observer
pub struct HeaterFuture {
    pub time_snapshot: Instant,
}

impl HeaterFuture {
    pub fn new() -> Self {
        HeaterFuture { time_snapshot: Instant::now() }
    }
}

impl Future for HeaterFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if HEAT_ON.load(Ordering::SeqCst) == false { // exit, because nothing is going to happen
            self.time_snapshot = Instant::now();
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }
        let current_snapshot = Instant::now();
        if current_snapshot.duration_since(self.time_snapshot) < Duration::from_secs(3) { // if duration is not over 3 seconds,
            cx.waker().wake_by_ref();                                                              // we also exit because time has not
            return Poll::Pending;                                                                  // elapsed for the heater to take effect.
        }
        TEMP.fetch_add(3, Ordering::SeqCst); // increase temp by three, because our both time has elapsed and the HEAT_ON flag is on
        self.time_snapshot = Instant::now();
        cx.waker().wake_by_ref();
        return Poll::Pending;
    }
}

pub struct HeatLossFuture {
    pub time_snapshot: Instant
}

impl HeatLossFuture {
    pub fn new() -> Self {
        HeatLossFuture { time_snapshot: Instant::now() }
    }
}

impl Future for HeatLossFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let current_snapshot = Instant::now();
        if current_snapshot.duration_since(self.time_snapshot) > Duration::from_secs(3) {
            TEMP.fetch_sub(1, Ordering::SeqCst);
            self.time_snapshot = Instant::now();
        }
        cx.waker().wake_by_ref();
        return Poll::Pending;
    }
}

pub async fn temperature_program() {
    let display = tokio::spawn(async {
        DisplayFuture::new().await;
    });
    let heat_loss = tokio::spawn(async {
        HeatLossFuture::new().await;
    });
    let heater = tokio::spawn(async {
        HeaterFuture::new().await;
    });
    display.await.unwrap();
    heat_loss.await.unwrap();
    heater.await.unwrap();
}

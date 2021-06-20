use cjemu_runtime::cjemu_api::VirtualMachine;
use cjemu_runtime::CJEmuVirtualMachine;
use std::sync::{mpsc, Arc, RwLock};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime};

#[derive(Copy, Clone)]
enum EmulationEvent {
    Exit,
    Tick,
    Cycle { ticks: u64, ticks_per_second: f64 },
}

unsafe impl Sync for EmulationEvent {}
unsafe impl Send for EmulationEvent {}

pub struct EmulationHandler {
    virtual_machine: Arc<RwLock<CJEmuVirtualMachine>>,

    join_handle: Option<JoinHandle<()>>,
    event_sender: mpsc::Sender<EmulationEvent>,

    has_exit: bool,
}

impl EmulationHandler {
    pub fn new(virtual_machine: CJEmuVirtualMachine) -> Self {
        let (event_sender, event_receiver) = mpsc::channel();

        let mut vm = Self {
            join_handle: None,

            virtual_machine: Arc::new(RwLock::new(virtual_machine)),
            event_sender,

            has_exit: false,
        };
        vm.join_handle = Some(Self::start_loop(event_receiver, vm.virtual_machine.clone()));
        vm
    }

    fn start_loop(
        event_receiver: mpsc::Receiver<EmulationEvent>,
        virtual_machine: Arc<RwLock<CJEmuVirtualMachine>>,
    ) -> JoinHandle<()> {
        thread::spawn(move || {
            println!("starting emulation loop");

            'main_loop: loop {
                match event_receiver
                    .recv()
                    .expect("failed to receive emulation event")
                {
                    EmulationEvent::Exit => break 'main_loop,
                    EmulationEvent::Tick => {
                        println!("ticking virtual machine");
                        virtual_machine
                            .write()
                            .expect("failed to lock write access for virtual machine")
                            .perform_tick()
                            .expect("failed to tick the virtual machine");
                    }
                    EmulationEvent::Cycle {
                        ticks,
                        ticks_per_second,
                    } => {
                        println!(
                            "running {} cycles on the virtual machine at {} cycles per second",
                            ticks, ticks_per_second
                        );

                        let mut past_ticks = 0;
                        let mut last_tick_time = SystemTime::now();
                        let mut last_print_time = SystemTime::now();
                        let mut last_print_ticks = 0;
                        let secs_per_tick = 1.0 / ticks_per_second;

                        while past_ticks < ticks {
                            // Get the time since the last tick
                            let current_time = SystemTime::now();
                            let elapsed_time_secs = current_time
                                .duration_since(last_tick_time)
                                .unwrap_or_else(|_| {
                                    panic!(
                                        "failed to get duration from {:?} to {:?}",
                                        last_tick_time, current_time
                                    )
                                })
                                .as_secs_f64();

                            let elapsed_print_secs = current_time
                                .duration_since(last_print_time)
                                .unwrap_or_else(|_| {
                                    panic!(
                                        "failed to get duration from {:?} to {:?}",
                                        last_print_time, current_time
                                    )
                                })
                                .as_secs_f64();
                            if elapsed_print_secs > 1.0 {
                                last_print_time = current_time;
                                let t = past_ticks - last_print_ticks;
                                last_print_ticks = past_ticks;
                                println!("processed {} cycles (of {}) in 1 second", t, ticks);
                            }

                            // Check if a tick needs to happen yet
                            if elapsed_time_secs > secs_per_tick {
                                last_tick_time = current_time;

                                // Tick the machine
                                virtual_machine
                                    .write()
                                    .expect("failed to lock write access for virtual machine")
                                    .perform_tick()
                                    .expect("failed to tick the virtual machine");

                                // Increment the tick counter
                                past_ticks += 1;

                                // If we have to wait more than 10 milliseconds,
                                // we might as well sleep this thread
                                if secs_per_tick > 0.010 {
                                    std::thread::sleep(Duration::from_millis(1))
                                }
                            }
                        }

                        println!("processed {} cycles", ticks);
                    }
                }
            }

            println!("exiting emulation loop");
        })
    }

    pub fn exit(&mut self) {
        if !self.has_exit {
            self.has_exit = true;

            // Send the exit event signal to the emulator
            self.event_sender
                .send(EmulationEvent::Exit)
                .map_err(|_| ())
                .expect("failed to send exit event to emulation thread");

            // Join the emulation thread and block until it finishes
            std::mem::replace(&mut self.join_handle, None)
                .expect("missing virtual machine thread join handle")
                .join()
                .expect("failed to join emulation thread");
        }
    }

    pub fn tick(&mut self) {
        self.event_sender
            .send(EmulationEvent::Tick)
            .expect("failed to send tick message to emulation thread");
    }

    pub fn cycle(&mut self, ticks: u64, ticks_per_second: f64) {
        self.event_sender
            .send(EmulationEvent::Cycle {
                ticks,
                ticks_per_second,
            })
            .expect("failed to send tick message to emulation thread");
    }
}

impl Drop for EmulationHandler {
    fn drop(&mut self) {
        self.exit();
    }
}

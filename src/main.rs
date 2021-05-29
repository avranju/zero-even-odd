use std::{
    sync::{Arc, Condvar, Mutex, RwLock},
    thread,
};

#[derive(Copy, Clone, PartialEq)]
enum PrintState {
    ZeroOdd,
    ZeroEven,
    Odd,
    Even,
}

#[derive(Clone)]
struct ZeroEvenOdd {
    print_state: Arc<(Mutex<PrintState>, Condvar)>,
    n: u32,
    current: Arc<RwLock<u32>>,
}

impl ZeroEvenOdd {
    fn new(n: u32) -> Self {
        ZeroEvenOdd {
            print_state: Arc::new((Mutex::new(PrintState::ZeroOdd), Condvar::new())),
            current: Arc::new(RwLock::new(0)),
            n,
        }
    }

    fn zero(&self) {
        let (mx, cvar) = &*self.print_state;
        let mut state = mx.lock().unwrap();

        while *self.current.read().unwrap() < self.n {
            match *state {
                PrintState::ZeroOdd => {
                    print!("0");
                    *state = PrintState::Odd;
                    cvar.notify_all();
                }
                PrintState::ZeroEven => {
                    print!("0");
                    *state = PrintState::Even;
                    cvar.notify_all();
                }
                _ => {
                    state = cvar.wait(state).unwrap();
                }
            }
        }
    }

    fn num(&self, cur_state: PrintState, next_state: PrintState) {
        let (mx, cvar) = &*self.print_state;
        let mut state = mx.lock().unwrap();

        while *self.current.read().unwrap() < self.n {
            if *state == cur_state {
                let current = *self.current.read().unwrap();
                *self.current.write().unwrap() = current + 1;
                print!("{}", current + 1);
                *state = next_state;
                cvar.notify_all();
            } else {
                state = cvar.wait(state).unwrap();
            }
        }
    }

    fn even(&self) {
        self.num(PrintState::Even, PrintState::ZeroOdd);
    }

    fn odd(&self) {
        self.num(PrintState::Odd, PrintState::ZeroEven);
    }
}

fn main() {
    let zeo = ZeroEvenOdd::new(6);

    let zeo1 = zeo.clone();
    let zt = thread::spawn(move || zeo1.zero());

    let zeo2 = zeo.clone();
    let et = thread::spawn(move || zeo2.even());

    let zeo3 = zeo.clone();
    let zo = thread::spawn(move || zeo3.odd());

    zt.join().unwrap();
    et.join().unwrap();
    zo.join().unwrap();
}

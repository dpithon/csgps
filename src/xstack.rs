use crate::Object;

pub trait ProcRunner {
    fn get_object(&mut self) -> Option<Object>;
}

pub struct OnceRunner {
    proc: Vec<Object>,
    pc: usize,
}

impl OnceRunner {
    pub fn new(proc: Vec<Object>) -> Self {
        Self { proc, pc: 0 }
    }

    pub fn reset(&mut self) {
        self.pc = 0;
    }
}

impl ProcRunner for OnceRunner {
    fn get_object(&mut self) -> Option<Object> {
        let object;

        if self.pc < self.proc.len() {
            object = Some(self.proc[self.pc].clone());
            self.pc += 1;
        } else {
            object = None;
        }

        object
    }
}

pub struct RepeatRunner {
    runner: OnceRunner,
    times: i64,
}

impl RepeatRunner {
    pub fn new(proc: Vec<Object>, times: i64) -> Self {
        Self {
            runner: OnceRunner::new(proc),
            times,
        }
    }
}

impl ProcRunner for RepeatRunner {
    fn get_object(&mut self) -> Option<Object> {
        if self.times == 0 {
            return None;
        }

        let mut object = self.runner.get_object();
        if object.is_none() {
            self.times -= 1;
            if self.times == 0 {
                return None;
            }
            self.runner.reset();
            object = self.runner.get_object();
        }

        object
    }
}

#[derive(Default)]
pub struct ExecStack {
    pub stack: Vec<Box<dyn ProcRunner>>,
}

impl ExecStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_runnable(&self) -> bool {
        !self.stack.is_empty()
    }

    pub fn get_object(&mut self) -> Option<Object> {
        loop {
            let object = match self.stack.last_mut() {
                None => None,
                Some(runner) => runner.get_object(),
            };

            if object.is_none() && !self.stack.is_empty() {
                self.stack.pop();
            } else if object.is_none() {
                return None;
            } else {
                return object;
            }
        }
    }

    pub fn push(&mut self, runner: Box<dyn ProcRunner>) {
        self.stack.push(runner);
    }
}

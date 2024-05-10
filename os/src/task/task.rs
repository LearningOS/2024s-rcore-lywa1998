//! Types related to task management

use super::TaskContext;
use crate::timer::get_time_ms;

const MAX_SYSCALL_NUM: usize = 500;

/// The task control block (TCB) of a task.
#[derive(Clone, Copy)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// Fist scheduled time
    pub time: Option<usize>,
    /// syscall_times
    pub syscal_times: [u32; MAX_SYSCALL_NUM],
    /// The task context
    pub task_cx: TaskContext,
}

impl TaskControlBlock {
    /// set task to ready
    pub fn ready(&mut self) {
        self.task_status = TaskStatus::Ready;
    }

    /// set task to running
    pub fn run(&mut self) {
        self.task_status = TaskStatus::Running;
        if self.time.is_none() {
            let time = get_time_ms();
            self.time = Some(time);
        }
    }

    /// set task to exit
    pub fn exit(&mut self) {
        self.task_status = TaskStatus::Exited;
    }

    /// justice task is ready
    pub fn is_ready(&self) -> bool {
        self.task_status == TaskStatus::Ready
    }
    
    /// update call
    pub fn call(&mut self, syscall_id: usize) {
        self.syscal_times[syscall_id] += 1;
    }

    /// return info
    pub fn info(&self) -> TaskInfo {
        TaskInfo { 
            status: self.task_status, 
            syscall_times: self.syscal_times.clone(), 
            time: get_time_ms() - self.time.unwrap_or(0)
        }
    }
}

impl Default for TaskControlBlock {
    fn default() -> Self {
        Self { 
            task_status: Default::default(), 
            time: Default::default(), 
            syscal_times: [0; MAX_SYSCALL_NUM], 
            task_cx: Default::default() 
        }
    }
}

impl AsMut<TaskControlBlock> for TaskControlBlock {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

/// Task information
#[allow(dead_code)]
#[derive(Clone)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// The status of a task
#[derive(Copy, Clone, Default, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    #[default]
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
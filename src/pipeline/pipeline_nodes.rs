

use crossbeam::channel::{self, Receiver, Sender};
use crate::pipeline::pipeline_core::PipelineNode;
use std::thread;



pub struct CommonNode <T> {
    num_threads: i32,
    pre_receiver: Option<Receiver<T>>,

    cur_sender: Option<Sender<T>>,
    cur_receiver: Option<Receiver<T>>,

    work_func: Option<fn (Option<Receiver<T>>, Option<Sender<T>>)>,

}

impl<T: Send + 'static> CommonNode<T> {

    pub fn new(num_threads: i32, work_func: fn (Option<Receiver<T>>, Option<Sender<T>>)) -> Self {
        let (s, r) = channel::unbounded::<T>();
        CommonNode { num_threads: num_threads,
            pre_receiver: None, 
            cur_sender: Some(s), 
            cur_receiver: Some(r),
            work_func: Some(work_func)
         }
    }
}

impl<T: Send + 'static> PipelineNode for CommonNode<T>  {
    type CommType = T;

    fn get_cur_receiver(&mut self) -> Receiver<Self::CommType> {
        self.cur_receiver.take().unwrap()
    }

    fn get_cur_sender(&mut self) -> Sender<Self::CommType> {
        self.cur_sender.take().unwrap()
    }

    fn set_pre_receiver(&mut self, receiver: Receiver<Self::CommType>) {
        self.pre_receiver = Some(receiver);
    }

    fn start(&mut self) {
        
        let pre_recv = self.pre_receiver.take();
        let cur_send = self.cur_sender.take();
        let work_func = self.work_func.take().unwrap();
        for _ in 0..self.num_threads {
            
            let pre_recv_ = if let Some(ref rec) = pre_recv {
                Some(rec.clone())
            } else {
                None
            };

            let cur_send_ = if let Some(ref sed) = cur_send {
                Some(sed.clone())
            } else {
                None
            };
            
            
            thread::spawn(move|| {
                work_func(pre_recv_, cur_send_);
            });
        }

    }


}

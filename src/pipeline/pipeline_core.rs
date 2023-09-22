
use crossbeam::channel::{Receiver, Sender};

/// PipelineNode 做的事情。
/// 1. 接收上一个Node的结果
/// 2. 处理上一个Node的结果
/// 3. 将处理后的结果发送出去。
/// 基于此：每个Node应该包含。上一个Node的Receiver, 以及当前Node的Sender。且需要提供当前Node的Receiver供下一个Node使用。
pub trait PipelineNode {
    type CommType;
    fn set_pre_receiver(&mut self, receiver: Receiver<Self::CommType>);
    
    fn get_cur_receiver(&mut self) -> Receiver<Self::CommType>;
    fn get_cur_sender(&mut self) -> Sender<Self::CommType>;

    fn start(&mut self);
}

pub struct Pipeline<T> {
    nodes: Vec<Box<dyn PipelineNode<CommType = T>>>,

}

impl<T> Pipeline<T> {
    
    pub fn new() -> Self {
        Pipeline { nodes: vec![]}
    }

    pub fn add_node(&mut self, pipeline_node: Box<dyn PipelineNode<CommType = T>>) {
        self.nodes.push(pipeline_node);
    }

    pub fn add_nodes(&mut self, pipeline_nodes: Vec<Box<dyn PipelineNode<CommType = T>>>) {
        for node in pipeline_nodes {
            self.nodes.push(node);
        }
    }

    pub fn build_pipeline(&mut self, init_receiver: Option<Receiver<T>>) {
        let num_nodes = self.nodes.len();
        if num_nodes == 0 {
            panic!("can't build pipeline when the nodes is empty");
        }
        if let Some(rec) = init_receiver {
            self.nodes[0].set_pre_receiver(rec);
        }
        for i in 1..num_nodes {
            let pre_receiver = self.nodes[i-1].get_cur_receiver();
            self.nodes[i].set_pre_receiver(pre_receiver);
        }

    }

    pub fn run_and_fetch_result<Output> (&mut self, callback: fn (Receiver<T>) -> Output) -> Output{
        let num_nodes = self.nodes.len();

        for node in self.nodes.iter_mut() {
            node.start();
        }
        let pipeline_receiver = self.nodes[num_nodes-1].get_cur_receiver();

        callback(pipeline_receiver)
    }

}


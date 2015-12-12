//! Provides NN for a CUDA backend.

use ::operation::*;
use ::binary::*;
use ::plugin::*;
use co::backend::Backend;
use co::device::DeviceType;
use co::memory::MemoryType;
use co::tensor::{SharedTensor, TensorDesc, ITensorDesc};
use co::plugin::Error as PluginError;
use co::frameworks::cuda::{Function, Module, Cuda};
use cudnn::*;
use std::mem::transmute;

lazy_static! {
    static ref SIGMOID: Function = Function::from_isize(1);
}

pub trait ICudnnTensorDesc<T> : ITensorDesc {
    fn get_cudnn_desc(&self) -> Result<TensorDescriptor, PluginError>;
}

impl ICudnnTensorDesc<f32> for TensorDesc {
    fn get_cudnn_desc(&self) -> Result<TensorDescriptor, PluginError> {
        match TensorDescriptor::new(&self.dims_i32(), &self.default_stride_i32(), DataType::Float) {
            Ok(desc) => Ok(desc),
            Err(err) => {
                println!("{:?}", err);
                Err(PluginError::Plugin("Unable to create CuDNN TensorDescriptor."))
            }
        }
    }
}

impl ICudnnTensorDesc<f64> for TensorDesc {
    fn get_cudnn_desc(&self) -> Result<TensorDescriptor, PluginError> {
        match TensorDescriptor::new(&self.dims_i32(), &self.default_stride_i32(), DataType::Double) {
            Ok(desc) => Ok(desc),
            Err(err) => {
                println!("{:?}", err);
                Err(PluginError::Plugin("Unable to create CuDNN TensorDescriptor."))
            }
        }
    }
}

pub trait ICudnn {
    fn cudnn(&self) -> Cudnn {
        Cudnn::new().unwrap()
    }
}

impl ICudnn for Module {}

macro_rules! impl_binary(($($t: ident), +) => (
    $(
        impl INnBinary<$t> for Module {
            type Sigmoid = Function;

            fn sigmoid(&self) -> Self::Sigmoid {
                //lazy_static! {
                //    static ref SIGMOID: Function = Function::from_isize(1);
                //}
                Function::from_isize(1)
            }
        }
    )+
));

macro_rules! impl_sigmoid_for {
    ($t:ident, $b:ty) => (
        impl IOperationSigmoid<$t> for $b {
            fn compute(&self, x: &MemoryType, result: &mut MemoryType) -> Result<(), PluginError> {
                let cudnn = Cudnn::new().unwrap();
                let x = 2 * 2;
                unimplemented!();
                Ok(())
            }
        }
    );
}

macro_rules! impl_plugin_for {
    ($t:ident, $b:ty) => (
        impl_sigmoid_for!($t, $b);
    );
}

impl_binary!(f32, f64);
impl_plugin_for!(f32, Function);
impl_plugin_for!(f64, Function);

impl_plugin_for!(f32, Backend<Cuda>);
impl_plugin_for!(f64, Backend<Cuda>);

impl INn<f32> for Backend<Cuda> {
    type B = Module;

    impl_ops_sigmoid_for!(f32, Backend<Cuda>);

    fn binary(&self) -> &Self::B {
        self.binary()
    }

    fn device(&self) -> &DeviceType {
        self.device()
    }
}

impl INn<f64> for Backend<Cuda> {
    type B = Module;

    impl_ops_sigmoid_for!(f64, Backend<Cuda>);

    fn binary(&self) -> &Self::B {
        self.binary()
    }

    fn device(&self) -> &DeviceType {
        self.device()
    }
}

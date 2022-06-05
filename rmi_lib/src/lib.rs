mod cache_fix;
mod codegen;
mod models;
mod train;

pub mod optimizer;
pub use codegen::rmi_size;
pub use codegen::{output_rmi, output_rmis};
pub use models::KeyType;
pub use models::{ModelInput, RMITrainingData, RMITrainingDataIteratorProvider};
pub use optimizer::find_pareto_efficient_configs;
pub use train::{train, train_bounded, train_for_size, TrainedRMI};

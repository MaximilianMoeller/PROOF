//! Provides configuration parameters for a run of the search algorithm.
use crate::algorithms::local_search::neighbourhood_generator::NeighbourhoodGenerator;
use crate::algorithms::local_search::regularizer::Regularizer;
use serde::{
	Serialize,
	Deserialize,
};
use crate::algorithms::local_search::run_config::Algorithm::BasicHillClimber;

/// Holds information about all possibly configurable parameters of a run.
#[derive(Debug, Serialize, Deserialize)]
pub struct RunConfig<const DATA_DIM: usize> {
	/// How often this configuration should be run.
	pub run_count:                u32,
	/// The path to the data sample file.
	pub data_path:                String,
	/// The metrics for this run.
	pub metrics:                  Option<Metrics>,
	/// Strategies for neighbourhood generation.
	pub neighbourhood_generators: Vec<NeighbourhoodGenerator>,
	/// Regularizer strategy.
	pub regularizer:              Regularizer,
	/// Which Algorithm to use.
	pub algorithm:                Algorithm,
}

impl<const DATA_DIM: usize> Default for RunConfig<DATA_DIM> {
	fn default() -> Self {
		Self {
			run_count:                1,
			data_path:                "data/prepared_data.json".to_string(),
			metrics:                  Some(Metrics {
				picture_frequency:     100,
				regularizer_frequency: 100,
			}),
			neighbourhood_generators: vec![NeighbourhoodGenerator::RemoveOneLiteral],
			regularizer:              Regularizer::DepthAndLength,
			algorithm:                BasicHillClimber(100),
		}
	}
}

/// Differentiates between the different algorithms to be used.
#[derive(Debug, Serialize, Deserialize)]
pub enum Algorithm {
	/// A very basic hill climber holding its maximum iteration count.
	BasicHillClimber(u32),
}

/// Holds information about which metrics should be run and where they should be stored.
#[derive(Debug, Serialize, Deserialize)]
pub struct Metrics {
	/// How often the resulting DNFs should be converted to pictures.
	pub picture_frequency:     u32,
	/// How often the regularizer value of the DNFs should be stored.
	pub regularizer_frequency: u32,
}

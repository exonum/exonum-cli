// Copyright 2019 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Standard Exonum CLI node configuration commands.

pub mod finalize;
pub mod generate_config;
pub mod generate_template;
pub mod maintenance;
pub mod run;
pub mod run_dev;

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use crate::command::finalize::Finalize;
use crate::command::generate_config::GenerateConfig;
use crate::command::generate_template::GenerateTemplate;
use crate::command::maintenance::Maintenance;
use crate::command::run::Run;
use crate::command::run_dev::RunDev;

/// Standard Exonum Core configuration command.
#[derive(StructOpt, Debug, Serialize, Deserialize)]
pub enum Command {
    #[structopt(name = "generate-template")]
    /// Generate common part of the nodes configuration.
    GenerateTemplate(GenerateTemplate),
    #[structopt(name = "generate-config")]
    /// Generate public and private configs of the node.
    GenerateConfig(GenerateConfig),
    #[structopt(name = "finalize")]
    /// Generate final node configuration using public configs
    /// of other nodes in the network.
    Finalize(Finalize),
    #[structopt(name = "run")]
    /// Run the node with provided node config.
    Run(Run),
    #[structopt(name = "run-dev")]
    /// Run the node with auto-generated config.
    RunDev(RunDev),
    #[structopt(name = "maintenance")]
    /// Perform different maintenance actions.
    Maintenance(Maintenance),
}

impl Command {
    /// Wrapper around `StructOpt::from_args` method.
    pub fn from_args() -> Self {
        <Self as StructOpt>::from_args()
    }
}

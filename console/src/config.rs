// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use cloud_config::{
    env_k8s::{execute_create_k8s, CreateK8sOpts},
    update_yaml::{execute_update_yaml, UpdateYamlOpts},
};
use common::{response::Response, status_code::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct UpdateChainConfigOpts {
    create_k8s_opts: CreateK8sOpts,
    update_yaml_opts: Vec<UpdateYamlOpts>,
}

pub fn update_chain_config(args: String) -> Response<()> {
    let Ok(opts) = serde_json::from_str::<UpdateChainConfigOpts>(&args) else {
        return StatusCode::Error.into();
    };
    let Ok(()) = execute_create_k8s(opts.create_k8s_opts) else {
        return StatusCode::Error.into();
    };
    for update_yaml_opt in opts.update_yaml_opts {
        let Ok(()) = execute_update_yaml(update_yaml_opt) else {
            return StatusCode::Error.into();
        };
    }
    StatusCode::Success.into()
}

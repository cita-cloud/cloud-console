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

package com.cita.cloud;

import org.junit.jupiter.api.Test;

class ConsoleTest {
    @Test
    void test() {
        System.out.println(ConsoleV1.updateChainConfig(
                "{\n    \"create_k8s_opts\": {\n        \"chain_name\": \"sla-bft\",\n        \"config_dir\": \"./tmp/\",\n        \"admin\": \"0x9bab5858df4a9e84ff3958884a01a4fce5e07edb\",\n        \"node_list\": \"localhost:40000:node0:k8s,localhost:40001:node1:k8s,localhost:40002:node2:k8s,localhost:40003:node3:k8s\",\n        \"controller_tag\": \"v6.6.3\",\n        \"consensus_image\": \"consensus_bft\",\n        \"consensus_tag\": \"v6.6.3\",\n        \"crypto_tag\": \"v6.6.3\",\n        \"network_tag\": \"v6.6.3\",\n        \"storage_tag\": \"v6.6.3\",\n        \"executor_tag\": \"v6.6.3\"\n    },\n    \"update_yaml_opts\": [\n        {\n            \"chain_name\": \"sla-bft\",\n            \"config_dir\": \"./tmp/\",\n            \"storage_class\": \"ceph-filesystem\",\n            \"docker_registry\": \"docker.io\",\n            \"docker_repo\": \"citacloud\",\n            \"requests_cpu\": \"120m\",\n            \"limits_cpu\": \"1\",\n            \"requests_memory\": \"240Mi\",\n            \"limits_memory\": \"2Gi\",\n            \"domain\": \"node0\"\n        },\n        {\n            \"chain_name\": \"sla-bft\",\n            \"config_dir\": \"./tmp/\",\n            \"storage_class\": \"ceph-filesystem\",\n            \"docker_registry\": \"docker.io\",\n            \"docker_repo\": \"citacloud\",\n            \"requests_cpu\": \"120m\",\n            \"limits_cpu\": \"1\",\n            \"requests_memory\": \"240Mi\",\n            \"limits_memory\": \"2Gi\",\n            \"domain\": \"node1\"\n        },\n        {\n            \"chain_name\": \"sla-bft\",\n            \"config_dir\": \"./tmp/\",\n            \"storage_class\": \"ceph-filesystem\",\n            \"docker_registry\": \"docker.io\",\n            \"docker_repo\": \"citacloud\",\n            \"requests_cpu\": \"120m\",\n            \"limits_cpu\": \"1\",\n            \"requests_memory\": \"240Mi\",\n            \"limits_memory\": \"2Gi\",\n            \"domain\": \"node2\"\n        },\n        {\n            \"chain_name\": \"sla-bft\",\n            \"config_dir\": \"./tmp/\",\n            \"storage_class\": \"ceph-filesystem\",\n            \"docker_registry\": \"docker.io\",\n            \"docker_repo\": \"citacloud\",\n            \"requests_cpu\": \"120m\",\n            \"limits_cpu\": \"1\",\n            \"requests_memory\": \"240Mi\",\n            \"limits_memory\": \"2Gi\",\n            \"domain\": \"node3\"\n        }\n    ]\n}"));

        // System.out.println(ConsoleV1.setBlockInterval("localhost:50004",
        // "localhost:50005", "79803604a6a6e0fc00291e8b9e1ef3f20af1af59", "4"));
        // String[] validators = {"", "", "" };
        // System.out.println(ConsoleV1.updateValidators("localhost:50004", "", "",
        // validators));
        // System.out.println(ConsoleV1.emergencyBrake("localhost:50004",
        // "localhost:50005", "79803604a6a6e0fc00291e8b9e1ef3f20af1af59", "off"));
        // System.out.println(ConsoleV1.setQuotaLimit("localhost:50004",
        // "localhost:50005", "79803604a6a6e0fc00291e8b9e1ef3f20af1af59",
        // "1073741716"));
        // System.out.println(ConsoleV1.updateAdmin("localhost:50004", "", "", ""));

        System.out.println(ConsoleV1.getBlockNumber());
        System.out.println(ConsoleV1.getSystemConfig());
        System.out.println(ConsoleV1.getAbi("b3eefbf4e5280217da74b83f316c5711827933a0"));
        System.out.println(ConsoleV1.getAccountNonce("757ca1c731a3d7e9bdbd0e22ee65918674a77bd7"));
        System.out.println(ConsoleV1.getBalance("757ca1c731a3d7e9bdbd0e22ee65918674a77bd7"));
        System.out.println(ConsoleV1.getBlockHash("100"));
        System.out.println(ConsoleV1.getBlock("100"));
        System.out.println(ConsoleV1.getCode("b3eefbf4e5280217da74b83f316c5711827933a0"));
        System.out.println(ConsoleV1.getReceipt("0x9251b32a617e08e8f5deb8468229269b969489069752c3e67c69a1f44909ca1b"));
        System.out.println(ConsoleV1.getTx("0x9251b32a617e08e8f5deb8468229269b969489069752c3e67c69a1f44909ca1b"));
        System.out.println(ConsoleV1.call("0x06661abd", "string", "0", "0xb3eefbf4e5280217da74b83f316c5711827933a0"));
        System.out.println(ConsoleV1.create("20",
                "0x608060405234801561001057600080fd5b5060f58061001f6000396000f3006080604052600436106053576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806306661abd1460585780634f2be91f146080578063d826f88f146094575b600080fd5b348015606357600080fd5b50606a60a8565b6040518082815260200191505060405180910390f35b348015608b57600080fd5b50609260ae565b005b348015609f57600080fd5b5060a660c0565b005b60005481565b60016000808282540192505081905550565b600080819055505600a165627a7a72305820faa1d1f51d7b5ca2b200e0f6cdef4f2d7e44ee686209e300beb1146f40d32dee0029",
                "0x0"));
        System.out.println(ConsoleV1.sendTx("20", "0x4f2be91f", "0x524268b46968103ce8323353dab16ae857f09a6f", "0x0"));
    }
}

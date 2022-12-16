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
        System.out.println(ConsoleV1.updateChainConfig("{}"));
        System.out.println(ConsoleV1.setBlockInterval("localhost:50004", "", "", "4"));
        // String[] validators = {"", "", "" };
        // System.out.println(ConsoleV1.updateValidators("localhost:50004", "", "", validators));
        System.out.println(ConsoleV1.emergencyBrake("localhost:50004", "", "", "off"));
        System.out.println(ConsoleV1.setQuotaLimit("localhost:50004", "", "", "1073741716"));
        // System.out.println(ConsoleV1.updateAdmin("localhost:50004", "", "", ""));
    }
}

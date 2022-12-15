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

package com.cita.v1;

import org.junit.jupiter.api.Test;

class ConsoleTest {
    @Test
    void test() {
        System.out.println(Console.updateChainConfig("{}"));
        System.out.println(Console.setBlockInterval("localhost:50004", "", "", "4"));
        String[] validators = { "79803604a6a6e0fc00291e8b9e1ef3f20af1af59", "e963afe7b072b3346fd0ecccdfd907f0984942af",
                "4a7c4fd1270b0e1ef1916fcb77666e793358327c" };
        System.out.println(Console.updateValidators("localhost:50004", "", "", validators));
        System.out.println(Console.emergencyBrake("localhost:50004", "", "", "off"));
        System.out.println(Console.setQuotaLimit("localhost:50004", "", "", "1073741716"));
        System.out.println(Console.updateAdmin("localhost:50004", "", "", "e963afe7b072b3346fd0ecccdfd907f0984942af"));
    }
}

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

public class ConsoleV1 {
    // if crypto_addr is empty, it will generate account from local private_key to
    // send utxo_tx
    // if crypto_addr is not empty, user also needs to input account_addr
    public static native String updateAdmin(String controller_addr, String crypto_addr, String account_addr,
            String new_admin);

    public static native String setBlockInterval(String controller_addr, String crypto_addr, String account_addr,
            String block_interval);

    public static native String updateValidators(String controller_addr, String crypto_addr, String account_addr,
            String[] validators);

    public static native String emergencyBrake(String controller_addr, String crypto_addr, String account_addr,
            String on_off);

    public static native String setQuotaLimit(String controller_addr, String crypto_addr, String account_addr,
            String quota_limit);

    public static native String updateChainConfig(String input);

    static {
        System.loadLibrary("console_dylib_v1");
    }
}

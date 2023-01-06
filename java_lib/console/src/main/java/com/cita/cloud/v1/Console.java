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

package com.cita.cloud.v1;

class Console {
    // if crypto_addr is empty, it will generate account from local private_key to
    // send utxo_tx
    // if crypto_addr is not empty, user also needs to input admin_addr
    static native String updateAdmin(String controller_addr, String crypto_addr, String admin_addr, String new_admin);

    static native String setBlockInterval(String controller_addr, String crypto_addr, String admin_addr, String block_interval);

    static native String updateValidators(String controller_addr, String crypto_addr, String admin_addr, String[] validators);

    static native String emergencyBrake(String controller_addr, String crypto_addr, String admin_addr, String on_off);

    static native String setQuotaLimit(String controller_addr, String crypto_addr, String admin_addr, String quota_limit);

    static native String updateChainConfig(String input);

    static native String getBlockNumber(String cache_addr);

    static native String getSystemConfig(String cache_addr);

    static native String getAbi(String cache_addr, String address);

    static native String getAccountNonce(String cache_addr, String address);

    static native String getBalance(String cache_addr, String address);

    static native String getBlockHash(String cache_addr, String block_number);

    static native String getBlock(String cache_addr, String hash_or_height);

    static native String getCode(String cache_addr, String address);

    static native String getReceipt(String cache_addr, String hash);

    static native String getTx(String cache_addr, String hash);

    static native String call(String cache_addr, String data, String from, String height, String to);

    static native String create(String cache_addr, String block_count, String data, String value);

    static native String sendTx(String cache_addr, String block_count, String data, String to, String value);

    static {
        System.loadLibrary("console_dylib_v1");
    }
}

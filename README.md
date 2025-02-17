# Cloud Console

## Usage

### 1. 确定链的类型与版本，修改`console`依赖版本

./console/Cargo.toml
```toml
cldi = { git = "https://github.com/cita-cloud/cloud-cli", branch = "lib_for_console" }
cloud-config = { git = "https://github.com/cita-cloud/cloud-config", branch = "lib_for_console" }
cita_cloud_proto = { package = "cita_cloud_proto", git = "https://github.com/cita-cloud/cloud-common-rs.git" }
cloud-util = { package = "cloud-util", git = "https://github.com/cita-cloud/cloud-common-rs.git" }
```

### 2. 修改`dylib`中所有`jni函数名`包含的java包名与类名

./dylib/src/config.rs
```rust
...
#[no_mangle]
pub extern "system" fn Java_com_cita_cloud_v1_Console_updateChainConfig(
    env: JNIEnv,
...
```

### 3. 编译rust端代码，得到动态链接库，将其放置在`java.library.path`对应的路径下并按类型与版本进行重命名

```shell
❯ cargo build -r

❯ exa -TL 1 target/release/
├── ...
├── libconsole_dylib.dylib
└── ...

MacOs:
❯ cp './target/release/libconsole_dylib.dylib' ~/Library/java/Extensions/libconsole_dylib_v1.dylib
Linux:
❯ sudo cp './target/release/libconsole_dylib.so' /usr/java/packages/lib/libconsole_dylib_v1.so

❯ exa -TL 1 ~/Library/java/Extensions/
├── ...
└── libconsole_dylib_v1.dylib
```

### 4. 在`java_lib`中新建对应版本类型的api类，暴露动态链接库中的方法

./java_lib/console/src/main/java/com/cita/cloud/v1/Console.java
```java
package com.cita.cloud;

public class Console {
    ...
    
    public static native String updateChainConfig(String input);

    ...
    
    static {
        System.loadLibrary("console_dylib_v1");
    }
}
```

### 5. 编译`java_lib`，得到jar包

```shell
❯ cd java_lib
❯ gradle build
```

### 6. 在应用中引用，示例`examples/java_example`

运行前的准备
- 正确引用动态连接库
- 准备发送治理交易需要的私钥文件**或**`Crypto`服务。
  - 执行应用的目录下需存在链`admin`账户对应的`private_key`文件。
  - 或运行链`admin`账户对应的`Crypto`服务，此时需要传入参数`crypto_addr`、`admin_addr`.

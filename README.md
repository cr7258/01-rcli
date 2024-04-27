# 第一周：魔法神箭：从 Hello world 到实用的 CLI 工具

* [添加依赖](#添加依赖)
* [Blake3 和 Ed25519](#blake3-和-ed25519)
* [主要代码讲解](#主要代码讲解)
* [运行单元测试](#运行单元测试)
* [验证效果](#验证效果)
    * [Blake3](#blake3)
    * [Ed25519](#ed25519)

## 添加依赖

```bash
cargo add blake3
cargo add ed25519-dalek --features rand_core
```

## Blake3 和 Ed25519

`blake3` 和 `ed25519` 是两种不同的加密原语，它们的密钥用途和性质也有所不同。

**blake3**:

- `blake3` 是一种加密哈希函数（Cryptographic Hash Function）。
- 它没有“密钥”的概念，而是直接对任意长度的输入数据进行哈希运算，生成固定长度的哈希值（通常为 32 字节）。
- 哈希函数的主要用途是验证数据的完整性，以确保数据在传输或存储过程中没有被篡改。
- blake3 哈希函数本身不需要任何密钥，但可以使用可选的“键控码”（keyed mode）来生成不同的哈希函数实例，从而增加安全性和抗冲突能力。
- 需要注意的是，相同的键控码和输入数据会产生相同的哈希值。但不同的键控码会导致完全不同的哈希函数行为，即使输入数据相同，也会得到完全不同的哈希值。

**ed25519**:

- `ed25519` 是一种基于椭圆曲线的数字签名算法。
- 它使用一对数学上生成的密钥：一个公钥（public key）和一个私钥（secret key）。
- 私钥用于对数据进行数字签名，而公钥则用于验证签名的真实性和完整性。
- `ed25519` 算法的主要用途是提供数据的真实性认证和防止否认性（）non-repudiation），常用于软件分发、代码签名、区块链交易等场景。
- `ed25519` 密钥对是使用加密安全的随机数生成器生成的，私钥必须妥善保管。防止泄露。

## 主要代码讲解

TextSigner 和 TextVerifier 是两个 trait，它们定义了签名和验证的行为。任何实现了这些特性的类型都需要提供 sign 和 verify 方法。
Blake3 和 Ed25519Signer/Ed25519Verifier 是结构体（structs），它们分别实现了 TextSigner 和 TextVerifier 特性。

```rust
pub trait TextSigner {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool>;
}

impl TextSigner for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        ......
    }
}

impl TextVerifier for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        ......
    }
}

impl TextSigner for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        ......
    }
}

impl TextVerifier for Ed25519Verifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        ......
    }
}
```

Blake3 和 Ed25519Signer/Ed25519Verifier 结构体的还分别提供了关联函数 try_new、new 和 generate。

```rust
impl Blake3 {
    // try_new 函数是一个构造函数，用于尝试创建一个新的实例。在这个上下文中，它被用于 Blake3 和 Ed25519Signer 类。
    // 这个函数接受一个实现 AsRef<[u8]> 特性的参数，然后尝试将其转换为一个固定长度的数组（[u8; 32]）。
    // 这意味着你可以传递一个Vec<u8>，一个[u8; N]，或者任何其他可以被看作是[u8]引用的类型。
    // 这提供了很大的灵活性，因为你可以接受多种类型的参数，只要它们可以被看作是[u8]的引用。
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }

    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        let key = process_genpass(32, false, false, false, false)?;
        let mut map = HashMap::new();
        map.insert("blake3.txt", key.as_bytes().to_vec());
        Ok(map)
    }
}
```


这段代码通过使用 trait 对象 `Box<dyn TextSigner>`，实现了动态分发。
在运行时，它可以根据传入的算法格式（TextSignFormat）动态选择合适的签名器实（Blake3 或 Ed25519Signer）。
这种设计提高了代码的灵活性和可扩展性。如果将来需要添加新的签名算法，只需实现 TextSigner trait 即可，而无需修改 process_text_sign 函数的逻辑。

在这个特定的上下文中，Box::new 用于创建一个实现了 TextSigner 或 TextVerifier trait 的对象的堆分配实例。这是因为 trait 对象的大小在编译时是未知的，所以它们必须被存储在堆上，而不能被存储在栈上。

```rust
pub fn process_text_sign(
    reader: &mut dyn Read,
    key: &[u8],
    format: TextSignFormat
) -> Result<Vec<u8>> {
    let signer: Box<dyn TextSigner> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Signer::try_new(key)?),
    };

    signer.sign(reader)
}
```

## 运行单元测试

```bash
cargo nextest run
   Compiling rcli v0.1.0 (/Users/I576375/Code/rust/rust-learning/geek-rust-bootcamp/01-rcli)
    Finished test [unoptimized + debuginfo] target(s) in 0.97s
    Starting 5 tests across 2 binaries (run ID: 097ab943-4baa-4113-8797-6f89bcb4af55, nextest profile: default)
        PASS [   0.008s] rcli cli::tests::test_verify_file
        PASS [   0.008s] rcli process::b64::tests::test_process_encode
        PASS [   0.009s] rcli process::b64::tests::test_process_decode
        PASS [   0.008s] rcli process::text::tests::test_blake3_sign_verify
        PASS [   0.012s] rcli process::text::tests::test_ed25519_sign_verify
------------
     Summary [   0.014s] 5 tests run: 5 passed, 0 skipped
```

## 验证效果

### Blake3

生成 Blake3 的键控码，使用相同的输入数据和键控码，可以得到相同的哈希值。

```bash
cargo run -- text generate -o fixtures
```

### Ed25519

生成 Ed25519 的密钥对，可以使用私钥对数据进行签名，然后使用公钥验证签名的有效性。

```bash
cargo run -- text generate -o fixtures --format ed25519
```

使用 Ed25519 的私钥对数据进行签名：

```bash
cargo run -- text sign -k fixtures/ed25519.sk --format ed25519

# 输入
hello
# 然后回车，按 ctrl + d，会显示签名结果
FjEWn2KDz8rUbfcPVMu476qA02uhKaChM9scukbDjzPWZ_qKg5woqQWzwqf5NoZFg-bwXMCWCeK5mGrzR7iaAg
```

使用 Ed25519 的公钥验证签名：

```bash
cargo run -- text verify -k fixtures/ed25519.pk --format ed25519 --sig FjEWn2KDz8rUbfcPVMu476qA02uhKaChM9scukbDjzPWZ_qKg5woqQWzwqf5NoZFg-bwXMCWCeK5mGrzR7iaAg

# 输入
hello!
# 然后回车，按 ctrl + d，会显示验证结果
✓ Signature verified
```

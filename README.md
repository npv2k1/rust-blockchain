# Rust blockchain

## Cấu trúc của một khối

```rust
struct Block {
    hash: String,
    data: String,
    prev_hash: String,
    nonce: u64,
}
```

`hash`: Mã băm của khối \
`data`: Dữ liệu của khối \
`prev_hash`: Mã băm của khối trước đó \
`nonce`: Số nguyên không âm

## Cấu trúc của một chuỗi khối

```rust
struct Blockchain {
    blocks: Vec<Block>,
}
```

`blocks`: Một mảng các khối

## Proof of work

```rust
struct ProofOfWork {
    block: Block,
    target: String,
}
```

`block`: Khối cần tìm nonce \
`target`: Một chuỗi nhị phân có độ dài bằng với số lượng bit 0 ở đầu của mã băm của khối

### Cách hoạt động của Proof of work

Cố gắng tìm một số nguyên không âm `nonce` sao cho khi thêm `nonce` vào khối, mã băm của khối đó bắt đầu bằng với `target`.

### Validate proof of work

```rust
fn validate(&self) -> bool {
      let mut hasher = Sha256::new();
      let data = self.prepare_data(self.block.nonce);
      hasher.update(data);

      let hash: String = hasher
          .finalize()
          .iter()
          .map(|x| format!("{:02x}", x))
          .collect();
      hash.starts_with(&self.target)
  }
```

### Before run
```bash
echo "TELEOXIDE_TOKEN=your_token" > .env
```

### Run
```bash
cargo build --release
./target/release/eq-ala
```

### Docker x86-x64
```bash
docker build -t eq-ala -f Dockerfile.alpine . 
docker run -d --name eq-ala eq-ala
```
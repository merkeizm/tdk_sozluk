## Türkçe Sözlük (TDK)
**Bu Modül Sayesinde Girdiğiniz Kelimeden Bilgiler Alabilirsiniz.**

## Kurulum
```bash
cargo add tdk_sozluk
```

## Kullanım
```rust
use tdk_sozluk::{kelime, kelime_sesli_okunus};

#[tokio::main]
async fn main() {
    let sonuc = kelime("bilgisayar").await.unwrap();

    println!("> Kelime: {}", sonuc.kelime);
    for anlam in sonuc.anlamlar {
        println!("> Anlam: {}", anlam.anlam);
        if !anlam.ornek_cumleler.is_empty() {
            println!("> Örnek: {}", anlam.ornek_cumleler[0]);
        }
    }

    let ses = kelime_sesli_okunus("bilgisayar").await.unwrap();
    println!("> Sesli Okunuş: {}", ses);
}
```

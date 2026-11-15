# NovaDock (Aşama 1)

Gereksinimler: Windows 11, Node 20+, Rust (MSVC), WebView2.

    npm install
    npm run tauri dev

Acil durum: uygulama donarsa görev çubuğunu bekçi süreç geri yükler.
Bu olmazsa Görev Yöneticisi > Windows Gezgini > Yeniden başlat.

## .exe üretmek (bilgisayarında Rust kurmadan)
1. GitHub'da boş bir depo aç, bu klasörün içeriğini yükle.
2. Actions > "Windows kurulum dosyası" > Run workflow.
3. ~10 dk sonra Artifacts bölümünden NovaDock-installer indir, çalıştır.

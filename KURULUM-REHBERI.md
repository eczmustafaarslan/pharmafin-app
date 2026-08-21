# Pharmafin'i Masaüstü Uygulamasına Çevirme — Adım Adım Rehber

Bu rehber, elindeki `pharmafin-tauri.zip` dosyasını kullanarak Pharmafin'in
hem macOS (.dmg) hem Windows (.exe/.msi) sürümünü **ücretsiz** şekilde,
GitHub'ın kendi bilgisayarları üzerinde otomatik olarak derletmeni anlatır.
Hiçbir yerde ödeme yapmayacaksın; sadece imzasız (unsigned) dağıtım
kullandığımız için kurulumda tek bir ekstra tıklama gerekecek (aşağıda
anlatılıyor).

## 1) GitHub hesabı oluştur (bir kereye mahsus)

1. https://github.com/join adresine git.
2. E-posta, kullanıcı adı ve şifre belirleyip hesabını oluştur.
3. E-postana gelen doğrulama kodunu gir.

## 2) Yeni bir depo (repository) oluştur (bir kereye mahsus)

1. Giriş yaptıktan sonra sağ üstteki **+** işaretine tıkla > **New repository**.
2. **Repository name** kısmına `pharmafin-app` yaz.
3. **Public** seçeneğinin işaretli olduğundan emin ol (arkadaşların
   uygulamayı indirebilsin diye depo herkese açık olmalı; içinde hasta/mali
   veri olmayacak, sadece program kodu olacak).
4. Başka hiçbir kutuyu işaretlemeden **Create repository** butonuna bas.

## 3) Proje dosyalarını GitHub'a yükle

1. Sana gönderdiğim `pharmafin-tauri.zip` dosyasını bilgisayarına indir ve
   çift tıklayıp klasöre çıkart (Finder'da otomatik olur).
2. Az önce oluşturduğun `pharmafin-app` deposunun sayfasında **"uploading an
   existing file"** bağlantısına (veya **Add file > Upload files**
   menüsüne) tıkla.
3. Çıkarttığın klasörün İÇİNDEKİ dosya ve klasörleri (klasörün kendisini
   değil, içindekileri: `.github`, `src`, `src-tauri`, `package.json` vb.)
   sürükleyip bu sayfaya bırak.
4. Sayfanın altındaki **Commit changes** butonuna bas.

Bundan sonra her düzeltme istediğinde sana güncellenmiş `pharmafin-tauri.zip`
dosyasını tekrar göndereceğim; sen de aynı şekilde çıkarttığın dosyaları
tekrar bu depoya sürükleyip bırakacaksın (üzerine yazılacak, sorun değil).

## 4) Derlemeyi başlat

1. Depo sayfasında üstteki **Actions** sekmesine tıkla.
2. Eğer "workflows aren't being run" gibi bir uyarı çıkarsa **"I understand
   my workflows, go ahead and enable them"** butonuna bas (bir kereye
   mahsus).
3. Solda **"Pharmafin Masaüstü Uygulamasını Derle"** iş akışına tıkla.
4. Sağda çıkan **Run workflow** düğmesine bas, açılan kutuya bir sürüm adı
   yaz (ilk seferde `v1.0.0` yazman yeterli, sonraki her seferinde `v1.0.1`,
   `v1.0.2` gibi bir sayı artırman gerekir) ve tekrar yeşil **Run workflow**
   butonuna bas.
5. Ekranda "Pharmafin Masaüstü Uygulamasını Derle" adında sarı/turuncu bir
   satır belirecek, birkaç dakika içinde (genelde 8-12 dakika) yeşil tik
   olacak. Bu, hem Mac hem Windows sürümünün hazır olduğu anlamına gelir.

## 5) Uygulamayı indir

1. Depo sayfasında sağdaki **Releases** bölümüne (veya `pharmafin-app`
   deposunun ana sayfasında sağ tarafta görünen sürüm etiketine) tıkla.
2. Az önce yazdığın sürüm adını (ör. `v1.0.0`) göreceksin, altında indirilecek
   dosyalar listelenecek:
   - Mac için: `.dmg` uzantılı dosya
   - Windows için: `.msi` (veya `.exe`) uzantılı dosya
3. Kendi bilgisayarın için olanı indir.

## 6) Kurulum

**Mac'te:** İndirilen `.dmg` dosyasına çift tıkla, açılan pencerede
Pharmafin simgesini "Applications" klasörüne sürükle. İlk açılışta "Apple
geliştiriciyi doğrulayamadı" uyarısı çıkarsa: uygulama simgesine **sağ
tık > Aç**, çıkan pencerede tekrar **Aç** butonuna bas. Bu sadece ilk
açılışta gerekir.

Bazı Mac'lerde (özellikle M1/M2/M3 gibi Apple Silicon işlemcili olanlarda)
bunun yerine daha sert bir **"Pharmafin bozuk, çöp kutusuna taşınsın mı"**
uyarısı çıkabilir. Uygulama bozuk değildir, imzasız olduğu için macOS böyle
diyor — şu adımlarla açılır:

1. Pharmafin.app'i Applications klasörüne taşıdığından emin ol.
2. **Cmd + Boşluk** tuşlarına bas, `Terminal` yaz, Enter'a bas.
3. Açılan pencereye şunu yazıp Enter'a bas:
   ```
   xattr -cr /Applications/Pharmafin.app
   ```
4. Pharmafin'i tekrar açmayı dene, bu sefer açılacaktır.

Bu adım her yeni sürüm indirişinde (dosya her indirildiğinde macOS'un
eklediği "internetten indirildi" işareti yüzünden) tekrar gerekebilir —
sorun değil, aynı komutu tekrar çalıştırman yeterli.

**Windows'ta:** İndirilen `.msi` dosyasına çift tıkla. "Windows
bilgisayarınızı korudu" (SmartScreen) uyarısı çıkarsa: **"Daha fazla
bilgi"** yazısına tıkla, sonra çıkan **"Yine de çalıştır"** butonuna bas.
Bu da sadece ilk kurulumda gerekir.

## 7) Arkadaşınla paylaşmak istersen

Deponun Releases sayfasının bağlantısını (adres çubuğundaki linki) ona
gönderebilirsin; depo Public olduğu için GitHub hesabı olmasa bile o linkten
indirip 6. adımdaki gibi kurabilir.

## 8) Verilerin her iki bilgisayarda da görünmesi

Uygulamayı hangi bilgisayara kurarsan kur, içindeki **"Otomatik Kayıt"**
butonuna basıp Google Drive/iCloud gibi bulutla senkron olan bir klasör
içinde **aynı isimde bir dosya** seçmen yeterli (örn. her iki bilgisayarda
da `Google Drive/Pharmafin/pharmafin-veri.json`). Bir bilgisayarda yapılan
değişiklik bulut klasörü aracılığıyla diğer dosyaya ulaştığında, açık duran
diğer Pharmafin penceresi bunu birkaç saniye içinde otomatik fark edip kendi
ekranını günceller — ekstra bir "Geri Yükle" işlemi yapmana gerek yok.

---

Bir yerde takılırsan, hangi adımda ne gördüğünü yazman yeterli; oradan devam
ederiz.

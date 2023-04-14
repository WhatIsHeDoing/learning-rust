Write-Output "Cleaning..."
cargo clean

Write-Output "Building debug and release..."
cargo build
cargo build --release
$exe = "whatishedoing_learning_rust.exe"

Write-Output "Compressing..."
upx --best --lzma .\target\release\$exe

$debugSize = ((Get-Item .\target\debug\$exe).length / 1KB) -as[int]
$releaseSize = ((Get-Item .\target\release\$exe).length / 1KB) -as[int]
$percent = $releaseSize /  $debugSize
Write-Output ("Finished: the release exe is {0:P} of the debug size!” -f $percent)

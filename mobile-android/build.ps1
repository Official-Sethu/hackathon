$env:JAVA_HOME = "C:\Users\ASUS\Desktop\hackathon\mobile-android\jdk17_root\jdk-17.0.20.1+1"
$env:ANDROID_HOME = "C:\Users\ASUS\AppData\Local\Android\Sdk"
Write-Host "Using JDK 17: $env:JAVA_HOME"
Write-Host "Using Android SDK: $env:ANDROID_HOME"
.\gradlew.bat assembleDebug --no-daemon

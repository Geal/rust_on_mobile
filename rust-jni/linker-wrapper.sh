#!/bin/bash

args=("$@")
myargs=()
nextarg=-1

for ((i=0; i<$#; i++)) {
   if [ $nextarg == $i ]; then continue; fi
   case ${args[$i]} in
#       -Wl,--as-needed) nextarg=$((i+1)) ;;
#       -Wl,--allow-multiple-definition) nextarg=$((i+1)) ;;
#       -Wl,-Bstatic) nextarg=$((i+1)) ;;
       *) myargs+="${args[$i]} "
   esac
}

echo $myargs

/usr/local/Cellar/android-ndk/r11c/toolchains/arm-linux-androideabi-4.9/prebuilt/darwin-x86_64/bin/arm-linux-androideabi-gcc --sysroot=/usr/local/Cellar/android-ndk/r11c/platforms/android-16/arch-arm/ "$@"
#/usr/local/Cellar/android-ndk/r11c/toolchains/arm-linux-androideabi-4.9/prebuilt/darwin-x86_64/bin/arm-linux-androideabi-ld --sysroot=/usr/local/Cellar/android-ndk/r11c/platforms/android-16/arch-arm/ "$@"
#/usr/local/Cellar/android-ndk/r11c/toolchains/arm-linux-androideabi-4.9/prebuilt/darwin-x86_64/bin/arm-linux-androideabi-gcc --sysroot=/usr/local/Cellar/android-ndk/r11c/platforms/android-16/arch-arm/ $myargs

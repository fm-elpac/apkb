#!/system/bin/sh
# ${AZI_DIR_SDCARD_DATA}/apkb-setup/azi_init.sh

FROM=${AZI_DIR_SDCARD_DATA}/apkb-setup
TO=${AZI_DIR_APP_DATA}

# atool
cp ${FROM}/atool $TO

# static-web-server
cp ${FROM}/static-web-server-v2.38.0-aarch64-linux-android/static-web-server $TO

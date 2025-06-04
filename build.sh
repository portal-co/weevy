cd $(dirname $0)
sh ./weevy-camo-wasm/build.sh
sh ./weevy-common/build.sh
sh ./weevy-src-packager/build.sh
sh ./weevy-single-tenant/build.sh
sh ./weevy-rt/build.sh
sh ./weevy-crx-rt/build.sh
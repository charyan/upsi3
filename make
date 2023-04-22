echo $1

case $1 in

	"web")
		cargo build --target wasm32-unknown-unknown
		cp target/wasm32-unknown-unknown/debug/upsi3.wasm ./web/
	;;

	"web-release")
		cargo build --release --target wasm32-unknown-unknown
		cp target/wasm32-unknown-unknown/debug/upsi3.wasm ./web/
	;;

	"run")
		cargo build --target wasm32-unknown-unknown
		cp target/wasm32-unknown-unknown/debug/upsi3.wasm ./web/
		basic-http-server -a 0.0.0.0:4000 ./web/
	;;

	*)
		echo "nothing"

esac